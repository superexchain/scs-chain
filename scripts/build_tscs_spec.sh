#!/bin/bash

cargo build --release --features tscs
../target/release/scs build-spec --raw  --base-path db --chain tscs-local --disable-default-bootnode > tscs-chain-spec.json
