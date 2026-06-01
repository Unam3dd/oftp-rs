---
groupe: commandes
rfc: "5024"
section: "5.3.1"
tags: [rfc5024, pdu, groupe/commandes]
---

# SSRM — Start Session Ready Message

| Propriété     | Valeur                                                         |
| ------------- | -------------------------------------------------------------- |
| **Octet 0**   | `'I'`                                                          |
| **Phase**     | Start Session — **premier** message ODETTE                     |
| **Direction** | **Responder → Initiator**                                      |
| **Table §9**  | **B** (serveur après `N_CON_IND`), **H** déclenche côté client |

## Rôle client / serveur

| Rôle            | Comportement                                                        |
| --------------- | ------------------------------------------------------------------- |
| **Serveur TCP** | Doit envoyer SSRM immédiatement après accept (transition 9.8 **B**) |
| **Client TCP**  | Ne l’envoie jamais ; attend en `I_WF_RM`                            |

## Structure (fixe, 19 octets)

| Pos | Champ | Valeur |
|-----|-------|--------|
| 0 | SSRMCMD | `'I'` |
| 1 | SSRMMSG | `'ODETTE FTP READY '` (17 car.) |
| 18 | SSRMCR | 0x0D ou 0x8D |

## Suite protocolaire

1. SSRM (Responder)
2. SSID (Initiator)
3. SSID (Responder)

Voir [[SSID]] · [[../09-tables/9.8-session-connection]].

---

## OFTP1 vs OFTP2

| Aspect | OFTP1 (RFC 2204) | OFTP2 (RFC 5024) |
|--------|------------------|------------------|
| Présence | ✅ §5.3.1 | ✅ §5.3.1 — **inchangé** |
| Format | `'I'` + `ODETTE FTP READY ` + CR | Identique |
| Implémentation | Même constante octets pour les deux versions | Idem |

Pas de différence de format sur le fil.

## Implémentation

- [ ] Constante `const SSRM: &[u8] = b"IODETTE FTP READY \r";`
- [ ] Test serveur : premier buffer émis = SSRM
