#!/usr/bin/env bash

set -euo pipefail
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
export CKB_TX_FILE="${CKB_TX_FILE:-$SCRIPT_DIR/tx_file.json}" CKB_RUNNING_SETUP="${CKB_RUNNING_SETUP:-$SCRIPT_DIR/running_setup.json}"

usage() {
  echo "$0 dumps"
}

if (( $# < 1 )); then
  usage
  exit
fi

declare -a tests=("${@/#/tests::dump_}")
pushd "$SCRIPT_DIR"
for test in "${tests[@]}"; do
  cargo test "${test}" -- --exact --nocapture
done
popd
