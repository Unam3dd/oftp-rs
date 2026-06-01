---
groupe: commandes
rfc: "5024"
section: "5.3.17"
tags: [rfc5024, pdu, groupe/commandes]
---

# AUCH — Authentication Challenge

| Octet 0 | `'A'` |
| Direction | Celui qui **reçoit** SECD → envoie AUCH |

## Contenu

- Défi aléatoire chiffré avec certificat public du pair (CMS).
- Certificat lié au **SSIDCODE** du Initiator dans le SSID reçu.

Transition 9.8 **J** : → état **WF_AURP**.

Voir §4.2.3 · [[../10-11-misc-securite]].
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Commande | **Absente** | **`A`** — Authentication Challenge |

Contient défi aléatoire chiffré **CMS** avec le certificat du pair (lié au `SSIDCODE`).

**Rust** : dépendance CMS + magasin certificats.
