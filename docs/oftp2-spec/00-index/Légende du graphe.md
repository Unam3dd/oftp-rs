---
groupe: index
tags: [groupe/index]
---

# Légende — Graphe vs Cartes

## Problème du graphe « hairball »

Trop de liens `[[wikilink]]` entre dossiers mélangent tout en gris. Deux solutions complémentaires :

---

## 1. Cartes Canvas (recommandé pour les groupes)

**Vrais groupes** avec rectangles et flèches de flux.

→ [[../Cartes/_MOC Cartes|Ouvrir les cartes]]

| Carte | Usage |
|-------|--------|
| `Vue d'ensemble.canvas` | Architecture : Index → Chapitres → PDU / Tables → Rust |
| `Commandes PDU.canvas` | Toutes les PDU + ordre protocolaire |
| `Tables états.canvas` | Tables §9 enchaînées |

---

## 2. Graphe global — couleurs par tag

Chaque note a le tag `#groupe/…`. Configuration dans `.obsidian/graph.json`.

| Tag | Couleur | Contenu |
|-----|---------|---------|
| `#groupe/commandes` | Bleu | `05-commandes/` |
| `#groupe/etats` | Orange | `09-tables/` |
| `#groupe/cas-speciaux` | Vert | `02-cas-speciaux/` |
| `#groupe/rust` | Violet | `03-rust/` |
| `#groupe/index` | Jaune | `00-index/`, Bienvenue |
| `#groupe/chapitre` | Cyan | chapitres `01-rfc5024/*.md` |

### Activer les couleurs (si tout est gris)

Obsidian **écrase** parfois `graph.json` à la fermeture du graphe :

1. **Fermer Obsidian** complètement
2. Rouvrir le coffre
3. Ouvrir le graphe → panneau **Groupes** en bas : les 8 groupes doivent apparaître
4. Sinon : **Groupe** → **Nouveau groupe** → requête `tag:groupe/commandes` → couleur bleue (répéter pour chaque tag)

### Filtrer un seul groupe

Dans la barre de recherche du graphe :

```
tag:groupe/commandes
```

Ou masquer un groupe en décochant sa case dans **Groupes**.

### Graphe local (propre)

1. Ouvrir [[../01-rfc5024/05-commandes/_MOC Commandes PDU]]
2. Panneau droit → **Graphe local**
3. Profondeur **1** ou **2** → seulement les PDU liées au hub

---

## 3. Hubs `_MOC`

Les notes `_MOC …` centralisent les liens **à l’intérieur** d’un dossier. Éviter de lier chaque PDU vers chaque table : passer par le hub.

| Hub |
|-----|
| [[_MOC Index]] |
| [[../01-rfc5024/05-commandes/_MOC Commandes PDU]] |
| [[../01-rfc5024/09-tables/_MOC Tables états]] |

---

## Réglages graphe appliqués

- `linkStrength: 0.25` — liens plus faibles, clusters plus distincts
- `repelStrength: 22` — nœuds plus espacés
- `showOrphans: false` — notes isolées masquées
