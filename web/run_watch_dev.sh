#!/bin/bash
source .env
sudo sysctl -w net.ipv4.ip_unprivileged_port_start=0
RUSTFLAGS="--cfg erase_components" DB_HOST=localhost PUBLIC_URL=localhost:80 DB_USER=user DB_PASS=pass DB_NAME=z80emu COMPILER_HOST="localhost:4560" cargo leptos watch
