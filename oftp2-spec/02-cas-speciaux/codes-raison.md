---
groupe: cas-speciaux
tags: [rfc5024, pdu, groupe/cas-speciaux]
---

# Codes raison (SFNA, ESID, NERP)

Référence rapide — détail complet dans `RFCs/rfc5024.txt` §5.3.

## SFNA (refus fichier) — extraits

| Code | Signification |
|------|----------------|
| 01 | Invalid file name |
| 02 | Invalid destination |
| 05 | Max record length not supported |
| 06 | File size too big |
| 13 | Duplicate file |
| 14 | File direction refused |
| 15–20 | Crypto / compression / signature |

## NERP (accusé négatif) — extraits

| Code | Signification |
|------|----------------|
| 03–04 | User / password |
| 11–16 | SFNA-like sur livraison |
| 31–34 | Signature / décompression / déchiffrement |
| 35–36 | Non livré / non acquitté |
| 99 | Unspecified |

## Usage Rust

```rust
// Suggestion : enum ReasonCode(u8) + Display + From<SfnaPdu>
```

## Liens

- [[01-rfc5024/05-commandes/SFNA]]
- [[01-rfc5024/05-commandes/NERP]]
- [[02-cas-speciaux/eerp-nerp]]

---

## Implémentation

- [ ] Enum central `ReasonCode` partagé SFNA/ESID/NERP
- [ ] Exposer à l’app en `thiserror`
