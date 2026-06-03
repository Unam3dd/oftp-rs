---
groupe: commandes
rfc: "5024"
section: "5.3.16"
tags: [rfc5024, pdu, groupe/commandes]
---

# SECD — Security Change Direction

| Octet 0 | `'J'` |
| Phase | Start Session (auth mutuelle) |
| Direction | Initiator commence, puis Responder |

## Séquence (§4.2.4)

1. Initiator → **SECD**
2. Responder → **AUCH** (défi CMS chiffré)
3. Initiator → **AURP**
4. Responder → **SECD** (tour inverse)
5. Initiator → **AUCH** / **AURP**

Tables : [[../09-tables/9.8-session-connection]] **D**, **G**, **K**, **J**.

Si auth non requise (SSIDAUTH=N des deux côtés) : pas de SECD, direct **IDLESP**/**IDLELI**.
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Commande | **Absente** | **`J`** — Security Change Direction |

Introduit avec l’auth mutuelle post-SSID (`SSIDAUTH=Y`). Démarre l’échange AUCH/AURP dans un sens puis l’autre.

**Rust** : module auth uniquement OFTP2 ; ignoré si négociation v1.
