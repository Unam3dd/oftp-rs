---
groupe: commandes
tags: [rfc5024, pdu, groupe/commandes]
---

# Identifiants de commande (1er octet)

Chaque **Exchange Buffer** commence par **un octet** ASCII identifiant la commande (§5.2).

| Octet | Cmd | Phase | Émetteur typique | Table §9 |
|-------|-----|-------|------------------|----------|
| `I` | [[SSRM]] | Session | **Responder** (serveur TCP) | 9.8 |
| `X` | [[SSID]] | Session | Initiator puis Responder | 9.8 |
| `J` | [[SECD]] | Session / auth | Initiator puis Responder | 9.8 |
| `A` | [[AUCH]] | Auth | Celui qui reçoit SECD | 9.8 |
| `S` | [[AURP]] | Auth | Celui qui reçoit AUCH | 9.8 |
| `H` | [[SFID]] | Start File | **Speaker** | 9.10 / 9.12 |
| `2` | [[SFPA]] | Start File | **Listener** | 9.10 / 9.12 |
| `3` | [[SFNA]] | Start File | **Listener** | 9.10 / 9.12 |
| `D` | [[DATA]] | Data | **Speaker** | 9.10 / 9.12 |
| `C` | [[CDT]] | Data | **Listener** | 9.10 / 9.12 |
| `T` | [[EFID]] | End File | **Speaker** | 9.11 / 9.12 |
| `4` | [[EFPA]] | End File | **Listener** | 9.11 / 9.12 |
| `5` | [[EFNA]] | End File | **Listener** | 9.11 / 9.12 |
| `R` | [[CD]] | Tour | **Speaker** (demande) | 9.10 / 9.12 |
| `E` | [[EERP]] | Accusé | **Speaker** (souvent) | 9.10 / 9.12 |
| `N` | [[NERP]] | Accusé | **Speaker** | 9.10 / 9.12 |
| `P` | [[RTR]] | Accusé | **Listener** | 9.10 / 9.12 |
| `F` | [[ESID]] | End Session | Les deux | 9.8–9.12, 9.9 |

## Rust

```rust
#[repr(u8)]
pub enum CommandId {
    Ssrm = b'I',
    Ssid = b'X',
    // ...
}
```

## Erreur

Octet inconnu → **ESID** reason `01` → table [[../09-tables/9.9-error-abort|9.9 Error]] (groupe orange, voir [[../../Cartes/Tables états.canvas|carte Tables]]).
