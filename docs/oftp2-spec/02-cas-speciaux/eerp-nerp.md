---
groupe: cas-speciaux
tags: [rfc5024, service, pdu, groupe/cas-speciaux]
---

# EERP et NERP (accusés bout-en-bout)

## Métier

Preuve que le fichier a été **traité** côté destinataire (pas seulement reçu sur le fil — c’est EFPA).

- **EERP** : succès
- **NERP** : échec (codes raison 03–36, 99…)

## Flux (Speaker a envoyé le fichier)

1. Application Speaker : `F_EERP_RQ` (ou `F_NERP_RQ`)
2. Protocole : **EERP** / **NERP** sur le fil → état `WF_RTR`
3. Listener : `F_EERP_IND` / `F_NERP_IND` → traitement → `F_RTR_RS`
4. **RTR** sur le fil → `F_RTR_CF` → Speaker `IDLESP`

## Signature (OFTP2)

Si demandé dans SFID (`signed-eerp`), Action 18 table 9.10 — CMS.

## Liens

- [[01-rfc5024/05-commandes/EERP]]
- [[01-rfc5024/05-commandes/NERP]]
- [[01-rfc5024/05-commandes/RTR]]
- [[02-cas-speciaux/codes-raison]]

---

## Implémentation

- [ ] API app : callback après réception fichier pour décider EERP vs NERP
- [ ] Ne pas envoyer EERP avant clôture fichier validée
- [ ] Test EERP signé si feature activée
