---
groupe: commandes
rfc: "5024"
section: "5.3.8"
tags: [rfc5024, pdu, groupe/commandes]
---

# EFID — End File

| Propriété | Valeur |
|-----------|--------|
| **Octet 0** | `'T'` |
| **Direction** | **Speaker → Listener** |
| **Table §9** | 9.11 **A** (depuis **OPO** uniquement) ; Listener **J** |

## Structure

| Pos | Champ | Description |
|-----|-------|-------------|
| 0 | EFIDCMD | `'T'` |
| 1 | EFIDRCNT | Nombre d’enregistrements (F/V) ou 0 (U/T) |
| 18 | EFIDUCNT | Nombre d’octets (unités) transmis |

Compteurs = **totaux fichier**, même en restart.

## Règle

**Interdit** depuis `OPOWFC` — attendre CDT et retour **OPO** ([[../09-tables/9.11-speaker-2]] Note 1).

## Suite

Listener répond [[EFPA]] ou [[EFNA]].
## OFTP1 vs OFTP2

| Champ | OFTP1 | OFTP2 |
|-------|-------|-------|
| `EFIDRCNT` | Numeric(**9**) | Numeric(**17**) |
| `EFIDUCNT` | Numeric(**12**) | Numeric(**17**) |

Sémantique identique : compteurs **totaux** du fichier (même en restart).
