#!/bin/bash
set -e

echo $(/usr/local/bin/scs key generate-node-key --base-path /data)

echo "The node has been initialized successfully."