#!/usr/bin/env sh
set -eu

buf format --diff --exit-code
buf lint
