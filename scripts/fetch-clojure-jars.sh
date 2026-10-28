#!/bin/sh
# Fetch JVM Clojure oracle jars into harness/lib/.
# Pinned coordinates (Clojure 1.12.0 needs spec.alpha + core.specs.alpha).
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
LIB="$ROOT/harness/lib"
mkdir -p "$LIB"
BASE=https://repo1.maven.org/maven2/org/clojure
fetch() {
  artifact="$1"; version="$2"
  jar="$artifact-$version.jar"
  url="$BASE/$artifact/$version/$jar"
  if [ -f "$LIB/$jar" ]; then
    echo "cached: $jar"
  else
    echo "fetch: $url"
    curl -fsSL --retry 3 -o "$LIB/$jar" "$url"
  fi
}
fetch clojure 1.12.0
fetch spec.alpha 0.3.218
fetch core.specs.alpha 0.2.62
ls -la "$LIB"
