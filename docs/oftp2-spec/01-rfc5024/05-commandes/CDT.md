---
groupe: commandes
rfc: "5024"
section: "5.3.7"
tags: [rfc5024, pdu, groupe/commandes]
---

# CDT — Set Credit

| Propriété | Valeur |
|-----------|--------|
| **Octet 0** | `'C'` |
| **Direction** | **Listener → Speaker** |
| **Table §9** | Speaker **O** ; Listener **I** (si P7) |

## Structure

| Pos | Champ | Description |
|-----|-------|-------------|
| 0 | CDTCMD | `'C'` |
| 1 | CDTRSV1 | Réservé (2 espaces) |

## Effet

- Speaker : `Credit_S := Window` (Action 12), `F_DATA_CF`, retour **OPO** depuis **OPOWFC**.
- Pas de payload : simple signal « tu peux renvoyer jusqu’à Window buffers DATA ».

Voir Note 1 [[../09-tables/9.12-listener]] (envoyer quand crédit pair = 0).
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Format | `'C'` + 2 octets réservés | **Identique** |

Aucune différence de PDU entre RFC 2204 et RFC 5024.
