#!/bin/bash
cargo build --release --features tscs
../target/release/scs build-spec --raw  --base-path db --chain dscs-local --disable-default-bootnode > dscs-chain-spec.json