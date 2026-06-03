---
groupe: cas-speciaux
tags: [rfc5024, état, groupe/cas-speciaux]
---

# Timers et inactivité

## RFC §4.7.2

- Actions **1** / **2** dans les tables §9 : stop / start **inactivity timer** à chaque transition significative.
- `TIME-OUT` = événement interne → traitement table 9.9 (souvent ESID + abort).

## Implémentation Rust

- [ ] `tokio::time::Interval` ou timer reset à chaque PDU émis/reçu
- [ ] Durée : configurable (souvent bilatérale, pas toujours fixée dans RFC — documenter choix)
- [ ] Test : peer silencieux → timeout → session fermée
