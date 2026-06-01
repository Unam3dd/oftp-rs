# Configuration Obsidian — coffre `oftp2-spec`

## Fichiers de ce dossier

| Fichier | Rôle |
|---------|------|
| `graph.json` | Vue graphique : physique, filtres, **`colorGroups`** (couleurs par dossier) |
| `workspace.json` | Fenêtres ouvertes, onglets, zoom graphe (généré par Obsidian) |
| `app.json` | Préférences app pour ce coffre (vide = défaut) |
| `appearance.json` | Thème / CSS snippets (vide = défaut) |
| `core-plugins.json` | Plugins natifs activés (graphe, canvas, propriétés…) |
| `graph.json.backup-*` | Sauvegarde avant restauration des couleurs |

## Problème fréquent : graphe tout gris

**Cause** : `graph.json` contient `"colorGroups": []`. Obsidian **réécrit** ce fichier à la fermeture si les groupes n’ont pas été créés via l’UI du graphe.

**Correctif durable** (Obsidian **fermé**) :

```bash
./scripts/restore-graph-groups.sh
```

Le script fusionne `.obsidian/graph-color-groups.source.json` puis **verrouille** `graph.json` (`chmod a-w`). Obsidian **ne peut plus** le réécrire → `colorGroups` ne redevient plus `[]`.

- Déverrouiller : `./scripts/restore-graph-groups.sh --unlock`
- Gardien si besoin : `./scripts/watch-graph-json.sh` (terminal ouvert pendant la session)

Détails : [[../00-index/Configuration Obsidian]].

**Alternative stable** : utiliser les **Cartes** (`Cartes/*.canvas`) — les groupes visuels ne dépendent pas de `graph.json`.

## Groupes de couleur actuels

| Requête | Couleur | Contenu |
|---------|---------|---------|
| `path:Cartes` | Rose | Toiles Canvas |
| `path:01-rfc5024/05-commandes` | Bleu | PDU |
| `path:01-rfc5024/09-tables` | Orange | Tables §9 |
| `path:02-cas-speciaux` | Vert | Cas limites |
| `path:03-rust` | Violet | Rust |
| `path:00-index` | Jaune | Index |
| `path:RFCs` | Gris | Fichiers .txt |
| `path:Templates` | Gris clair | Modèles |
| `file:Bienvenue` | Jaune | Accueil |
| `path:01-rfc5024` | Cyan | Chapitres (hors sous-dossiers ci-dessus) |

L’ordre compte : **le premier match gagne** (sous-dossiers avant le parent `01-rfc5024`).

## Plugins activés (`core-plugins.json`)

Graphe, Canvas, Propriétés, Backlinks, Templates, Sync, Bases — adaptés à ce coffre spec/RFC.

## Sauvegarde manuelle des groupes

Copie de secours : `../00-index/graph-groups-backup.json`
