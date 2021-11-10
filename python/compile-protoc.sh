#!/bin/bash

set -e

yum install -y autoconf \
  automake \
  libtool \
  gzip \
  make \
  wget \
  g++ \
  git

git clone https://github.com/protocolbuffers/protobuf.git protobuf
cd protobuf
git checkout v3.19.1
git submodule update --init --recursive
./autogen.sh

# cd protoc-artifacts/
# ./autogen.sh

./configure --prefix=/usr/bin
make -B
sudo make install

# ./build-protoc.sh linux ppcle_64 protoc
# ./build-protoc.sh linux s390_64 protoc
# file target/linux/ppcle_64/protoc.exe
# file target/linux/s390_64/protoc.exe
# cp target/linux/ppcle_64/protoc.exe $GITHUB_WORKSPACE/ppcle_64_protoc
# cp target/linux/s390_64/protoc.exe $GITHUB_WORKSPACE/s390_64_protoc
# echo $(realpath target/linux/ppcle_64/protoc.exe)
