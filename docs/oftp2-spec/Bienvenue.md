---
groupe: index
tags: [groupe/index]
---

# Coffre OFTP2 — RFC 5024

Ce coffre sert à **décomposer la RFC 5024** (ODETTE File Transfer Protocol 2) pour implémenter un client/serveur en Rust sans rien oublier.

## Par où commencer

1. [[00-index/Parcours implémentation Rust|Parcours implémentation Rust]] — ordre de lecture et de code
2. [[00-index/Rôles client serveur et tables|Rôles client / serveur / Speaker]] — quelle table utiliser
3. [[00-index/OFTP2 - carte mentale|Carte mentale]] — vue d’ensemble des chapitres
4. [[01-rfc5024/09-tables/00-vue-ensemble|§9 Tables d’états]] — matrices et transitions complètes
5. [[01-rfc5024/05-commandes/00-identifiants-pdu|Index PDU]] — chaque commande décrite
4. [[01-rfc5024/03-service-etats|§3 Service et automates]] — API `F_*` côté application

## Sources

| Document | Fichier | Usage |
|----------|---------|--------|
| **RFC 5024** (OFTP2) | `RFCs/rfc5024.txt` | Référence principale |
| RFC 2204 (OFTP1) | `RFCs/rfc2204.txt` | Historique / deltas v1→v2 |

## Structure du coffre (groupes graphe)

Chaque dossier a une note **`_MOC …`** (hub) et une **couleur** dans la vue graphique.

| Hub | Groupe (couleur) |
|-----|------------------|
| [[00-index/_MOC Index]] | Index (jaune) |
| [[01-rfc5024/_MOC Chapitres RFC]] | Chapitres (cyan) |
| [[01-rfc5024/05-commandes/_MOC Commandes PDU]] | Commandes PDU (bleu) |
| [[01-rfc5024/09-tables/_MOC Tables états]] | Tables §9 (orange) |
| [[02-cas-speciaux/_MOC Cas spéciaux]] | Cas spéciaux (vert) |
| [[03-rust/_MOC Rust]] | Rust (violet) |

→ **Vues groupées (recommandé)** : [[Cartes/_MOC Cartes|Cartes Canvas]]  
→ Graphe coloré : [[00-index/Légende du graphe|Légende du graphe]] · [[00-index/Configuration Obsidian|Config `.obsidian`]]

## Convention des notes

Chaque note RFC contient en bas une section **Implémentation** avec cases à cocher Obsidian (`- [ ]`). Utilisez le modèle [[Templates/Note RFC|Note RFC]] pour les nouvelles fiches.

## Tags utiles

- `#rfc5024` — lié à la norme
- `#état` — machine à états
- `#pdu` — commande sur le fil
- `#service` — primitives `F_*`
- `#rust` — décisions crate
- `#fait` / `#à-faire` — suivi d’avancement
