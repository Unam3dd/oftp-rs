---
groupe: commandes
rfc: "5024"
section: "5.3.15"
tags: [rfc5024, pdu, groupe/commandes]
---

# RTR — Ready To Receive

| Propriété | Valeur |
|-----------|--------|
| **Octet 0** | `'P'` |
| **Direction** | **Listener → Speaker** |
| **Tables §9** | Speaker **N** ; Listener **M** |

## Structure

1 octet : `'P'`.

## Sémantique

Le Listener confirme avoir **traité** l’EERP/NERP. Le Speaker peut reprendre (fichier suivant ou release).

Transition Speaker **N** : `F_RTR_CF` → **IDLESP**.

Transition Listener **M** : après `F_RTR_RS` de l’app → envoi RTR → **IDLELI**.
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Format | `'P'` seul | **Identique** |

Flow control après EERP **ou NERP** en v2 — en v1 seulement après EERP (pas de NERP).
