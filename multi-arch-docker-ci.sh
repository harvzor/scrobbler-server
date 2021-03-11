#!/bin/bash
# (c) Artur.Klauser@computer.org
#
# This script installs support for building multi-architecture docker images
# with docker buildx on CI/CD pipelines like Github Actions or Travis. It is
# assumed that you start of with a fresh VM every time you run this and have to
# install everything necessary to support 'docker buildx build' from scratch.
#
# Example usage in Travis stage:
#
# jobs:
#   include:
#     - stage: Deploy docker image
#       script:
#         - source ./multi-arch-docker-ci.sh
#         - set -ex; multi_arch_docker::main; set +x
#
# More information about Linux environment constraints can be found at:
# https://nexus.eddiesinentropy.net/2020/01/12/Building-Multi-architecture-Docker-Images-With-Buildx/

function _version() {
  printf '%02d' $(echo "$1" | tr . ' ' | sed -e 's/ 0*/ /g') 2>/dev/null
}

function multi_arch_docker::install_docker_buildx() {
  # Check kernel version.
  local -r kernel_version="$(uname -r)"
  if [[ "$(_version "$kernel_version")" < "$(_version '4.8')" ]]; then
    echo "Kernel $kernel_version too old - need >= 4.8."
    exit 1
  fi

  # Install up-to-date version of docker, with buildx support.
  local -r docker_apt_repo='https://download.docker.com/linux/ubuntu'
  curl -fsSL "${docker_apt_repo}/gpg" | sudo apt-key add -
  local -r os="$(lsb_release -cs)"
  sudo add-apt-repository "deb [arch=amd64] $docker_apt_repo $os stable"
  sudo apt-get update
  sudo apt-get -y -o Dpkg::Options::="--force-confnew" install docker-ce

  # Enable docker daemon experimental support (for 'docker pull --platform').
  local -r config='/etc/docker/daemon.json'
  if [[ -e "$config" ]]; then
    sudo sed -i -e 's/{/{ "experimental": true, /' "$config"
  else
    echo '{ "experimental": true }' | sudo tee "$config"
  fi
  sudo systemctl restart docker

  # Install QEMU multi-architecture support for docker buildx.
  docker run --rm --privileged multiarch/qemu-user-static --reset -p yes

  # Enable docker CLI experimental support (for 'docker buildx').
  export DOCKER_CLI_EXPERIMENTAL=enabled
  # Instantiate docker buildx builder with multi-architecture support.
  docker buildx create --name mybuilder
  docker buildx use mybuilder
  # Start up buildx and verify that all is OK.
  docker buildx inspect --bootstrap
}

# Run buildx build and push.
# Env:
#   DOCKER_PLATFORMS ... space separated list of Docker platforms to build.
function multi_arch_docker::build() {
  docker buildx build \
    --platform "${DOCKER_PLATFORMS}" \
    -t "$IMAGE_NAME:latest" -t "$IMAGE_NAME:$TRAVIS_BUILD_NUMBER" \
    --progress plain \
    .
}

# Try to solve:
#9 [5/7] RUN cargo build --release
#9 sha256:18d66e9af217609109ab7171b73584c9a4ad2b194d2689b2212723e765b17ce7
#9 3.581     Updating crates.io index
#9 3.761 warning: spurious network error (2 tries remaining): could not read directory '/usr/local/cargo/registry/index/github.com-1285ae84e5963aae/.git//refs': Value too large for defined data type; class=Os (2)
#9 3.871 warning: spurious network error (1 tries remaining): could not read directory '/usr/local/cargo/registry/index/github.com-1285ae84e5963aae/.git//refs': Value too large for defined data type; class=Os (2)
#9 3.997 error: failed to get `chrono` as a dependency of package `core v0.1.0 (/app/core)`
# https://www.reddit.com/r/rust/comments/dn1g58/spurios_network_error_and_failed_to_load_source/f59861u/?utm_source=reddit&utm_medium=web2x&context=3
function multi_arch_docker::fix_libgit2() {
    mkdir ~/.cargo
    echo "[http]\ntimeout = 29 # default is 30. Any non-default config works." >> ~/.cargo/config
}

function multi_arch_docker::main() {
  set -ex
  # Set docker platforms for which to build (careful, takes forever!)
  export DOCKER_PLATFORMS='linux/arm/v7'
  # DOCKER_PLATFORMS+=',linux/amd64'
  # DOCKER_PLATFORMS+=',linux/arm/v6'
  # DOCKER_PLATFORMS+=',linux/arm64'

  export IMAGE_NAME=${DOCKERHUB_USER}/${IMAGE_REPO_NAME}

  multi_arch_docker::install_docker_buildx
  multi_arch_docker::fix_libgit2
  multi_arch_docker::build
  set +x
  return 0
}
