#!/usr/bin/env bash

#=======================================================================#
# Copyright (C) 2020 - 2024 Dominik Willner <th33xitus@gmail.com>       #
#                                                                       #
# This file is part of KIAUH - Klipper Installation And Update Helper   #
# https://github.com/dw-0/kiauh                                         #
#                                                                       #
# This file may be distributed under the terms of the GNU GPLv3 license #
#=======================================================================#

set -e
clear -x

# make sure we have the correct permissions while running the script
umask 022

function launch_rust_kiauh_v1() {
  validate_cargo_installed
  cargo run --release
}

function validate_cargo_installed() {
  if ! command -v cargo 2>&1 /dev/null
  then
      echo "Cargo could not be found, install using rustup? (y/n)"

      read -r answer
      if [[ "${answer}" == "y" || "${answer}" == "Y" ]]
      then
          curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --profile minimal -y
      else
          echo "Install aborted, exiting..."
          exit 1
      fi
  fi
}

function main() {
  launch_rust_kiauh_v1
}

main