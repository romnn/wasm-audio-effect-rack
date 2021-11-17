#!/bin/bash

set -e

git clone https://github.com/protocolbuffers/protobuf.git protobuf
cd protobuf
git checkout v3.19.1
git submodule update --init --recursive
./autogen.sh

CXXFLAGS="-DNDEBUG"
LDFLAGS=""

# Statically link libgcc and libstdc++.
# -s to produce stripped binary.
if [ "$(uname)" == "Darwin" ]; then
  # mac os
  LDFLAGS="$LDFLAGS -static-libgcc -static-libstdc++ -s"
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
  # linux
  echo ""
else
  # windows
  LDFLAGS="$LDFLAGS -static-libgcc -static-libstdc++ -Wl,-Bstatic -lstdc++ -lpthread -s"
fi

echo $CXXFLAGS
echo $LDFLAGS

./configure --prefix=/usr --disable-shared
make -j8
make install
which protoc
