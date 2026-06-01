---
groupe: chapitre
rfc: "5024"
section: "5"
tags: [rfc5024, pdu, groupe/chapitre]
---

# §5 Commands and Formats — Index PDU

> `RFCs/rfc5024.txt` — §5 (formats binaires détaillés)

## Conventions (§5.1)

- **1 commande = 1 Exchange Buffer** — jamais mélanger DATA et commande (§5.2).
- Premier octet = **identifiant de commande**.
- Caractères : sous-ensemble ISO 646 sauf champs marqués **UTF-8** (ex. texte NERP).

## Index rapide

[[05-commandes/00-identifiants-pdu|Table des octets de commande]] — 1er octet de chaque PDU.

[[05-commandes/00-OFTP1-vs-OFTP2|Comparatif OFTP1 (RFC 2204) vs OFTP2 (RFC 5024)]] — par commande.

## Fiches par commande (champs + tables §9 + client/serveur)

Chaque fiche décrit la structure, le sens **Speaker/Listener**, et les **codes de transition** associés.

| §5.3 | Cmd | Fiche | RFC ~ligne |
|------|-----|-------|------------|
| 5.3.1 | SSRM | [[01-rfc5024/05-commandes/SSRM]] | 2137 |
| 5.3.2 | SSID | [[01-rfc5024/05-commandes/SSID]] | 2191 |
| 5.3.3 | SFID | [[01-rfc5024/05-commandes/SFID]] | 2415 |
| 5.3.4 | SFPA | [[01-rfc5024/05-commandes/SFPA]] | 2751 |
| 5.3.5 | SFNA | [[01-rfc5024/05-commandes/SFNA]] | 2776 |
| 5.3.6 | DATA | [[01-rfc5024/05-commandes/DATA]] | 2873 |
| 5.3.7 | CDT | [[01-rfc5024/05-commandes/CDT]] | 2895 |
| 5.3.8 | EFID | [[01-rfc5024/05-commandes/EFID]] | 2923 |
| 5.3.9 | EFPA | [[01-rfc5024/05-commandes/EFPA]] | 2975 |
| 5.3.10 | EFNA | [[01-rfc5024/05-commandes/EFNA]] | 3000 |
| 5.3.11 | ESID | [[01-rfc5024/05-commandes/ESID]] | 3087 |
| 5.3.12 | CD | [[01-rfc5024/05-commandes/CD]] | 3203 |
| 5.3.13 | EERP | [[01-rfc5024/05-commandes/EERP]] | 3221 |
| 5.3.14 | NERP | [[01-rfc5024/05-commandes/NERP]] | 3389 |
| 5.3.15 | RTR | [[01-rfc5024/05-commandes/RTR]] | 3647 |
| 5.3.16 | SECD | [[01-rfc5024/05-commandes/SECD]] | 3664 |
| 5.3.17 | AUCH | [[01-rfc5024/05-commandes/AUCH]] | 3680 |
| 5.3.18 | AURP | [[01-rfc5024/05-commandes/AURP]] | 3725 |

## §5.4 Identification Code

Algorithme de calcul du code d’identification session — utile pour logs et corrélation.

## Usage avec le §9

Quand la table 9.10 dit « Action 5: Build SFID », ouvrir [[01-rfc5024/05-commandes/SFID]] + transition concernée dans [[01-rfc5024/09-tables-etats#9.10 Speaker State Table 1]].

---

## Implémentation

- [ ] Crate module `codec::command` — enum `Command` + `encode`/`decode`
- [ ] Tests round-trip par PDU avec vecteurs hex fixtures
- [ ] Validation longueurs / champs obligatoires avant envoi
