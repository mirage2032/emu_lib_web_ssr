#!/bin/bash
source .env
RUSTFLAGS="--cfg erase_components" DB_HOST=localhost DB_USER=user DB_PASS=pass DB_NAME=z80emu COMPILER_HOST="localhost:4560" cargo leptos build --release -vv
