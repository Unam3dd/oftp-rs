---
groupe: cas-speciaux
tags: [rfc5024, état, service, groupe/cas-speciaux]
---

# Tours Speaker / Listener

## Concepts

- **Speaker** : envoie SFID, DATA, EFID, EERP/NERP (côté actif).
- **Listener** : reçoit et répond SFPA, CDT, EFPA, RTR.
- Le rôle **change** avec la commande **CD** (Change Direction).

## Séquence typique (les deux envoient)

1. Partenaire A Speaker → envoie fichier 1
2. **CD** (souvent dans EFPA ou `F_CD_RQ`)
3. B devient Speaker → envoie fichier 2
4. Répéter jusqu’à `F_RELEASE_RQ` / **ESID**

## États protocole liés

| État                | Situation                                    |
| ------------------- | -------------------------------------------- |
| `IDLESP` / `IDLELI` | En attente d’action                          |
| `IDLESPCD`          | CD indiqué à l’app, pas encore CD sur le fil |
| `IDLELICD`          | `F_CD_RQ` reçu, prêt à devenir speaker       |
| `WF_CD`             | Listener a demandé le tour via EFPA          |

## États « requête stockée »

Si l’app envoie `F_START_FILE_RQ` ou `F_EERP_RQ` **avant** que le CD arrive :

- `SFSTWFCD`, `ERSTWFCD`, `NRSTWFCD`, `CDSTWFCD`

→ Variable `V.Req-buf` (§9.6).

## Liens

- [[01-rfc5024/03-service-etats#Tours]]
- [[01-rfc5024/05-commandes/CD]]
- [[01-rfc5024/09-tables-etats]]

---

## Implémentation

- [ ] Ne pas accepter `F_START_FILE_RQ` si `V.Mode = Receiver-only`
- [ ] Queue `PendingRequest` quand en `WF_CD`
- [ ] Test : A envoie → CD → B envoie
