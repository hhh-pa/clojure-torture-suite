#!/bin/sh
# Scaffold a new torture test from the template.
# Usage: new-test.sh <suite-dir> <base-name>   e.g. new-test.sh torture/execute/seqs take
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
SUITE="${1:?usage: new-test.sh <suite-dir> <base-name>}"
BASE="${2:?usage: new-test.sh <suite-dir> <base-name>}"
DIR="$ROOT/$SUITE"
mkdir -p "$DIR"
n=$(ls "$DIR" 2>/dev/null | grep -c "^$BASE-" || true)
FILE=$(printf "%s/%s-%03d.clj" "$DIR" "$BASE" $((n + 1)))
if [ -e "$FILE" ]; then echo "exists: $FILE" >&2; exit 1; fi
DO=run
case "$SUITE" in
  *torture/compile*) DO=load;;
  *torture/reader*) DO=read;;
esac
cat > "$FILE" <<EOF
;; TODO: one-line description of the tortured behavior.
;; { dg-do $DO }
EOF
if [ "$DO" = run ]; then
  cat >> "$FILE" <<'EOF'
;; { dg-output "TORTURE-PASS\n" }

(assert (= 1 1))
(println "TORTURE-PASS")
EOF
elif [ "$DO" = load ]; then
  cat >> "$FILE" <<'EOF'

(defn tortured-fn [x] x)
EOF
else
  cat >> "$FILE" <<'EOF'

{:single "form"}
EOF
fi
echo "wrote $FILE"
