---
groupe: commandes
rfc: "5024"
section: "5.3.10"
tags: [rfc5024, pdu, groupe/commandes]
---

# EFNA — End File Negative Answer

| Propriété | Valeur |
|-----------|--------|
| **Octet 0** | `'5'` |
| **Direction** | **Listener → Speaker** |
| **Table §9** | 9.11 **D** → IDLESP |

## Structure

| Pos | Champ | Description |
|-----|-------|-------------|
| 0 | EFNACMD | `'5'` |
| 1 | EFNAREAS | Code raison (01–23, 99) |
| 3 | EFNAREASL | Longueur texte |
| 6 | EFNAREAST | UTF-8 optionnel |

Codes alignés SFNA + `21–23` traitement fichier (signature, decrypt, decompress).

Effet : `F_CLOSE_FILE_CF(-)`.
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Structure | `EFNACMD` + `EFNAREAS` seulement | + **`EFNAREASL` + `EFNAREAST`** (UTF-8) |

**Codes raison EFNA** : v1 s’arrête à `13` et `99`. v2 ajoute **`14`–`23`** :

| Code v2 | Signification |
|---------|----------------|
| `14`–`20` | Comme SFNA (direction, crypto, compression, signature) |
| `21` | Invalid file signature |
| `22` | File decryption failure |
| `23` | File decompression failure |

En OFTP1, l’échec de traitement fichier en fin de transfert n’avait pas ces codes dédiés sur EFNA.
