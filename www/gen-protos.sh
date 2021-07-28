#!/bin/bash

set -e

export DIR=$(dirname $0)

cd $DIR
cmd="$@"

if [ -z "$SKIPPROTOCOMPILATION" ]; then
  echo "Installing compiler"
  pip3 install "grpc_web_proto_compile>=1.1.0"
  echo "Compiling protos"
  grpc_web_proto_compile \
    --grpc_web_plugin_version="1.2.1" \
    -clear ../proto ./src/generated
fi

exec $cmd
