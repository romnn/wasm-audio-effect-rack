#!/bin/bash

set -e

PROTOBUF_VERSION="${PROTOBUF_VERSION:-v3.19.1}"
PREFIX="${PREFIX:-/usr}"
echo "using protobuf $PROTOBUF_VERSION"
echo "will install into $PREFIX"

git clone https://github.com/protocolbuffers/protobuf.git protobuf
cd protobuf
git checkout $PROTOBUF_VERSION
git submodule update --init --recursive
bash ./autogen.sh || bash ./autogen.sh

# OS=$1
# ARCH=$2
# DEST=$3

# ./protoc-artifacts/build-protoc.sh $OS $ARCH protoc
# cp ./target/$OS/$ARCH/protoc $DEST/protoc

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

./configure --prefix=$PREFIX --disable-shared
make -j8
make install
