---
groupe: chapitre
rfc: "5024"
section: "10-11"
tags: [rfc5024, groupe/chapitre]
---

# §10–§11 Miscellaneous & Security

## §10.1–10.3

- Choix d’algorithmes (bilatéral).
- Algorithmes cryptographiques autorisés.
- Extensions de protocole (champs réservés).

## §10.4 Certificate Services

- Liaison identité SSID ↔ certificats pour AUCH/AURP.

## §11 Security Considerations

- Menaces, bonnes pratiques TLS/CMS.
- À lire avant déploiement production.

## Annexes (hors Internet pur)

| Annexe | Sujet | Priorité Rust TCP/TLS |
|--------|-------|------------------------|
| A | Exemple mapping fichier virtuel | Moyenne |
| B | Jeu de caractères ISO 646 | Haute (codec) |
| C | X.25 | Basse sauf besoin legacy |
| D | ISDN | Basse |

## Liens

- [[01-rfc5024/02-reseau-tls]]
- [[01-rfc5024/06-fichiers-crypto]]

---

## Implémentation

- [ ] Documenter suites TLS et CMS acceptées (config)
- [ ] Store certificats + mapping ODETTE ID
- [ ] Ignorer annexes C/D sauf feature `x25`
