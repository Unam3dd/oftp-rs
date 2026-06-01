---
groupe: chapitre
rfc: "5024"
section: "3"
tags: [rfc5024, service, état, groupe/chapitre]
---

# §3 File Transfer Service

> `RFCs/rfc5024.txt` — §3 (service vu par le **User Monitor** = votre application)

## Modèle (§3.1)

```
Application  ←F_*_RQ/RS/IND/CF→  Entité ODETTE-FTP  ←N_*→  Réseau
```

L’implémentation des primitives `F_*` est **côté application** ; l’entité protocole les consomme et émet (§3 intro).

## Primitives principales

### Session (§3.2)

| Primitive | Sens |
|-----------|------|
| `F_CONNECT_RQ` | Demander une session |
| `F_CONNECT_IND` | Session entrante (paramètres pair) |
| `F_CONNECT_RS` | Accepter/refuser négociation |
| `F_CONNECT_CF` | Session établie |

Paramètres : adresses, **ID**, **password**, **mode** (Sender-only / Receiver-only / Both), **restart**, **authentication**.

**Négociation de mode** : voir table §3.2.1 — le mode peut être réduit (Both → Sender-only côté pair).

### Fichier (§3.3)

| Phase | Request | Indication | Response | Confirm |
|-------|---------|------------|----------|---------|
| Ouverture | `F_START_FILE_RQ` | `F_START_FILE_IND` | `F_START_FILE_RS(+/-)` | `F_START_FILE_CF(+/-)` |
| Données | `F_DATA_RQ` | `F_DATA_IND` | — | `F_DATA_CF` |
| Fermeture | `F_CLOSE_FILE_RQ` | `F_CLOSE_FILE_IND` | `F_CLOSE_FILE_RS(+/-)` | `F_CLOSE_FILE_CF(+/-)` |

### Tours (§3.3.4)

- **Premier Speaker** : décidé à la connexion (souvent l’initiateur si mode Both).
- **Tours suivants** : via `F_CD_RQ` / `F_CD_IND` et commande **CD** sur le fil.

### Accusés (§3.3.5–6)

| Primitive | Rôle |
|-----------|------|
| `F_EERP_RQ` / `F_EERP_IND` | Accusé positif bout-en-bout |
| `F_NERP_RQ` / `F_NERP_IND` | Accusé négatif |
| `F_RTR_RS` / `F_RTR_CF` | Prêt à recevoir après EERP/NERP |

### Fin de session (§3.4)

| Primitive | Cas |
|-----------|-----|
| `F_RELEASE_RQ` | Fin normale ou erreur |
| `F_RELEASE_IND` | Pair a fermé |
| `F_ABORT_RQ` / `F_ABORT_IND` | Abort immédiat |

## Automates service (§3.5)

États **métier** (diagrammes ASCII dans la RFC) :

### IDLE (0)

- `F_CONNECT_RQ` → attente `F_CONNECT_CF` → **IDLE SPEAKER (1)** ou **IDLE LISTENER (2)**

### IDLE SPEAKER (1)

Transitions notables :

| Depuis | Événement | Vers |
|--------|-----------|------|
| IDLE SPEAKER | `F_START_FILE_RQ` (si P2) | **OPENING** |
| IDLE SPEAKER | `F_CD_RQ` | attente EERP/NERP |
| OPENING | `f_start_file_cf(+)` | **DATA TRANSFER** |
| DATA TRANSFER | `F_DATA_RQ` / `f_data_cf` | **NEXT RECORD** ↔ boucle |
| DATA TRANSFER | `F_CLOSE_FILE_RQ` | **CLOSING** |
| CLOSING | `f_file_close_cf(+)` | IDLE SPEAKER ou LISTENER selon P1 |

**Prédicats service §3.5.2 :**

- **P1** : confirmation positive ET Speaker = YES → repasse listener
- **P2** : Mode = Both OU Sender-only → autorisé à envoyer un fichier

### IDLE LISTENER (2)

Miroir : réception `F_START_FILE_IND`, `F_DATA_IND`, `F_CLOSE_FILE_IND`, etc.

> Les noms d’états **protocole** (`IDLESP`, `OPOP`…) sont dans [[01-rfc5024/09-tables-etats]] — c’est la couche sous-jacente.

## Liens

- [[01-rfc5024/04-phases-protocole]]
- [[01-rfc5024/09-tables-etats]]
- [[02-cas-speciaux/tours-speaker-listener]]

---

## Implémentation

- [ ] Trait `OftpUser` ou channel mpsc pour `F_*`
- [ ] Enum `ServiceState` aligné sur §3.5 (optionnel, peut dériver du §9)
- [ ] Exposer mode négocié après `F_CONNECT_CF`
- [ ] Tests : refus `F_START_FILE_RQ` en Receiver-only
