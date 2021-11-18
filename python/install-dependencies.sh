#!/bin/bash

set -e

COMMON_DEPS="tree gcc openssl wget git gzip make autoconf automake libtool"

if [ -x "$(command -v apt-get)" ]; then
  # use apt-get
  apt-get update
  apt-get install -y $COMMON_DEPS
  apt-get install -y \
    g++-aarch64-linux-gnu \
    g++-powerpc64-linux-gnu \
    g++-s390x-linux-gnu \
    libasound2-dev \
    libffi-dev \
    python-dev \
    libssl-dev \
    g++
elif [ -x "$(command -v yum)" ]; then
  # use yum
  yum install -y $COMMON_DEPS
  yum install -y \
    alsa-lib-devel \
    libffi-devel \
    python-devel \
    openssl-devel \
    gcc-c++ \
    glibc-static \
    libstdc++-static

elif [ -x "$(command -v apk)" ]; then
  # use apk
  apk add $COMMON_DEPS
  apk add \
    libffi-dev \
    python3-dev \
    alsa-lib-dev \
    openssl-dev \
    g++
fi
