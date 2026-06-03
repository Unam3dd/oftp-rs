---
groupe: commandes
tags: [rfc5024, rfc2204, pdu]
---

# OFTP1 vs OFTP2 — Vue d’ensemble des commandes

> **OFTP1** : [RFC 2204](RFCs/rfc2204.txt) (1997, niveau protocole `1`)  
> **OFTP2** : [RFC 5024](RFCs/rfc5024.txt) (2007, obsoletes 2204, niveau `5`)

## Commandes : présence

| Octet | Commande | OFTP1 | OFTP2 | Fiche |
|-------|----------|-------|-------|-------|
| `I` | SSRM | ✅ identique | ✅ | [[SSRM]] |
| `X` | SSID | ✅ simplifié | ✅ + auth | [[SSID]] |
| `J` | SECD | ❌ | ✅ **nouveau** | [[SECD]] |
| `A` | AUCH | ❌ | ✅ **nouveau** | [[AUCH]] |
| `S` | AURP | ❌ | ✅ **nouveau** | [[AURP]] |
| `H` | SFID | ✅ | ✅ étendu | [[SFID]] |
| `2` | SFPA | ✅ | ✅ compteur élargi | [[SFPA]] |
| `3` | SFNA | ✅ | ✅ + codes crypto | [[SFNA]] |
| `D` | DATA | ✅ | ✅ payload binaire | [[DATA]] |
| `C` | CDT | ✅ identique | ✅ | [[CDT]] |
| `T` | EFID | ✅ | ✅ compteurs élargis | [[EFID]] |
| `4` | EFPA | ✅ identique | ✅ | [[EFPA]] |
| `5` | EFNA | ✅ | ✅ + texte UTF-8 | [[EFNA]] |
| `F` | ESID | ✅ | ✅ + auth / texte | [[ESID]] |
| `R` | CD | ✅ identique | ✅ | [[CD]] |
| `E` | EERP | ✅ simple | ✅ hash + signature | [[EERP]] |
| `N` | NERP | ❌ | ✅ **nouveau** | [[NERP]] |
| `P` | RTR | ✅ identique | ✅ | [[RTR]] |

## Différences transverses (hors PDU)

| Sujet | OFTP1 | OFTP2 |
|-------|-------|-------|
| Transport Internet | TCP port **3305** | TCP **3305** + **TLS 6619** (`odette-ftps`) |
| Sécurité fichier | Compression buffer (§6.2) | CMS : chiffrement, signature, compression fichier |
| Accusé négatif **bout-en-bout** | Pas de NERP (échec = SFNA/EFNA seulement) | **NERP** dédié |
| Auth session mutuelle | Mot de passe SSID seulement | **SECD/AUCH/AURP** + certificats |
| Dates fichier | `YYMMDD` / `HHMMSS` (6 ch.) | `CCYYMMDD` / `HHMMSScccc` (8 + 10) |
| Taille fichier max (SFID) | 7 chiffres (~9999999 Ko) | 13 chiffres (~ pétaoctets) |
| Restart position | 9 chiffres | 17 chiffres |
| Texte d’erreur humain | Non (sauf codes) | Champs **UTF-8** optionnels (SFNA, EFNA, ESID, NERP) |
| SSIDSPEC | Toujours `N` (TCP) | `Y`/`N` (Y = X.25 async seulement) |

## Impact Rust

- Parser **versionné** selon `SSIDLEV` négocié (`1` vs `5`).
- Ne pas envoyer de champs v2 si le pair est v1.
- **NERP / SECD / AUCH / AURP** : feature flag `oftp2` uniquement.

## Index des fiches détaillées

Chaque commande a une section **« OFTP1 vs OFTP2 »** dans sa note.
