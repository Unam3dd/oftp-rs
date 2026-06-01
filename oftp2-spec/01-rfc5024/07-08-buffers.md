---
groupe: chapitre
rfc: "5024"
section: "7-8"
tags: [rfc5024, pdu, groupe/chapitre]
---

# §7–§8 Buffers

## §7 ODETTE-FTP Data Exchange Buffer (DEB)

- Unité d’échange applicative : **une commande** ou **un bloc DATA** par buffer.
- Taille négociée dans SSID (`Buf-size`, max 99999 octets selon constantes).
- §7.3 : règles de remplissage (padding, taille réelle vs max).

## §8 Stream Transmission Buffer (STB)

- En-tête pour transport sur flux (ex. X.25) ; sur TCP pur, souvent couche simplifiée.
- §8.2 : format en-tête STB.

## Lien PoC actuelle

Si votre PoC envoie déjà des payloads bruts : vérifier qu’ils respectent **DEB** (command id + corps) et la taille `V.Buf-size`.

## Liens

- [[01-rfc5024/05-commandes/DATA]]
- [[01-rfc5024/02-reseau-tls]]

---

## Implémentation

- [ ] Type `ExchangeBuffer { cmd: u8, payload: Vec<u8> }`
- [ ] Split / assemble sur `N_DATA_IND` si fragmentation TCP
- [ ] Respect `Max-buf-size` local avant négociation
