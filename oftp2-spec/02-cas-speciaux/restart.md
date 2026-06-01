---
groupe: cas-speciaux
tags: [rfc5024, service, groupe/cas-speciaux]
---

# Restart (reprise de transfert)

## RFC

- §1.5.4 — concept
- §4.3.3 — protocole
- Négociation session : `V.Restart` Y/N (SSID)
- SFID : **restart position** ; SFPA acquitte la position

## Erreurs

| Cas | Prédicat / code |
|-----|-----------------|
| Restart demandé, session sans restart | Speaker P1 (9.10) |
| SFPA position > SFID position | Speaker P2 (erreur protocole) |
| SFNA | codes 10–11 (record/byte count) |

## Liens

- [[01-rfc5024/05-commandes/SFID]]
- [[01-rfc5024/09-tables-etats#9.10 Speaker 1]]

---

## Implémentation

- [ ] Persister offset + hash partiel côté émetteur
- [ ] Reprendre lecture fichier à `Restart-pos`
- [ ] Test : couper TCP mi-fichier, reconnecter, restart
