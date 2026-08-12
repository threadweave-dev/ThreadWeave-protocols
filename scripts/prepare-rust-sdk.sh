#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

destination="${repository_root}/sdk/rust/proto"
rm -rf "${destination}"
mkdir -p "${destination}"
cp -R "${repository_root}/proto/." "${destination}/"
