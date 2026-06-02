---
groupe: cas-speciaux
tags: [rfc5024, état, groupe/cas-speciaux]
---

# Abort, erreurs protocole et ESID

## Table 9.9

Gère depuis (presque) tous les états :

- `F_ABORT_RQ` utilisateur
- **ESID** reçu (reason code)
- `N_RST_IND`, `N_DISC_IND`
- `TIME-OUT` (inactivité)
- Erreur protocole (PDU inattendu) → transition **P** / **C** selon tables

## Effets typiques

- Émission **ESID** avec code raison
- `F_ABORT_IND` vers l’application
- `N_DISC_RQ` → `WF_NDISC` → `IDLE`

## ESID reason codes (voir §5.3.11)

Exemples : 00 normal, 02 erreur protocole, 03 user unknown, 04 bad password, 10/12 négociation SSID…

## Liens

- [[01-rfc5024/05-commandes/ESID]]
- [[01-rfc5024/09-tables-etats#9.9 Error and Abort]]
- [[02-cas-speciaux/timers]]

---

## Implémentation

- [ ] Type `OftpError` mappant reason ESID / SFNA / NERP
- [ ] `Drop` session garantit fermeture TCP
- [ ] Logs structurés : état + dernier PDU + reason
