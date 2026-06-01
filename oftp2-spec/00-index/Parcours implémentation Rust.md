---
groupe: index
tags: [groupe/index]
---

# Parcours implémentation Rust

Ordre recommandé pour passer d’une PoC « envoi d’octets » à un stack OFTP2 conforme.

## Phase 0 — Fondations

- [ ] TCP + TLS (§2, port **6619** `odette-ftps`)
- [ ] Parser / builder **un octet = command id** (§5.2)
- [ ] [[01-rfc5024/07-08-buffers|DEB]] : une commande par exchange buffer, pas de mélange DATA/commande

## Phase 1 — Session

- [ ] Table **[[01-rfc5024/09-tables-etats#9.8 Session Connection|9.8]]** : `IDLE` → `I_WF_NC` → … → `IDLESP` / `IDLELI`
- [ ] Séquence **SSRM** (Responder d’abord) puis **SSID** ×2 (§4.2)
- [ ] Primitives `F_CONNECT_RQ/IND/RS/CF` (§3.2)
- [ ] Optionnel : auth **SECD / AUCH / AURP** (§4.2.3)

**Test** : handshake seul, pas encore de fichier.

## Phase 2 — Un fichier en envoi (Speaker)

- [ ] Table **9.10 + 9.11** (Speaker)
- [ ] `F_START_FILE_RQ` → **SFID** → **SFPA** → **DATA** + **CDT** (crédit fenêtre) → **EFID** → **EFPA**
- [ ] États `OPOP` → `OPO` → `OPOWFC` → `CLOP`

**Test** : un fichier vers un partenaire de référence ou mock listener.

## Phase 3 — Réception (Listener)

- [ ] Table **9.12** (Listener)
- [ ] `F_START_FILE_IND/RS`, `F_DATA_IND`, `F_CLOSE_FILE_IND/RS`
- [ ] États `OPIP` → `OPI` → `CLIP`

## Phase 4 — Tours et accusés

- [ ] **CD** — changement Speaker/Listener (§3.3.4, [[02-cas-speciaux/tours-speaker-listener]])
- [ ] **EERP / NERP / RTR** (§3.3.5–6, [[02-cas-speciaux/eerp-nerp]])
- [ ] États `WF_CD`, `WF_RTR`, `IDLESPCD`, `IDLELICD`

## Phase 5 — Robustesse

- [ ] Table **9.9** erreur / abort ([[02-cas-speciaux/abort-et-erreurs]])
- [ ] Timers inactivité (§4.7.2, [[02-cas-speciaux/timers]])
- [ ] **ESID** fermeture normale et anormale (§3.4)

## Phase 6 — OFTP2 complet

- [ ] [[01-rfc5024/06-fichiers-crypto|Fichiers signés/chiffrés/compressés]]
- [ ] Restart (§4.3.3, [[02-cas-speciaux/restart]])
- [ ] Conformance / interop

## Mapping crate (suggestion)

Voir [[03-rust/structure-crate]] et [[03-rust/mapping-etats]].

| Module | Responsabilité |
|--------|----------------|
| `transport` | TCP/TLS, `N_*` |
| `codec` | PDU §5 |
| `protocol` | FSM §9 |
| `service` | Primitives §3 vers l’app |
| `vfs` | Fichier virtuel §1.5 |

## Suivi dans Obsidian

Filtrer ou rechercher `#à-faire` dans les notes **Implémentation**. Cocher au fur et à mesure.
