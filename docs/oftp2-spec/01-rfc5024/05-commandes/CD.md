---
groupe: commandes
rfc: "5024"
section: "5.3.12"
tags: [rfc5024, pdu, groupe/commandes]
---

# CD — Change Direction

| Propriété | Valeur |
|-----------|--------|
| **Octet 0** | `'R'` |
| **Phase** | Start File, End File, End Session |
| **Direction** | **Speaker → Listener** (demande de tour) |
| **Tables §9** | Speaker **D**, **H**, **I**, **J** ; Listener **C** |

## Structure

Message **1 octet** seul : `'R'`.

## Sémantique

- Le **Speaker** cède le tour : le pair devient Speaker.
- Souvent précédé de `F_CD_RQ` côté app ou demandé via EFPA (`CD-Request=Y`).

## Transitions

| État Speaker | Code | → |
|--------------|------|---|
| IDLESP | D | **IDLELICD** + envoi CD |
| WF_CD | H | **IDLESP** (CD reçu du pair) |
| OPOP | I/J | Relance SFID ou IDLELICD |

Listener recevant CD : **C** → `F_CD_IND` → **IDLESPCD**.

Voir [[02-cas-speciaux/tours-speaker-listener]].
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Format | 1 octet `'R'` | **Identique** |
| Phases d’usage | Start File, End File, End Session | Identique |

Aucune différence de PDU.
