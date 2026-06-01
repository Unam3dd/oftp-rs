---
groupe: etats
hub: true
tags: [groupe/etats]
---

# Hub — Tables d’états (§9)

> Groupe **orange** du graphe.

## Vue d’ensemble

- [[00-vue-ensemble]]
- [[00-actions-et-predicats|Actions et prédicats §9.8–9.12]] ← **définitions Action 1…18 (RFC intégral)**
- [[00-index-transitions-par-PDU|Index transitions A,B,C… par PDU]] ← **PDU → table §9**
- [[../09-tables-etats|Index §9 complet]]
- [[../../00-index/Rôles client serveur et tables|Client / Serveur / Speaker]]

## Tables (ordre d’implémentation)

1. [[9.8-session-connection]] — ouverture session (SSRM, SSID)
2. [[9.9-error-abort]] — timeout, abort, buffer invalide
3. [[9.10-speaker-1]] — envoi fichier, DATA, EERP
4. [[9.11-speaker-2]] — EFID, clôture
5. [[9.12-listener]] — réception, RTR

## Carte visuelle

- [[../../Cartes/Tables états.canvas|Carte Tables états]]

## Commandes (groupe bleu)

→ [[../05-commandes/_MOC Commandes PDU]]
