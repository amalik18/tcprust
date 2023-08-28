#!/bin/bash

cargo build --release
sudo setcap cap_net_admin=eip "$CARGO_TARGET_DIR"/release/tcprust
"$CARGO_TARGET_DIR"/release/tcprust &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
trap kill $pid TERM
wait $pid
