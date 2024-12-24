#!/bin/bash

cargo build --release --features scs
../target/release/scs build-spec --raw  --base-path db --chain scs-local --disable-default-bootnode > scs-chain-spec.json