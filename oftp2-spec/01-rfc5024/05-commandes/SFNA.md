---
groupe: commandes
rfc: "5024"
section: "5.3.5"
tags: [rfc5024, pdu, groupe/commandes]
---

# SFNA — Start File Negative Answer

| Propriété     | Valeur                 |     |
| ------------- | ---------------------- | --- |
| **Octet 0**   | `'3'`                  |     |
| **Direction** | **Listener → Speaker** |     |
| **Table §9**  | Speaker **L** → IDLESP |     |

## Structure

| Pos | Champ | Description |
|-----|-------|-------------|
| 0 | SFNACMD | `'3'` |
| 1 | SFNAREAS | Code raison 2 chiffres |
| 3 | SFNARRTR | `Y`/`N` retry |
| 4 | SFNAREASL | Longueur texte |
| 7 | SFNAREAST | Texte UTF-8 optionnel |

## Codes raison (extraits)

`01` nom invalide · `06` fichier trop gros · `15–20` crypto/signature · liste complète [[02-cas-speciaux/codes-raison]].

## Effet

`F_START_FILE_CF(-)` → retour **IDLESP** (réessayer plus tard si RETRY=Y).
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Structure de base | `SFNACMD` + `SFNAREAS` + `SFNARRTR` | Identique |
| Texte libre | ❌ | **`SFNAREASL` + `SFNAREAST`** (UTF-8, max 999 oct.) |

**Codes raison ajoutés en v2** :

| Code | Signification |
|------|----------------|
| `14` | File direction refused |
| `15` | Cipher suite not supported |
| `16` | Encrypted file not allowed |
| `17` | Unencrypted file not allowed |
| `18` | Compression not allowed |
| `19` | Signed file not allowed |
| `20` | Unsigned file not allowed |

Codes `01`–`13`, `99` : communs aux deux versions.
