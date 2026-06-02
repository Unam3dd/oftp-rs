---
groupe: commandes
rfc: "5024"
section: "5.3.4"
tags: [rfc5024, pdu, groupe/commandes]
---

# SFPA — Start File Positive Answer

| Propriété | Valeur |
|-----------|--------|
| **Octet 0** | `'2'` |
| **Direction** | **Listener → Speaker** |
| **Table §9** | Speaker **K** ; Listener **H** (si app accepte) |

## Structure

| Pos | Champ | Description |
|-----|-------|-------------|
| 0 | SFPACMD | `'2'` |
| 1 | SFPAACNT | Position restart acquiescée (≤ SFIDREST), 0 si pas de restart |

## Transition Speaker **K**

- Si `SFPA.pos > SFID.pos` → ESID(02) abort.
- Sinon → `F_START_FILE_CF(+)`, `Credit_S = Window`, état **OPO**.

## Liens

[[SFID]] · [[../09-tables/9.10-speaker-1#Transitions]]
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Octet commande | `'2'` | `'2'` — identique |
| `SFPAACNT` | **Numeric(9)** | **Numeric(17)** — aligné sur `SFIDREST` v2 |

Sémantique identique : position restart acquiescée ≤ position demandée dans SFID.
