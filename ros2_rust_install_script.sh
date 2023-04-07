#!/bin/bash -xve

sudo apt install curl gnupg2 lsb-release python3-pip git clang

sudo curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

sudo apt update -y
sudo apt install -y ros-humble-desktop python3-colcon-common-extensions

pip install git+https://github.com/tier4/colcon-cargo.git
pip install git+https://github.com/colcon/colcon-ros-cargo.git

. /opt/ros/humble/setup.bash

git clone https://github.com/tier4/cargo-ament-build.git
cd cargo-ament-build
cargo install --path .
cd ..
rm cargo-ament-build -r