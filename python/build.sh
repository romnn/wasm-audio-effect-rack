#!/bin/bash

set -e

export ROOT=$(realpath "$(dirname $0)/../")
export SRC="${ROOT}/python"
export OUT="${SRC}/cibuildwheel.out"

cd $ROOT

if [ ! -z "$BUILD_CONTAINER" ]; then
  echo "Building docker container..."
  docker build --load --progress auto -t manylinux2014rust -f $SRC/Dockerfile .
fi

export CIBW_BUILD='cp38-*'
export CIBW_SKIP='*-win32'
export CIBW_PLATFORM='linux'
export CIBW_TEST_REQUIRES='pytest'
export CIBW_TEST_COMMAND='pytest {project}/python/tests -s'
export CIBW_ENVIRONMENT='PATH="$HOME/.cargo/bin:$PATH"'
export CIBW_ENVIRONMENT_WINDOWS='PATH="$UserProfile\.cargo\bin;$PATH"'
export CIBW_MANYLINUX_X86_64_IMAGE='manylinux2014'
export CIBW_MANYLINUX_I686_IMAGE='manylinux2014'
# export CIBW_BEFORE_BUILD="\
#   yum install tree alsa-lib-devel openssl openssl-devel -y &&\
#   pip install -U setuptools-rust &&\
#   rustup default stable &&\
#   rustup show &&\
#   ls -lia"

export CIBW_BEFORE_TEST="\
  yum install tree alsa-lib-devel openssl openssl-devel -y &&\
  pip install -U pip pipenv setuptools setuptools-rust wheel &&\
  curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=stable --profile=default -y &&\
  rustup show &&\
  PIPENV_PIPFILE=./python/Pipfile pipenv lock -r > requirements.txt &&\
  pip install -U -r requirements.txt"

export CIBW_BEFORE_BUILD_LINUX="\
  yum install tree alsa-lib-devel openssl openssl-devel -y &&\
  curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=stable --profile=default -y &&\
  rustup show &&\
  pip install -U pip pipenv setuptools setuptools-rust wheel &&\
  PIPENV_PIPFILE=./python/Pipfile pipenv lock -r > requirements.txt &&\
  pip install -U -r requirements.txt &&\
  unlink python/disco-src &&\
  cp -r disco python/disco-src &&\
  cp -r proto python/proto &&\
  tree -I 'node_modules|target|build' python"

cibuildwheel --output-dir $OUT $SRC
