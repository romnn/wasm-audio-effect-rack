#!/bin/bash

set -e

DIR=$(realpath $(dirname "$0"))
BUILD="${DIR}/.protoc-build"
OUT="${DIR}/protoc"

PROTOBUF_VERSION="v3.19.1"

mkdir -p ${OUT}
for CONTAINER_TYPE in musllinux_1_1 manylinux2014 
do
  for ARCH in x86_64 i686 aarch64 ppc64le s390x
  do
    CONTAINER="quay.io/pypa/${CONTAINER_TYPE}_${ARCH}"
    echo "building ${CONTAINER} ..."

    rm -rf ${BUILD}
    mkdir -p ${BUILD}

    docker run \
      -v ${BUILD}:/out \
      -v ${DIR}/compile-protoc.sh:/compile.sh \
      -v ${DIR}/install-dependencies.sh:/install.sh \
      -e PROTOBUF_VERSION="${PROTOBUF_VERSION}" \
      -e PREFIX="/out" \
      ${CONTAINER} \
      /bin/bash -c "bash install.sh && bash compile.sh"

    cp "${BUILD}/bin/protoc" "$OUT/${CONTAINER_TYPE}_${ARCH}_protoc"
    zip "$OUT/${CONTAINER_TYPE}_${ARCH}_protoc.zip" "${BUILD}/bin/protoc"
    rm -rf ${BUILD}

  done
done
