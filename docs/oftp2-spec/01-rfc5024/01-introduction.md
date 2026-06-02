---
groupe: chapitre
rfc: "5024"
section: "1"
tags: [rfc5024, service, groupe/chapitre]
---

# §1 Introduction

> `RFCs/rfc5024.txt` — lignes ~175–566

## Résumé

OFTP2 (ODETTE-FTP v2) est un protocole **pair-à-pair** d’échange de fichiers métier (EDI), pas un FTP interactif. La v2 ajoute **TLS**, **CMS** (chiffrement/signature/compression), **EERP/NERP** signés, et fonctionne sur Internet (TCP), X.25, ISDN.

## §1.3 Principes généraux

- Communication **half-duplex** en mode commande (un sens à la fois).
- **Speaker** = émetteur du fichier courant ; **Listener** = récepteur.
- Accusés de réception bout-en-bout (**EERP**) ou négatifs (**NERP**).
- Fichiers **virtuels** : métadonnées + contenu (§1.5).

## §1.5 Fichier virtuel

| Concept | Note |
|---------|------|
| Organisation | Enregistrements ou flux binaire |
| Identification | Nom, date/heure, origine, destination |
| Format | Fixe, variable, texte, binaire (V-format) |
| Restart | Reprise à un offset (§1.5.4) |

## §1.7 Sécurité (aperçu)

- Transport : TLS (§2.3)
- Fichier : CMS (§6)
- Session : authentification mutuelle certificats (§4.2.3, §10)

## Liens

- [[01-rfc5024/04-phases-protocole]]
- [[01-rfc5024/06-fichiers-crypto]]
- [[02-cas-speciaux/restart]]

---

## Implémentation

- [ ] Modèle `VirtualFile` dans le crate
- [ ] Mapping stockage local ↔ champs SFID/EERP
- [ ] Politique sécurité (TLS obligatoire, suites CMS)
