#!/usr/bin/env bash
set -euo pipefail

cargo +nightly run -Z build-std=core -Z build-std-features=panic_immediate_abort

