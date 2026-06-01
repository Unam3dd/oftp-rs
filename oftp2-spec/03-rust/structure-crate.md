---
groupe: rust
tags: [rust, rfc5024, groupe/rust]
---

# Structure crate Rust (suggestion)

Alignée sur [[00-index/Parcours implémentation Rust]].

```
oftp2/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── transport/      # TCP/TLS, N_* primitives
│   ├── codec/          # DEB, command encode/decode (§5)
│   ├── protocol/
│   │   ├── mod.rs
│   │   ├── state.rs    # ProtocolState enum (§9.3)
│   │   ├── vars.rs     # SessionVars (§9.6)
│   │   ├── config.rs   # Capabilities (§9.7)
│   │   ├── session.rs  # Table 9.8
│   │   ├── speaker.rs  # Tables 9.10, 9.11
│   │   ├── listener.rs # Table 9.12
│   │   └── error.rs    # Table 9.9
│   ├── service/        # F_* API vers application
│   └── vfs/            # Fichier virtuel (§1.5)
└── tests/
    ├── roundtrip_codec.rs
    └── scenario_rfc_9_13.rs
```

## Principes

1. **Une seule FSM** par session TCP ; rôle Speaker/Listener = variable d’état.
2. **`transition(event) -> Effects`** : liste PDU + primitives `F_*` + nouveau état.
3. **Codec sans état** ; FSM sans I/O (testable à 100 % en unitaire).
4. Boucle runtime : `read DEB` → `dispatch` → `write DEB`.

## Liens Obsidian

| Module | Notes |
|--------|-------|
| `transport` | [[01-rfc5024/02-reseau-tls]] |
| `codec` | [[01-rfc5024/05-commandes-index]] |
| `protocol` | [[01-rfc5024/09-tables-etats]] |
| `service` | [[01-rfc5024/03-service-etats]] |
| `vfs` | [[01-rfc5024/01-introduction]] |

---

## Implémentation

- [ ] Initialiser le workspace Cargo
- [ ] Feature flags : `tls`, `cms`, `restart`
