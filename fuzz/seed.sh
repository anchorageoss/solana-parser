#!/usr/bin/env sh
# Seed the fuzz_idl_json corpus from the IDLs the library embeds, so the seeds
# track src/solana/idls instead of living as committed copies. Idempotent.
set -eu
cd "$(dirname "$0")"
mkdir -p corpus/fuzz_idl_json
cp ../src/solana/idls/*.json corpus/fuzz_idl_json/
