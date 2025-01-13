
#!/bin/zsh
set -e

echo $(../target/release/scs key insert --key-type gran --scheme ed25519 --base-path ../data  --suri //$SESSION_KEYS_PASSWORD//fir//ed//$INDEX)
echo $(../target/release/scs key insert --key-type babe --scheme sr25519 --base-path ../data  --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX)
echo $(../target/release/scs key insert --key-type imon --scheme sr25519 --base-path ../data  --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX)
echo $(../target/release/scs key insert --key-type auth --scheme sr25519 --base-path ../data  --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX)
echo $(../target/release/scs key insert --key-type mixn --scheme sr25519 --base-path ../data  --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX)
echo $(../target/release/scs key insert --key-type beef --scheme ecdsa --base-path ../data  --suri //$SESSION_KEYS_PASSWORD//fir//ecdsa//$INDEX)
