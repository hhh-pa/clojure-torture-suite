#!/bin/sh
# Fetch JVM Clojure oracle jars into harness/lib/.
# Pinned coordinates (Clojure 1.12.0 needs spec.alpha + core.specs.alpha),
# verified against pinned SHA-256 hashes.
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
LIB="$ROOT/harness/lib"
mkdir -p "$LIB"
BASE=https://repo1.maven.org/maven2/org/clojure
if command -v sha256sum >/dev/null 2>&1; then
  SHA256="sha256sum"
elif command -v shasum >/dev/null 2>&1; then
  SHA256="shasum -a 256"
else
  echo "need sha256sum or shasum" >&2
  exit 1
fi
verify() {
  jar="$1"; want="$2"
  got=$($SHA256 "$LIB/$jar" | awk '{print $1}')
  if [ "$got" != "$want" ]; then
    echo "checksum mismatch: $jar (want $want, got $got)" >&2
    rm -f "$LIB/$jar"
    exit 1
  fi
}
fetch() {
  artifact="$1"; version="$2"; want="$3"
  jar="$artifact-$version.jar"
  url="$BASE/$artifact/$version/$jar"
  if [ -f "$LIB/$jar" ]; then
    echo "cached: $jar"
  else
    echo "fetch: $url"
    curl -fsSL --retry 3 -o "$LIB/$jar" "$url"
  fi
  verify "$jar" "$want"
}
fetch clojure 1.12.0 c45333006441a059ea9fdb1341fc6c1f40b921a10dccd82665311e48a0384763
fetch spec.alpha 0.3.218 67ec898eb55c66a957a55279dd85d1376bb994bd87668b2b0de1eb3b97e8aae0
fetch core.specs.alpha 0.2.62 06eea8c070bbe45c158567e443439681bc8c46e9123414f81bfa32ba42d6cbc8
ls -la "$LIB"
