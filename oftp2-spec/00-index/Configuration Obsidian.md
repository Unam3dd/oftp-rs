---
groupe: index
tags: [groupe/index]
---

# Configuration Obsidian — graphe qui s’efface

## Pourquoi `colorGroups` redevient `[]`

Dès que vous ouvrez la **vue graphique**, Obsidian **réenregistre** `.obsidian/graph.json` et remet souvent `"colorGroups": []`. Ce n’est pas vous : c’est le comportement normal d’Obsidian.

## Solution recommandée : verrouiller `graph.json`

Une fois les couleurs fusionnées, le script met le fichier en **lecture seule**. Obsidian **ne peut plus** écraser les groupes (l’écriture échoue, le fichier reste intact).

### Procédure (une fois)

1. **Fermez Obsidian** complètement  
2. Dans un terminal :

```bash
cd /home/stales/WORK/oftp2-spec
./scripts/restore-graph-groups.sh
```

3. Rouvrez Obsidian → ouvrez le graphe → les **couleurs doivent rester**  
4. Vérifiez : `.obsidian/graph.json` doit toujours contenir 10 entrées dans `colorGroups`

### Modifier les couleurs ou le zoom enregistré dans le JSON

```bash
./scripts/restore-graph-groups.sh --unlock   # autorise l’écriture
# … éditez .obsidian/graph-color-groups.source.json pour les couleurs …
./scripts/restore-graph-groups.sh            # refusionne + reverrouille
```

**Éditez les couleurs dans** : `.obsidian/graph-color-groups.source.json` (pas dans `graph.json` directement).

## Plan B : surveillance automatique

Si le verrouillage ne suffit pas (rare) :

```bash
sudo apt install inotify-tools   # une fois
./scripts/watch-graph-json.sh    # laisser tourner pendant votre session Obsidian
```

À chaque effacement, les groupes sont réinjectés en ~1 seconde.

## Plan C : ne pas utiliser le graphe global

Les **Cartes** (`.canvas`) ne sont pas touchées par ce bug :

- [[../Cartes/Vue d'ensemble.canvas]]
- [[../Cartes/Commandes PDU.canvas]]
- [[../Cartes/Tables états.canvas]]

C’est la vue la plus stable pour voir **qui est relié à quoi**.

## Fichiers utiles

| Fichier | Rôle |
|---------|------|
| `.obsidian/graph-color-groups.source.json` | **Source** des 10 groupes (à éditer) |
| `.obsidian/graph.json` | Config graphe (verrouillé après restore) |
| `.obsidian/.graph-json-locked` | Indique que le verrou est actif |
| `scripts/restore-graph-groups.sh` | Fusion + verrou |
| `scripts/watch-graph-json.sh` | Gardien optionnel |

Voir aussi [[../.obsidian/README]].
