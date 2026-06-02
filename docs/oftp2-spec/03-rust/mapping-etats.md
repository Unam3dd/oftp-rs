---
groupe: rust
tags: [rust, état, groupe/rust]
---

# Mapping états §9 → Rust

## Enum suggérée

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolState {
    Idle,
    InitWaitNc,
    InitWaitRm,
    InitWaitSsid,
    RespNcOnly,
    RespWaitConRs,
    IdleSp,
    IdleSpCd,
    IdleLi,
    IdleLiCd,
    OpOutPending,      // OPOP
    OpOut,             // OPO
    OpOutWaitCredit,   // OPOWFC
    OpInPending,       // OPIP
    OpIn,              // OPI
    CloseInPending,    // CLIP
    CloseOutPending,   // CLOP
    WaitCd,
    WaitRtr,
    WaitNDisc,
    WaitSecd,
    WaitAuch,
    WaitAurp,
    RtrPending,
    // États buffer requête
    CdStoredWaitCd,
    SfStoredWaitCd,
    EerpStoredWaitCd,
    NerpStoredWaitCd,
}
```

## SessionVars (§9.6)

```rust
pub struct SessionVars {
    pub buf_size: u32,
    pub mode: Mode,
    pub window: u16,
    pub credit_s: u16,
    pub credit_l: u16,
    pub restart: bool,
    pub restart_pos: u64,
    pub caller: bool,
    pub authentication: bool,
    pub compression: bool,
    pub pending: Option<PendingPrimitive>,
    // id, password, challenge...
}
```

## Dispatch

```rust
pub enum Input {
    User(UserPrimitive),
    Network(NetworkPrimitive),
    Peer(Command),
    Timeout,
}

pub struct Effects {
    pub next: ProtocolState,
    pub to_user: Vec<UserPrimitive>,
    pub to_network: Vec<NetworkPrimitive>,
    pub to_peer: Vec<Command>,
}
```

## Table-driven (option avancée)

Pour coller à la RFC, chaque ligne 9.10.2 peut devenir :

```rust
{ state: IDLESP, event: F_START_FILE_RQ, pred: not_p1, actions: [1,2,5], out: [Sfid], next: OPOP }
```

→ génération ou macro depuis CSV futur.

## Liens

- [[01-rfc5024/09-tables-etats]]
- [[03-rust/structure-crate]]
- [[03-rust/tests-conformance]]

---

## Implémentation

- [ ] Commencer par 9.8 + chemin happy SFID→DATA→EFID
- [ ] Propriété tests : pas de transition depuis `OpOutWaitCredit` avec EFID
