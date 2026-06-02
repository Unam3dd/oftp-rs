---
groupe: commandes
rfc: "5024"
section: "5.3.14"
tags: [rfc5024, pdu, groupe/commandes]
---

# NERP — Negative End Response

| Propriété | Valeur |
|-----------|--------|
| **Octet 0** | `'N'` |
| **Direction** | **Speaker → Listener** |
| **Tables §9** | Speaker **Y**, **Z1** ; Listener **L** |

## Structure (résumé)

Champs analogues **EERP** + :

| Champ | Description |
|-------|-------------|
| NERPREAS | Code raison 2 chiffres |
| NERPREASL / NERPREAST | Texte UTF-8 |
| NERPHSH / NERPSIG | Hash et signature optionnels |

Codes : [[02-cas-speciaux/codes-raison]] (03–36, 99).

## Flux

Comme EERP mais `F_NERP_IND` côté listener ; transitions **Y** avec variantes P4/P5 (special logic / signé).

État stocké : **NRSTWFCD** si avant CD.
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Commande | **N’existe pas** | **`N`** — Negative End Response (§5.3.14) |

**Rôle v2** : accusé **négatif** bout-en-bout (échec traitement chez le destinataire final), symétrique de l’EERP.

Champs spécifiques v2 : `NERPCREA` (créateur du NERP), `NERPREAS` + texte UTF-8, hash/signature CMS comme EERP.

**Rust** : ne pas émettre/parser NERP en mode OFTP1 ; tables §9.10 transitions **Y** / **Z1**.
