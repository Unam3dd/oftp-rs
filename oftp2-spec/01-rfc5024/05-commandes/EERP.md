---
groupe: commandes
rfc: "5024"
section: "5.3.13"
tags: [rfc5024, pdu, groupe/commandes]
---

# EERP — End to End Response

| Propriété | Valeur |
|-----------|--------|
| **Octet 0** | `'E'` |
| **Direction** | **Speaker → Listener** (accusé métier) |
| **Tables §9** | Speaker **A**, **R** ; Listener **E** |

## Structure (résumé)

| Champ | Description |
|-------|-------------|
| EERPDSN | Nom fichier (26) |
| EERPDATE / EERPTIME | Horodatage fichier |
| EERPUSER | User data |
| EERPDEST / EERPORIG | **Attention** : sens inversé vs SFID (dest=créateur, orig=destinataire final dans EERP) |
| EERPHSHL / EERPHSH | Hash fichier **transmis** |
| EERPSIGL / EERPSIG | Signature CMS si demandée (SFIDSIGN=Y) |

## Flux

1. App Speaker : `F_EERP_RQ`
2. Protocole : **EERP** → **WF_RTR**
3. App Listener : `F_EERP_IND` → traitement → `F_RTR_RS`
4. **RTR** → `F_RTR_CF` → Speaker **IDLESP**

Si CD pas encore reçu : état **ERSTWFCD** (stockage).

Voir [[02-cas-speciaux/eerp-nerp]] · [[../06-fichiers-crypto]].
## OFTP1 vs OFTP2

| Champ | OFTP1 | OFTP2 |
|-------|-------|-------|
| Identité fichier | DSN, date, time, user, dest, orig | Même logique |
| `EERPDATE` / `EERPTIME` | 6 + 6 car. (`YYMMDD`, `HHMMSS`) | **8 + 10** (`CCYYMMDD`, `HHMMSScccc`) |
| `EERPRSV1` | 9 octets | 3 octets |
| `EERPHSH` / `EERPHSHL` | ❌ | Hash fichier **transmis** (si EERP signé) |
| `EERPSIG` / `EERPSIGL` | ❌ | Signature **CMS** optionnelle |

**OFTP1** : accusé positif bout-en-bout **sans** hash ni signature sur le fil.  
**OFTP2** : si `SFIDSIGN=Y`, l’EERP doit être signé (champs hash + signature).

Pas de NERP en v1 : l’échec métier passait par SFNA/EFNA, pas par un accusé négatif E2E.
