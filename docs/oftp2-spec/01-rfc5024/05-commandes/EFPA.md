---
groupe: commandes
rfc: "5024"
section: "5.3.9"
tags: [rfc5024, pdu, groupe/commandes]
---

# EFPA — End File Positive Answer

| Propriété | Valeur |
|-----------|--------|
| **Octet 0** | `'4'` |
| **Direction** | **Listener → Speaker** |
| **Table §9** | 9.11 **C** ; Listener **K** |

## Structure

| Pos | Champ | Description |
|-----|-------|-------------|
| 0 | EFPACMD | `'4'` |
| 1 | EFPACD | `Y` = demande **CD** / `N` = pas de changement de tour |

## Transitions Speaker **C**

| EFPACD | Effet |
|--------|--------|
| `Y` (P1) | `F_CLOSE_FILE_CF(+)`, envoi **CD**, → **IDLELI** |
| `N` | `F_CLOSE_FILE_CF(+)`, → **IDLESP** |

`Credit_S` remis à 0 à l’envoi EFID (Action 7, 9.11).
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Format | `'4'` + `EFPACD` (Y/N) | **Identique** |

Pas de changement de structure entre les deux RFC.
