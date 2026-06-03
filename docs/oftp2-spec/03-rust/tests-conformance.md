---
groupe: rust
tags: [rust, rfc5024, groupe/rust]
---

# Tests de conformance

## Niveaux

| Niveau | Cible |
|--------|--------|
| Unit | `codec` round-trip chaque PDU |
| FSM | Chaque transition 9.8–9.12 (état + event → état) |
| Integration | Scénario §9.13 + peer réel |
| Interop | Partenaire EDI / simulateur commercial |

## Fixtures

- Captures hex de votre PoC actuelle → tests de régression
- Golden files depuis [[RFCs/rfc5024.txt]] §9.13

## Checklist par phase

Reprennent les cases de [[00-index/Parcours implémentation Rust]].

---

## Implémentation

- [ ] `insta` ou snapshots pour PDU encodés
- [ ] Mock `NetworkService` pour FSM sans socket
- [ ] CI : `cargo test --all-features`
