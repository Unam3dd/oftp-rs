---
groupe: commandes
rfc: "5024"
section: "5.3.2"
tags: [rfc5024, pdu, groupe/commandes]
---

# SSID — Start Session

| Propriété     | Valeur                                                   |
| ------------- | -------------------------------------------------------- |
| **Octet 0**   | `'X'` (0x58)                                             |
| **Phase**     | Start Session                                            |
| **Direction** | Initiator ↔ Responder (les deux envoient un SSID)        |
| **Tables §9** | [[../09-tables/9.8-session-connection]] (H, D, E, F, G…) |

## Rôle client / serveur

| Rôle TCP                | Action                                                                    |
| ----------------------- | ------------------------------------------------------------------------- |
| **Client** (Initiator)  | Envoie SSID après SSRM (transition **H**) ; reçoit SSID Responder (**D**) |
| **Serveur** (Responder) | Reçoit SSID d’abord (**E** → `F_CONNECT_IND`) ; répond SSID (**G**)       |

Ce n’est **pas** une PDU Speaker/Listener.

## Structure des champs

| Pos | Champ | Format | Description |
|-----|-------|--------|-------------|
| 0 | SSIDCMD | F X(1) | `'X'` |
| 1 | SSIDLEV | F 9(1) | Niveau protocole : `5` = OFTP2 (v2.0) |
| 2 | SSIDCODE | V X(25) | Code identification **Initiator** (§5.4) |
| 27 | SSIDPSWD | V X(8) | Mot de passe bilatéral |
| 35 | SSIDSDEB | V 9(5) | Taille max DEB (128–99999) — **min** des deux au final |
| 40 | SSIDSR | F X(1) | `S` send-only / `R` receive-only / `B` both |
| 41 | SSIDCMPR | F X(1) | Compression buffer Y/N |
| 42 | SSIDREST | F X(1) | Restart Y/N |
| 43 | SSIDSPEC | F X(1) | Special logic Y/N (X.25 async, pas TCP) |
| 44 | SSIDCRED | V 9(3) | Fenêtre crédit DATA (max 999) — **min** négocié |
| 47 | SSIDAUTH | F X(1) | Auth mutuelle Y/N — **doit être identique** des deux côtés |
| 48 | SSIDRSV1 | F X(4) | Réservé (espaces) |
| 52 | SSIDUSER | V X(8) | Données utilisateur bilatérales |
| 60 | SSIDCR | F X(1) | CR 0x0D ou 0x8D |

## Négociation (implémentation)

- **Buf-size** : plus petit des deux SSIDSDEB.
- **Credit (window)** : plus petit SSIDCRED côté Responder sinon erreur protocole.
- **Mode** : voir [[../03-service-etats#Mode]] — pas deux `S` ou deux `R` adjacents.
- **Auth** : si `p != q` → abort (pas de négociation).

## Variables mises à jour (Action 5, table 9.8)

`V.Mode`, `V.Restart`, `V.Compression`, `V.Buf-size`, `V.Window`, `V.Authentication`

## Erreurs liées

ESID `03` user unknown, `04` invalid password, `10`/`12` négociation — [[02-cas-speciaux/codes-raison]].

---

## OFTP1 vs OFTP2

| Champ / aspect | OFTP1 (RFC 2204) | OFTP2 (RFC 5024) |
|----------------|------------------|------------------|
| `SSIDLEV` | Seulement `'1'` | `'1'`,`'2'`,`'4'`,`'5'` (v2.0 = **`5`**) |
| `SSIDSPEC` | Fixe **`N`** (TCP) | **`Y`/`N`** (Y utile X.25 async, pas TCP) |
| `SSIDRSV1` | 5 octets réservés | 4 octets + champ **`SSIDAUTH`** |
| `SSIDAUTH` | ❌ absent | **`Y`/`N`** — auth mutuelle certificats (non négociable) |
| `SSIDCMPR` | Compression **buffer** (§6.2) | Compression **buffer** OFTP (≠ compression **fichier** CMS) |

**Négociation v2** : si `SSIDAUTH` différent entre les deux SSID → **abort session** (pas de compromis).

**Rust** : refuser champs v2 si `SSIDLEV` négocié = 1 ; activer SECD/AUCH/AURP seulement si auth = Y.

## Implémentation

- [ ] Parser / builder avec champs fixes + longueurs variables
- [ ] `negotiate_ssid(local, peer) -> Result<SessionVars>`
- [ ] Lier SSIDCODE au certificat TLS si auth
- [ ] Tests : deux `S` → rejet
