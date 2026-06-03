# 🦀 Crusty OFTP

> **Crusty OFTP** est une collection d’outils autour du protocole **OFTP / OFTP2** (ODETTE File Transfer Protocol).
> L’objectif est de proposer une stack Rust moderne, testable et intégrée à l’écosystème **crusty-transfert** :
> bibliothèque protocole, **client**, **serveur**, et **CLI** pour gérer les configurations.

---

## 🎯 Vision du projet

| Composant | Description | Statut |
|-----------|-------------|--------|
| 📚 **`oftp-lib`** | Cœur : codec STB/OEB, PDU, FSM de session (RFC 5024 §9.8), négociation SSID | 🟢 En cours — base solide |
| 📡 **`oftp-client`** | Client OFTP2 (Tokio) — connexion TCP, handshake SSRM / SSID | 🟡 En cours — scénario connexion |
| 🖥️ **Serveur** | Responder : écoute TCP, transitions B / E / E+G, `cap_mode` | 🔴 À venir |
| ⚙️ **CLI configuration** | Profils (codes, modes, buffer, politique partenaire, compression…) | 🔴 À venir |

---

## 🏗️ Architecture du dépôt

```
crusty-oftp/
├── oftp-lib/       # Bibliothèque : codec + protocole + négociation
├── oftp-client/    # Binaire client (initiator)
└── (futur)         # serveur OFTP2, CLI de config
```

### Couches de `oftp-lib`

| Couche | Rôle |
|--------|------|
| **Codec** | Trames **STB** / **STH**, **OEB** (buffer d’échange), encodage/décodage des PDU |
| **Protocole** | `Session`, `SessionConfig`, machine à états table-driven (connexion §9.8) |
| **Négociation** | Fusion SSID local + pair → `SessionVars` (§5.3.2, action 5) |
| **I/O** | Lecture/écriture STB synchrone et **async** (feature `async`) |

---

## 📦 PDU — tableau de suivi

> Ce tableau est la **référence vivante** du dépôt : à mettre à jour à chaque PDU ajouté ou branché dans la FSM.

| Commande | PDU | Codec encode/decode | Dans `OftpExchangeBuffer` | FSM / usage session | Notes |
|:--------:|:-----|:-------------------:|:-------------------------:|:-------------------:|-------|
| `I` | **SSRM** — Start Session Ready Message | ✅ | ✅ | ✅ Transition **B** (responder) | Premier message serveur |
| `X` | **SSID** — Start Session Identity | ✅ | ✅ | ✅ **H**, **D**, **E**, **E+G** | Identité + capacités |
| `F` | **ESID** — End Session ID | ✅ | ✅ | ✅ Négociation KO, **release** | Raisons ESID typées (`EsidReason`) |
| `A` | **AUCH** — Authentication Challenge | ✅ | ✅ | ✅ Transitions **I**, **J** | Défi variable ; CMS à brancher |
| `S` | **AURP** — Authentication Response | ✅ | ✅ | ✅ Transition **K** | 20 octets ; vérif stub / CMS |
| `J` | **SECD** — Security Change Direction | ✅ | ✅ | ✅ Transitions **D**, **K** | 1 octet |
| `E` | **SFID** — Start File ID | 🔴 | 🔴 | 🔴 | Début transfert fichier |
| `H` | **EFID** — End File ID | 🔴 | 🔴 | 🔴 | Fin fichier + signature |
| `C` | **CD** — Change Direction | 🔴 | 🔴 | 🔴 | Inversion Speaker / Listener |
| … | **EERP** / **NERP**, **CDT**, etc. | 🔴 | 🔴 | 🔴 | Phases fichier & fin de session |

**Légende :** ✅ implémenté · 🟡 partiel · 🔴 non implémenté

---

## 🔐 Session & configuration

`SessionConfig` permet de piloter explicitement le **SSID émis** et les règles de session :

| Champ / builder | Rôle |
|-----------------|------|
| `with_code` / `with_password` | Identité locale (SSIDCODE / SSIDPSWD) |
| `with_mode` | Capacité émetteur/récepteur annoncée (`S` / `R` / `B`) |
| `with_cap_mode` | Contrainte site **P4** (responder) — distinct du `mode` SSID |
| `with_level`, `with_buffer_size`, `with_credit` | Niveau protocole, taille buffer, fenêtre |
| `with_compression`, `with_restart`, `with_special_logic` | Options négociées (fusion AND / min) |
| `with_authentication` | Indicateur SSIDAUTH (`Y` / `N`) |
| `with_partner_policy` | Validation du SSID **reçu** (code / mot de passe attendus) |

Exemple :

```rust
use oftp_lib::{SessionConfig, PartnerPolicy};
use oftp_lib::codec::pdu::ssid::{SsidMode, protocol_level::ProtocolLevel};

let config = SessionConfig::default()
    .with_code("MON_CLIENT")?
    .with_mode(SsidMode::SendOnly)
    .with_buffer_size(4096)?
    .with_level(ProtocolLevel::Rev20)
    .with_partner_policy(PartnerPolicy::RequireCode("MON_SERVEUR".into()));
```

---

## 🤝 Négociation SSID (état actuel)

Après échange des SSID, `negotiate_ssid` applique :

| Étape | Règle | Échec → ESID |
|-------|--------|----------------|
| 1 | **P4** `cap_mode` (responder uniquement) | Mode incompatible |
| 2 | **Partner policy** (code / mot de passe du pair) | 03 / 04 |
| 3 | **Level** — niveaux identiques + supportés | Capacités incompatibles |
| 4 | **Auth** — SSIDAUTH identiques (Y/Y ou N/N) | Auth incompatible |
| 5 | **Special logic** — pas de `Y` sur TCP | Capacités incompatibles |
| 6 | **Mode** — pas S+S ni R+R ; complémentaires → `Both` | Mode incompatible |
| 7 | **Merge** — `min` buffer/credit ; **AND** compression/restart | — |

Résultat stocké dans `SessionVars` (`buf_size`, `window`, `mode`, `compression`, etc.).

---

## 🔄 Machine à états — connexion (§9.8)

### États modélisés

| État | Rôle |
|------|------|
| `Idle` | Pas de session |
| `InitWaitRm` / `InitWaitSsid` | Initiator : SSRM puis SSID pair |
| `RespNcOnly` / `RespWaitConRs` | Responder : après SSRM, attente / validation |
| `IdleSp` / `IdleLi` | Session établie (Speaker / Listener) |
| `WaitNDisc` | Fin de session (ESID), attente fermeture |

### Transitions implémentées (connexion)

| ID | Transition | Rôle | Statut |
|:--:|------------|------|--------|
| **B** | SSRM reçu → envoi SSRM + état responder | Responder | ✅ |
| **H** | SSRM → envoi SSID local | Initiator | ✅ |
| **D** | SSID pair → négociation → `IdleSp` | Initiator | ✅ |
| **E** | SSID pair → négociation → `RespWaitConRs` | Responder | ✅ |
| **E+G** | SSID pair + envoi SSID local → `IdleLi` | Responder (`auto_accept`) | ✅ |
| **F** | ESID reçu en handshake → `Idle` | Initiator | ✅ |
| **Release** | Demande fin session → ESID | Les deux | ✅ |
| **WF_SECD** / auth **Y** | Chaîne SECD → AUCH → AURP | — | 🔴 À faire |

---

## 🛠️ Prérequis & commandes

**Prérequis :** [Rust](https://www.rust-lang.org/) stable (édition 2021)

```bash
# Compiler tout le workspace
cargo build

# Tests de la bibliothèque
cargo test -p oftp-lib

# Lancer le client (adresse optionnelle)
cargo run -p oftp-client -- 127.0.0.1:3305
```

Le client se connecte en TCP, lit le **SSRM**, envoie le **SSID** via la FSM, reçoit le SSID serveur et poursuit la négociation (transition **D**).

---

## 🗺️ Feuille de route protocolaire

### Phase 1 — Connexion & session ✅ *en cours*

- [x] Codec STB / OEB, SSRM, SSID, ESID
- [x] FSM connexion (B, H, D, E, E+G, F, release)
- [x] Négociation SSID + `SessionConfig` / `SessionVars`
- [x] Client Tokio (handshake de base)
- [ ] Binaire **serveur** responder complet
- [ ] Tests d’interop avec un partenaire OFTP réel

### Phase 2 — Authentification sécurisée 🔴

- [ ] PDU SECD, AUCH, AURP
- [ ] Branches FSM si `SSIDAUTH = Y`
- [ ] Gestion des raisons ESID 11 / 12 côté applicatif

### Phase 3 — Transfert de fichiers 🔴

- [ ] SFID, EFID, CD (change direction)
- [ ] États fichier §9.8 (hors connexion seule)
- [ ] Fenêtre, crédits, restart, compression sur flux données
- [ ] EERP / NERP, fin de session complète

### Phase 4 — Outils & exploitation 🔴

- [ ] **CLI** : créer / éditer / valider des profils `SessionConfig`
- [ ] Logs structurés, traces PDU (mode debug)
- [ ] Documentation opérateur (codes ODETTE, mapping ESID)

---

## 📚 Références

- [ODETTE — OFTP2](https://www.odette.org/) (RFC 5024 et documentation associée)
- Projet parent : **crusty-transfert**

---

## 📄 Licence

*À préciser.*

---

<p align="center">
  <sub>🦀 Implémentation Rust — tableau PDU et roadmap mis à jour au fil des merges.</sub>
</p>
