---
groupe: etats
hub: true
rfc: "5024"
section: "9.8.3-9.12.3"
tags: [rfc5024, état, groupe/etats]
---

# Actions et prédicats — tables §9 (RFC 5024)

> Source : `RFCs/rfc5024.txt` §9.8.3, 9.9.3, 9.10.3, 9.11.3, 9.12.3.

**Attention** : les numéros **Action N** sont **locaux à chaque table**.  
`Action 1` en **9.8** ≠ `Action 1` en **9.10**. Toujours lire la section **9.x.3** de la table en cours.

| Table | Transitions (9.x.2) | Prédicats + actions (9.x.3) |
|-------|-------------------|-----------------------------|
| [[9.8-session-connection]] | [[9.8-session-connection#§9.8.2 — Table des transitions]] | [[#Table 9.8 — Session Connection]] |
| [[9.9-error-abort]] | [[9.9-error-abort#§9.9.2 — Transitions]] | [[#Table 9.9 — Error and Abort]] |
| [[9.10-speaker-1]] | [[9.10-speaker-1#§9.10.2 — Table des transitions]] | [[#Table 9.10 — Speaker 1]] |
| [[9.11-speaker-2]] | [[9.11-speaker-2#§9.11.2 — Transitions]] | [[#Table 9.11 — Speaker 2]] |
| [[9.12-listener]] | [[9.12-listener#§9.12.2 — Table des transitions]] | [[#Table 9.12 — Listener]] |

---

## Table 9.8 — Session Connection

RFC §9.8.3 (l. ~4655–4717).

### Prédicats

| ID      | Définition RFC                                                                                                                                                                                                |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **P1**  | (Pas de ressources) **OU** `C.Cap-init = Responder` **OU** (`C.Cap-mode = Sender-only` ET `I.F_CONNECT_RQ.Mode = Receiver-only`) **OU** (`C.Cap-mode = Receiver-only` ET `I.F_CONNECT_RQ.Mode = Sender-only`) |
| **P2**  | Négociation **SSID** réussie (comparer buf-size, restart, compression, mode, special logic, window ; toute incompatibilité → échec)                                                                           |
| **P3**  | `C.Cap-init = Initiator`                                                                                                                                                                                      |
| **P4**  | Mode dans SSID incompatible avec `C.Cap-mode`                                                                                                                                                                 |
| **P5**  | `V.Caller = Yes`                                                                                                                                                                                              |
| **P6**  | `V.Caller = Yes` **ET** signature **AURP** vérifiable avec `V.Challenge`                                                                                                                                      |
| **P7**  | `V.Caller = No` **ET** signature **AURP** vérifiable avec `V.Challenge`                                                                                                                                       |
| **P8**  | `V.Authentication = I.SSID.Authentication`                                                                                                                                                                    |
| **P9**  | `I.F_CONNECT_RS.Authentication = Yes`                                                                                                                                                                         |
| **P10** | `O.F_CONNECT_IND.Authentication = I.F_CONNECT_RS.Authentication`                                                                                                                                              |
| **P11** | `V.Authentication = Yes`                                                                                                                                                                                      |

### Actions

| ID    | Définition RFC                                                                                                                                                                                                                             |
| ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **1** | `V.Mode` ← (`C.Cap-mode`, `I.F_CONNECT_RQ.Mode`) · `V.Pswd`, `V.Id`, `V.Restart`, `V.Authentication` ← `I.F_CONNECT_RQ` · `V.Buf-size = C.Max-buf-size` · `V.Compression = C.Cap-compression` · `V.Caller = Yes` · construire `O.N_CON_RQ` |
| **2** | Démarrer le **timer d’inactivité**                                                                                                                                                                                                         |
| **3** | Remplir `O.SSID` depuis les variables locales                                                                                                                                                                                              |
| **4** | **Arrêter** le timer                                                                                                                                                                                                                       |
| **5** | `V.Mode`, `V.Restart`, `V.Compression`, `V.Buf-size`, `V.Window`, `V.Authentication` ← **SSID** reçu                                                                                                                                       |
| **6** | `V.Challenge` ← nombre aléatoire unique à la session                                                                                                                                                                                       |

---

## Table 9.9 — Error and Abort

RFC §9.9.3 (l. ~4793–4797). Pas de prédicats nommés.

### Actions

| ID | Définition RFC |
|----|----------------|
| **1** | Arrêter le timer d’inactivité |
| **2** | Démarrer le timer d’inactivité |

*(Les transitions 9.9.2 référencent surtout l’action **1** ; **I** utilise **1,2** + ESID reason 01.)*

---

## Table 9.10 — Speaker 1

RFC §9.10.3 (l. ~4960–5032).

### Prédicats

| ID | Définition RFC |
|----|----------------|
| **P1** | (`I.F_START_FILE_RQ.Restart-pos > 0` ET `V.Restart = No`) **OU** (`V.Mode = Receiver-only`) — restart demandé non supporté |
| **P2** | `I.SFPA.Restart-pos > V.Restart-pos` — erreur protocole (acquittement restart > demande SFID) |
| **P3** | `V.Credit_S - 1 = 0` — crédit Speaker épuisé → **OPOWFC** |
| **P4** | Pas de **special logic** |
| **P5** | EERP/NERP **signé** demandé (`SFIDSIGN=Y`, etc.) |

### Actions

| ID | Définition RFC |
|----|----------------|
| **1** | Arrêter le timer d’inactivité |
| **2** | Démarrer le timer d’inactivité |
| **3** | Construire **EERP** depuis `F_EERP_RQ` |
| **4** | Stocker `F_EERP_RQ` dans `V.Req-buf` |
| **5** | Construire **SFID** depuis `F_START_FILE_RQ` · `V.Restart-pos = I.F_START_FILE_RQ.Restart-pos` |
| **6** | Stocker `F_START_FILE_RQ` dans `V.Req-buf` |
| **7** | Construire `F_START_FILE_CF(+)` depuis `I.SFPA` |
| **8** | Construire `F_START_FILE_CF(-)` depuis `I.SFNA` |
| **9** | Construire **EERP** depuis `F_EERP_RQ` stocké dans `V.Req-buf` |
| **10** | Construire **SFID** depuis `F_START_FILE_RQ` dans `V.Req-buf` · mettre à jour `V.Restart-pos` |
| **11** | Construire l’**Exchange Buffer** (PDU sur le fil) |
| **12** | `V.Credit_S = V.Window` |
| **13** | `V.Credit_S = V.Credit_S - 1` |
| **14** | Activer calcul **CRC** · envelopper le buffer en **special logic** |
| **15** | Construire **NERP** depuis `F_NERP_RQ` |
| **16** | Stocker `F_NERP_RQ` dans `V.Req-buf` |
| **17** | Construire **NERP** depuis `F_NERP_RQ` dans `V.Req-buf` |
| **18** | **Signer** le contenu NERP/EERP (CMS) |

**Note 1 (RFC)** : certaines cellules marquées **UE** (User Error) = l’événement ne devrait pas arriver dans cet état ; politique locale possible.

---

## Table 9.11 — Speaker 2

RFC §9.11.3 (l. ~5085–5121).

### Prédicats

| ID | Définition RFC |
|----|----------------|
| **P1** | `I.EFPA.CD-Request = Yes` |
| **P2** | Pas de **special logic** |

### Actions

| ID | Définition RFC |
|----|----------------|
| **1** | Arrêter le timer d’inactivité |
| **2** | Démarrer le timer d’inactivité |
| **3** | `O.F_CLOSE_FILE_CF(+).Speaker = No` |
| **4** | `O.F_CLOSE_FILE_CF(+).Speaker = Yes` |
| **5** | Construire **EFID** depuis `F_CLOSE_FILE_RQ` |
| **6** | Construire `F_CLOSE_FILE_CF(-)` depuis **EFNA** |
| **7** | `V.Credit_S = 0` |
| **8** | Envelopper l’exchange buffer en **special logic** |

**Note 1 (RFC)** : **interdit** d’envoyer **EFID** en état **OPOWFC** ; uniquement en **OPO**. L’implémentation doit éviter `F_CLOSE_FILE_RQ` / EFID tant que le crédit n’est pas régularisé (CDT reçu).

---

## Table 9.12 — Listener

RFC §9.12.3 (l. ~5279–5345).

### Prédicats

| ID | Définition RFC |
|----|----------------|
| **P1** | (`I.SFID.Restart-pos > 0` ET `V.Restart = No`) **OU** (`V.Mode = Sender-only`) — SFID invalide |
| **P2** | Réponse application **positive** |
| **P3** | `I.F_CLOSE_FILE_RS(+).Speaker = Yes` |
| **P4** | `I.F_START_FILE_RS(+).Restart-pos > V.Restart` |
| **P5** | **Special logic** utilisée |
| **P6** | `V.Credit_L - 1 < 0` — Speaker a dépassé le crédit (erreur protocole) |
| **P7** | `V.Credit_L - 1 = 0` — réinitialiser crédit Speaker (**CDT**) |
| **P8** | CRC reçu **invalide** (special logic) |

### Actions

| ID | Définition RFC |
|----|----------------|
| **1** | Arrêter le timer d’inactivité |
| **2** | Démarrer le timer d’inactivité |
| **3** | Construire `F_START_FILE_IND` depuis `I.SFID` · `V.Restart-pos = I.SFID.Restart-pos` |
| **4** | Construire `F_EERP_IND` depuis `I.EERP` |
| **5** | Ajouter l’en-tête **special logic** à la commande envoyée au Speaker |
| **6** | Retirer l’en-tête special logic du buffer **DATA** avant remontée à l’application |
| **7** | `V.Credit_L = V.Credit_L - 1` |
| **8** | `V.Credit_L = V.Window` |
| **10** | Construire `F_NERP_IND` depuis `I.NERP` |

> La RFC **ne définit pas d’Action 9** dans §9.12.3 (numérotation saute de 8 à 10).

**Note 1 (RFC)** — flow control réception : le Listener doit envoyer **CDT** sans retard quand le crédit Speaker est à zéro ; timing selon capacité app, buffers disponibles, et crédit pair.

---

## Index croisé action → effet (résumé implémentation)

| Action (tables qui l’ont) | Effet typique Rust |
|---------------------------|-------------------|
| 1 / 2 (toutes) | `timer.stop()` / `timer.start()` |
| 5 (9.8) | Appliquer négociation SSID → `Config` session |
| 12 (9.10) | `credit_s = window` après SFPA |
| 13 (9.10) | `credit_s -= 1` à chaque DATA |
| 7 (9.11) | `credit_s = 0` à l’EFID |
| 7 / 8 (9.12) | `credit_l -= 1` / `credit_l = window` (CDT) |
| 11 (9.10) | Encoder PDU + STB TCP |

---

## Implémentation

- [ ] Une fonction `apply_actions(table_id, &[u8])` par table §9
- [ ] Tests unitaires : transition **K** 9.10 → actions **1,2,7,12**
- [ ] Ne jamais mélanger les tables dans un même enum d’actions global sans préfixe
