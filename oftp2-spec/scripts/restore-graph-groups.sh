#!/usr/bin/env bash
# Fusionne graph-color-groups.source.json → graph.json puis verrouille le fichier.
# Obsidian ne pourra plus écraser colorGroups (échec d'écriture silencieux).
#
# Usage:
#   ./scripts/restore-graph-groups.sh          # Obsidian FERMÉ recommandé
#   ./scripts/restore-graph-groups.sh --no-lock  # fusion sans chmod
#   ./scripts/restore-graph-groups.sh --unlock   # rend graph.json modifiable
set -euo pipefail
VAULT="$(cd "$(dirname "$0")/.." && pwd)"
GRAPH="$VAULT/.obsidian/graph.json"
SOURCE="$VAULT/.obsidian/graph-color-groups.source.json"
LOCKFILE="$VAULT/.obsidian/.graph-json-locked"

do_unlock() {
  if [[ -f "$GRAPH" ]]; then
    chmod u+w "$GRAPH" 2>/dev/null || true
  fi
  rm -f "$LOCKFILE"
  echo "graph.json déverrouillé (Obsidian peut l'écraser à nouveau)."
  exit 0
}

[[ "${1:-}" == "--unlock" ]] && do_unlock

LOCK=1
[[ "${1:-}" == "--no-lock" ]] && LOCK=0

if [[ ! -f "$SOURCE" ]]; then
  echo "Erreur: $SOURCE introuvable" >&2
  exit 1
fi

# Déverrouiller avant écriture
chmod u+w "$GRAPH" 2>/dev/null || true

if [[ -f "$GRAPH" ]]; then
  cp "$GRAPH" "$VAULT/.obsidian/graph.json.backup-$(date +%Y%m%d-%H%M)"
fi

python3 << PY
import json
from pathlib import Path

graph = Path("$GRAPH")
source = Path("$SOURCE")
src = json.loads(source.read_text())
groups = src["colorGroups"]

if graph.exists():
    data = json.loads(graph.read_text())
else:
    data = {
        "collapse-filter": False,
        "search": "",
        "showTags": True,
        "showAttachments": False,
        "hideUnresolved": False,
        "showOrphans": False,
        "collapse-color-groups": False,
        "collapse-display": True,
        "showArrow": false,
        "textFadeMultiplier": 0,
        "nodeSizeMultiplier": 1,
        "lineSizeMultiplier": 1,
        "collapse-forces": True,
        "centerStrength": 0.5,
        "repelStrength": 12,
        "linkStrength": 0.4,
        "linkDistance": 150,
        "scale": 1,
        "close": False,
    }

data["colorGroups"] = groups
data["collapse-color-groups"] = False
graph.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
print(f"Fusion OK: {len(groups)} groupes dans graph.json")
PY

if [[ "$LOCK" -eq 1 ]]; then
  chmod a-w "$GRAPH"
  date -Iseconds > "$LOCKFILE"
  echo "Verrouillé: chmod a-w sur graph.json"
  echo "  → Obsidian peut encore AFFICHER le graphe ; il ne peut plus vider colorGroups."
  echo "  → Pour modifier zoom/physique: ./scripts/restore-graph-groups.sh --unlock"
else
  rm -f "$LOCKFILE"
  echo "Sans verrou (--no-lock). Obsidian pourra réécraser le fichier."
fi

echo "Rouvrez le coffre ou Ctrl+R dans Obsidian."
