#!/bin/bash
set -e

echo $(/usr/local/bin/scs key generate-node-key --base-path $BASE_PATH)

echo "The node has been initialized successfully."