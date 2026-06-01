---
groupe: chapitre
rfc: "5024"
section: "9"
tags: [rfc5024, état, rust, groupe/chapitre]
---

# §9 Protocol State Machine — Index

> Source complète : `RFCs/rfc5024.txt` §9

## Documentation détaillée (tables complètes)

| Table | Fiche | Rôle |
|-------|-------|------|
| Vue d’ensemble | [[09-tables/00-vue-ensemble]] | Quelle table utiliser quand |
| **9.8** Session | [[09-tables/9.8-session-connection]] | TCP → SSRM/SSID → `IDLESP`/`IDLELI` |
| **9.9** Erreurs | [[09-tables/9.9-error-abort]] | Timeout, abort, buffer invalide |
| **9.10** Speaker 1 | [[09-tables/9.10-speaker-1]] | SFID, DATA, CD, EERP/NERP |
| **9.11** Speaker 2 | [[09-tables/9.11-speaker-2]] | EFID, EFPA/EFNA |
| **9.12** Listener | [[09-tables/9.12-listener]] | Réception + RTR |

**Client TCP vs serveur TCP** : [[00-index/Rôles client serveur et tables]]

Les matrices et transitions des fiches ci-dessus sont **transcrites** depuis la RFC 5024 (vérifiées ligne à ligne).

---

## 9.3 — États protocole (référence)

> **Entité** = rôle RFC de l’état (pas « client » au sens TCP seul).  
> **TCP typique** = qui a souvent cet état en pratique (`C` = client TCP, `S` = serveur TCP, `—` = les deux).  
> Voir [[00-index/Rôles client serveur et tables]] pour les trois axes.

| État         | Entité (rôle RFC)           | Initiator / Responder   | TCP typique | Table §9      | Signification                             |
| ------------ | --------------------------- | ----------------------- | ----------- | ------------- | ----------------------------------------- |
| `IDLE`       | **Session** (aucune)        | —                       | C ou S      | 9.8, 9.9      | Pas de session ODETTE                     |
| `I_WF_NC`    | **Initiator** session       | Initiator uniquement    | **C**       | 9.8           | Attend `N_CON_CF` (TCP ouvert)            |
| `I_WF_RM`    | **Initiator** session       | Initiator               | **C**       | 9.8           | Attend **SSRM** du Responder              |
| `I_WF_SSID`  | **Initiator** session       | Initiator               | **C**       | 9.8           | SSID envoyé, attend SSID pair             |
| `A_NC_ONLY`  | **Responder** session       | Responder uniquement    | **S**       | 9.8           | SSRM envoyé, attend **SSID**              |
| `A_WF_CONRS` | **Responder** session       | Responder               | **S**       | 9.8           | SSID reçu, attend `F_CONNECT_RS` app      |
| `WF_SECD`    | **Session** auth            | Les deux (tour auth)    | C ou S      | 9.8           | Auth : attente SECD / AURP                |
| `WF_AUCH`    | **Session** auth            | Celui qui a envoyé SECD | C ou S      | 9.8           | Attend **AUCH**                           |
| `WF_AURP`    | **Session** auth            | Celui qui a envoyé AUCH | C ou S      | 9.8           | Attend **AURP**                           |
| `IDLESP`     | **Speaker** (tour fichier)  | Les deux                | C ou S      | 9.10, 9.11    | Au repos, peut **envoyer** un fichier     |
| `IDLELI`     | **Listener** (tour fichier) | Les deux                | C ou S      | 9.12          | Au repos, peut **recevoir** un fichier    |
| `IDLESPCD`   | **Speaker** (tour cédé)     | Les deux                | C ou S      | 9.10          | `F_CD_IND` émis, pas encore CD sur le fil |
| `IDLELICD`   | **Listener** (après CD)     | Les deux                | C ou S      | 9.12          | **CD** reçu, prêt à devenir Speaker       |
| `OPOP`       | **Speaker** envoi           | Les deux                | C ou S      | 9.10          | **SFID** envoyé, attend SFPA/SFNA         |
| `OPO`        | **Speaker** envoi           | Les deux                | C ou S      | 9.10, 9.11    | Transfert sortant (DATA)                  |
| `OPOWFC`     | **Speaker** envoi           | Les deux                | C ou S      | 9.11          | Crédit épuisé, attend **CDT**             |
| `CLOP`       | **Speaker** envoi           | Les deux                | C ou S      | 9.11          | **EFID** envoyé, attend EFPA/EFNA         |
| `OPIP`       | **Listener** réception      | Les deux                | C ou S      | 9.12          | **SFID** reçu, attend `F_START_FILE_RS`   |
| `OPI`        | **Listener** réception      | Les deux                | C ou S      | 9.12          | Réception **DATA**                        |
| `CLIP`       | **Listener** réception      | Les deux                | C ou S      | 9.12          | **EFID** reçu, attend `F_CLOSE_FILE_RS`   |
| `WF_CD`      | **Listener** → tour         | Les deux                | C ou S      | 9.10, 9.12    | EFPA a demandé CD, attend **CD**          |
| `WF_RTR`     | **Speaker** accusé          | Les deux                | C ou S      | 9.10          | **EERP/NERP** envoyé, attend **RTR**      |
| `RTRP`       | **Listener** accusé         | Les deux                | C ou S      | 9.12          | EERP/NERP reçu, attend `F_RTR_RS`         |
| `SFSTWFCD`   | **Speaker** (buffer)        | Les deux                | C ou S      | 9.10          | `F_START_FILE_RQ` stocké avant CD         |
| `ERSTWFCD`   | **Speaker** (buffer)        | Les deux                | C ou S      | 9.10          | `F_EERP_RQ` stocké avant CD               |
| `NRSTWFCD`   | **Speaker** (buffer)        | Les deux                | C ou S      | 9.10          | `F_NERP_RQ` stocké avant CD               |
| `CDSTWFCD`   | **Speaker** (buffer)        | Les deux                | C ou S      | 9.10          | `F_CD_RQ` stocké avant CD                 |
| `WF_NDISC`   | **Session** fermeture       | Les deux                | C ou S      | 9.8–9.12, 9.9 | **ESID** envoyé, attend `N_DISC_IND`      |

### Légende entités

| Valeur « Entité » | Signification |
|-------------------|---------------|
| **Initiator** session | Seule l’entité qui a ouvert la connexion TCP peut être dans cet état |
| **Responder** session | Seule l’entité qui a accepté la connexion |
| **Speaker** | Rôle **émetteur** du fichier courant (SFID, DATA, EFID, EERP) |
| **Listener** | Rôle **récepteur** (SFPA, CDT, EFPA, RTR) |
| **Session** | Avant ou hors tour fichier (IDLE, auth, fermeture) |
| **Les deux** | Initiator **ou** Responder peut être Speaker/Listener selon le tour |

### Exemple lecture

- **`A_NC_ONLY`** : entité = **Responder** → en pratique le **serveur TCP** qui vient d’envoyer SSRM.  
- **`OPO`** : entité = **Speaker** → peut être le client TCP **ou** le serveur, selon qui envoie le fichier **maintenant**.

## 9.4 / 9.5 — Événements

**Entrants** : primitives `F_*`, `N_*`, PDU pair (SSID…RTR), `TIME-OUT`.

**Sortants** : mêmes familles vers app, réseau, pair.

Liste exhaustive : `RFCs/rfc5024.txt` l. 4387–4439.

## 9.6 / 9.7 — Variables et constantes

→ [[03-rust/mapping-etats]] pour mapping Rust.

## 9.8.3 – 9.12.3 — Actions et prédicats

→ [[09-tables/00-actions-et-predicats]] — texte RFC complet pour chaque **Action N** (numéros **par table**, ne pas mélanger).

## 9.13 — Exemple

Scénario OPOP + SFPA transition **K** : `RFCs/rfc5024.txt` l. 5347+ — à transformer en test golden [[03-rust/tests-conformance]].

---

## Implémentation

- [ ] Implémenter les 5 dispatchers de table
- [ ] Tests par code de transition (A, B, C…)
- [ ] Lier chaque transition aux PDU dans `05-commandes/`
