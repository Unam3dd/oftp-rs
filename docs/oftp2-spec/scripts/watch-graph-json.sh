#!/usr/bin/env bash
# Surveille graph.json et réinjecte colorGroups si Obsidian l'écrase.
# Utile si le chmod ne suffit pas. Laisser tourner dans un terminal pendant la session.
#
#   ./scripts/watch-graph-json.sh
# Ctrl+C pour arrêter
set -euo pipefail
VAULT="$(cd "$(dirname "$0")/.." && pwd)"
GRAPH="$VAULT/.obsidian/graph.json"
RESTORE="$VAULT/scripts/restore-graph-groups.sh"

if ! command -v inotifywait >/dev/null 2>&1; then
  echo "Installez inotify-tools: sudo apt install inotify-tools" >&2
  exit 1
fi

echo "Surveillance de $GRAPH (Ctrl+C pour arrêter)…"
while inotifywait -e close_write,modify,moved_to "$GRAPH" 2>/dev/null; do
  EMPTY=$(python3 -c "import json; d=json.load(open('$GRAPH')); print(len(d.get('colorGroups') or []))")
  if [[ "$EMPTY" -eq 0 ]]; then
    echo "[$(date +%H:%M:%S)] colorGroups vidé → restauration…"
    chmod u+w "$GRAPH" 2>/dev/null || true
    "$RESTORE" --no-lock
    chmod a-w "$GRAPH" 2>/dev/null || true
  fi
done
