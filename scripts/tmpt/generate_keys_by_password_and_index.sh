#!/bin/zsh
set -e

# ed25519 
subkey inspect --scheme ed25519 --output-type json "//$SESSION_KEYS_PASSWORD"//fir//ed//$INDEX
# sr25519
subkey inspect --scheme sr25519 --output-type json  "//$SESSION_KEYS_PASSWORD"/fir/sr/$INDEX
# ecdsa
subkey inspect --scheme ecdsa --output-type json "//$SESSION_KEYS_PASSWORD"//fir//ecdsa//$INDEX


