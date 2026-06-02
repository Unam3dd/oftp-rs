---
groupe: chapitre
rfc: "5024"
section: "2"
tags: [rfc5024, rust, groupe/chapitre]
---

# §2 Network Service

> `RFCs/rfc5024.txt` — §2

## Primitives réseau (`N_*`)

| Primitive | Direction | Rôle |
|-----------|-----------|------|
| `N_CON_RQ` / `N_CON_RS` | Local → réseau | Demande / acceptation connexion |
| `N_CON_IND` / `N_CON_CF` | Réseau → local | Connexion entrante / confirmée |
| `N_DATA_RQ` / `N_DATA_IND` | Les deux | Données (buffers ODETTE) |
| `N_DISC_RQ` / `N_DISC_IND` | Les deux | Déconnexion |
| `N_RST_IND` | Réseau → local | Reset |

## TLS — Secure ODETTE-FTP (§2.3)

- Session TLS **avant** tout octet ODETTE-FTP.
- Authentification mutuelle possible via certificats (lié à §4.2.3).

## Port (§2.4)

| Service | Port | Nom IANA |
|---------|------|----------|
| OFTP2 over TLS | **6619** | `odette-ftps` |

(OFTP1 classique : 3305 — ne pas confondre.)

## Liens

- [[01-rfc5024/09-tables-etats#9.8 Session Connection|Table 9.8]] utilise `N_CON_*`, `N_DISC_*`
- [[03-rust/structure-crate#transport]]

---

## Implémentation

- [ ] `async` TCP (tokio ou équivalent)
- [ ] TLS : rustls / native-tls, SNI, chaîne de confiance
- [ ] Adapter `N_*` vers une trait `NetworkService`
- [ ] Tests : connexion refusée, certificat invalide
