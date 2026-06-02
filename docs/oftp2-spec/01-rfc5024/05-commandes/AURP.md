---
groupe: commandes
rfc: "5024"
section: "5.3.18"
tags: [rfc5024, pdu, groupe/commandes]
---

# AURP — Authentication Response

| Octet 0 | `'S'` |
| Direction | Réponse au **AUCH** |

## Contenu

Défi déchiffré + signature prouvant possession de la clé privée.

## Vérification (table 9.8)

| Prédicat | Condition |
|----------|-----------|
| P6 | Initiator (`V.Caller`) et signature OK → `F_CONNECT_CF` / IDLESP |
| P7 | Responder et signature OK → SECD suivant |

Échec → ESID(11), abort.

**Note** : octet `'S'` = AURP ; ne pas confondre avec SSIDSR `'S'` (send-only) dans un autre contexte.
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Commande | **Absente** | **`S`** — Authentication Response |

Attention : octet `'S'` = AURP en auth, mais **`SSIDSR='S'`** = Send-only dans SSID (contexte différent).

Réponse au défi AUCH ; vérification signature → `F_CONNECT_CF` ou ESID(11).
