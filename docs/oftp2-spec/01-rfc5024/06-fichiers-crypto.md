---
groupe: chapitre
rfc: "5024"
section: "6"
tags: [rfc5024, rust, groupe/chapitre]
---

# §6 File Services

> `RFCs/rfc5024.txt` — §6

## §6.2 Signature

- Fichier et/ou **EERP/NERP** signés (CMS).
- Champs SFID / EERP : indicateurs + enveloppe signature.

## §6.3 Chiffrement

- Fichier chiffré CMS ; suite négociée (SFID cipher).
- Erreurs SFNA : cipher non supporté, fichier chiffré/non chiffré refusé (codes 15–17).

## §6.4 Compression

- Compression fichier ; négociation session + SFID.
- SFNA code 18 si compression refusée.

## §6.5 Format V — longueurs d’enregistrement

Règles pour fichiers à enregistrements variables.

## Liens

- [[01-rfc5024/05-commandes/SFID]]
- [[01-rfc5024/05-commandes/EERP]]
- [[01-rfc5024/10-11-misc-securite]]
- Codes raison SFNA : [[02-cas-speciaux/codes-raison]]

---

## Implémentation

- [ ] Dépendance CMS (ex. `cryptographic-message-syntax` ou openssl)
- [ ] Pipeline : compress → encrypt → sign (ordre selon spec bilatérale)
- [ ] Vérification à la réception avant `F_START_FILE_CF(+)`
- [ ] Feature flags `sign`, `encrypt`, `compress`
