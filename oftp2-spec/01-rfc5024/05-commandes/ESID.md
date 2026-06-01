---
groupe: commandes
rfc: "5024"
section: "5.3.11"
tags: [rfc5024, pdu, groupe/commandes]
---

# ESID — End Session

| Propriété | Valeur |
|-----------|--------|
| **Octet 0** | `'F'` |
| **Direction** | Les deux |
| **Tables §9** | Partout (souvent → **WF_NDISC**) |

## Structure

| Pos | Champ | Description |
|-----|-------|-------------|
| 0 | ESIDCMD | `'F'` |
| 1 | ESIDREAS | Code 00–99 |
| 3 | ESIDREASL | Longueur texte |
| 6 | ESIDREAST | UTF-8 optionnel |
| — | ESIDCR | CR |

## Codes raison principaux

| Code | Signification |
|------|----------------|
| `00` | Fin normale |
| `01` | Commande inconnue (1er octet) |
| `02` | Violation protocole (PDU hors état) |
| `03` | SSIDCODE inconnu |
| `04` | Mot de passe invalide |
| `05` | Urgence site local |
| `06` | Données invalides dans commande |
| `07` | Taille buffer incorrecte |
| `10`–`12` | Négociation SSID / mode |
| `11` | Échec authentification |

Utilisé aussi en interne par le moteur (ex. ESID(02) sur SFNA inattendu).

## Client / serveur

Identique : celui qui veut fermer envoie ESID ; l’autre passe en `F_ABORT_IND` ou `F_RELEASE_IND` selon code.

Voir [[../09-tables/9.9-error-abort]] · [[02-cas-speciaux/abort-et-erreurs]].
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Structure | `ESIDREAS` + CR | + **`ESIDREASL` + `ESIDREAST`** (UTF-8) |
| Codes `00`–`10`, `99` | ✅ | ✅ (même sens) |
| Code **`11`** | ❌ | Invalid challenge response (auth) |
| Code **`12`** | ❌ | Secure authentication requirements incompatible |

**Rust** : mapper `11`/`12` seulement si session auth CMS active.
