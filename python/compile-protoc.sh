#!/bin/bash

set -e

# gcc-c++ is g++ under apt
yum install -y autoconf \
  automake \
  libtool \
  gzip \
  make \
  wget \
  gcc-c++ \
  glibc-static \
  libstdc++-static \
  git

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

# make distclean
./configure --prefix=/usr --disable-shared
make -B -j8 || echo "" > /dev/null
make install
which protoc

# export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/usr/local/lib"
# ./build-protoc.sh linux ppcle_64 protoc
# ./build-protoc.sh linux s390_64 protoc
# file target/linux/ppcle_64/protoc.exe
# file target/linux/s390_64/protoc.exe
# cp target/linux/ppcle_64/protoc.exe $GITHUB_WORKSPACE/ppcle_64_protoc
# cp target/linux/s390_64/protoc.exe $GITHUB_WORKSPACE/s390_64_protoc
# echo $(realpath target/linux/ppcle_64/protoc.exe)
