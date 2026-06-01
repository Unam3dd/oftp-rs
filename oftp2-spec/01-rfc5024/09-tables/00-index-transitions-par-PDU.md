---
groupe: etats
tags: [rfc5024, état, pdu]
---

# Index — Transitions (A, B, C…) par PDU

## Où sont les tables complètes ?

Les lettres **H**, **D**, **K**… ne sont **pas** définies dans les fiches PDU : ce sont des **codes de ligne** dans les **tables de transitions** du §9 de la RFC 5024.

| Table | Fiche Obsidian | Contenu |
|-------|----------------|---------|
| **9.8** Session | [[9.8-session-connection]] | Matrice + transitions **A–L** (SSRM, SSID, auth) |
| **9.9** Erreurs | [[9.9-error-abort]] | Transitions **A–I** (timeout, abort) |
| **9.10** Speaker 1 | [[9.10-speaker-1]] | Matrice + transitions **A–Z1** (SFID, DATA, EERP…) |
| **9.11** Speaker 2 | [[9.11-speaker-2]] | Transitions **A–E** (EFID, EFPA, EFNA) |
| **9.12** Listener | [[9.12-listener]] | Transitions **A–M** (réception) |

Vue d’ensemble : [[00-vue-ensemble]] · Index §9 : [[../09-tables-etats]]

Source brute : `RFCs/rfc5024.txt` §9.8.2, 9.9.2, 9.10.2, 9.11.2, 9.12.2.

## Comment lire une transition

Exemple : **SFID** reçu côté Speaker en état `OPOP` → code **K** (table 9.10) :

1. Ouvrir [[9.10-speaker-1#Table des transitions]]
2. Ligne **K** : prédicat → actions → PDU sortantes → **état suivant** (`OPO` ou `WF_NDISC`…)

La fiche PDU ([[../05-commandes/SFID]]) indique seulement **quelles** tables et **quels** codes concernent cette commande.

---

## Par PDU — quelle table ? quels codes ?

### Session (table **9.8**)

| PDU      | Rôle             | Codes transition (extraits)                               | Fiche PDU                |
| -------- | ---------------- | --------------------------------------------------------- | ------------------------ |
| **SSRM** | Responder envoie | **B** (après `N_CON_IND`)                                 | [[../05-commandes/SSRM]] |
| **SSID** | Initiator envoie | **H** ; Responder **E**, **G** ; négociation **D**        | [[../05-commandes/SSID]] |
| **SECD** | Auth             | **D**, **G**, **K**, **J**                                | [[../05-commandes/SECD]] |
| **AUCH** | Auth             | **J**                                                     | [[../05-commandes/AUCH]] |
| **AURP** | Auth             | **I**, **K**                                              | [[../05-commandes/AURP]] |
| **ESID** | Fermeture        | **F**, **L** (session) ; aussi partout via 9.9/9.10 **C** | [[../05-commandes/ESID]] |

### Speaker — fichier & données (tables **9.10**, **9.11**)

| PDU / événement   | Table | Codes               | État(s) typique(s)        | Fiche                    |
| ----------------- | ----- | ------------------- | ------------------------- | ------------------------ |
| `F_START_FILE_RQ` | 9.10  | **B**               | `IDLESP` → `OPOP`         | [[../05-commandes/SFID]] |
| **SFID** (envoyé) | —     | (suite de B)        | `OPOP`                    | [[../05-commandes/SFID]] |
| **SFPA** reçu     | 9.10  | **K**               | `OPOP` → `OPO`            | [[../05-commandes/SFPA]] |
| **SFNA** reçu     | 9.10  | **L**               | → `IDLESP`                | [[../05-commandes/SFNA]] |
| `F_DATA_RQ`       | 9.10  | **M**               | `OPO` / `OPOWFC`          | [[../05-commandes/DATA]] |
| **CDT** reçu      | 9.10  | **O**               | `OPOWFC` → `OPO`          | [[../05-commandes/CDT]]  |
| `F_CLOSE_FILE_RQ` | 9.11  | **A**               | `OPO` → `CLOP` + **EFID** | [[../05-commandes/EFID]] |
| **EFPA** reçu     | 9.11  | **C**               | → `IDLELI` ou `IDLESP`    | [[../05-commandes/EFPA]] |
| **EFNA** reçu     | 9.11  | **D**               | → `IDLESP`                | [[../05-commandes/EFNA]] |
| `F_CD_RQ`         | 9.10  | **D**               | → `IDLELICD` + **CD**     | [[../05-commandes/CD]]   |
| **CD** reçu       | 9.10  | **H**, **I**, **J** | selon état                | [[../05-commandes/CD]]   |
| `F_EERP_RQ`       | 9.10  | **A**               | → `WF_RTR` + **EERP**     | [[../05-commandes/EERP]] |
| `F_NERP_RQ`       | 9.10  | **Y**, **Z1**       | → `WF_RTR` + **NERP**     | [[../05-commandes/NERP]] |
| **RTR** reçu      | 9.10  | **N**               | `WF_RTR` → `IDLESP`       | [[../05-commandes/RTR]]  |

### Listener (table **9.12**)

| PDU reçu | Code | Effet principal | Fiche |
|----------|------|-----------------|-------|
| **SFID** | **A** | `F_START_FILE_IND` → `OPIP` | [[../05-commandes/SFID]] |
| `F_START_FILE_RS` | **H** | **SFPA** ou **SFNA** → `OPI` / `IDLELI` | [[../05-commandes/SFPA]] |
| **DATA** | **I** | `F_DATA_IND` (+ **CDT** si P7) | [[../05-commandes/DATA]] |
| **EFID** | **J** | `F_CLOSE_FILE_IND` → `CLIP` | [[../05-commandes/EFID]] |
| `F_CLOSE_FILE_RS` | **K** | **EFPA** / **EFNA** | [[../05-commandes/EFPA]] |
| **EERP** | **E** | `F_EERP_IND` → `RTRP` | [[../05-commandes/EERP]] |
| **NERP** | **L** | `F_NERP_IND` → `RTRP` | [[../05-commandes/NERP]] |
| `F_RTR_RS` | **M** | **RTR** → `IDLELI` | [[../05-commandes/RTR]] |
| **CD** | **C** | `F_CD_IND` → `IDLESPCD` | [[../05-commandes/CD]] |

### Erreurs (table **9.9** — toute session)

| Événement | Codes | Fiche |
|-----------|-------|-------|
| `TIME-OUT`, `F_ABORT_RQ`, `N_RST_IND`… | **A–I** | [[9.9-error-abort]] |

---

## Matrices (événement × état → lettre)

Avant la table de transitions, chaque fiche §9 a une **matrice** :

- Ex. [[9.10-speaker-1#Matrice]] : en colonne `OPOP`, ligne `SFPA` → cellule **K** → aller lire la ligne **K** dans la table des transitions.

**C** dans une cellule = souvent erreur protocole (pas une ligne nommée C à chercher — voir note en bas de [[9.10-speaker-1]]).

---

## Parcours recommandé

1. Vous codez la réception de **SFPA** côté Speaker  
2. [[../05-commandes/SFPA]] → code **K**, table **9.10**  
3. [[9.10-speaker-1#Table des transitions]] ligne **K** : prédicats P2, actions 1,2,7,12, sortie `F_START_FILE_CF(+)`, état **OPO**

## Liens

- [[../05-commandes/_MOC Commandes PDU]]
- [[_MOC Tables états]]
- [[00-index/Rôles client serveur et tables]]
