#!/usr/bin/bash
#
rustc ./enums.rs -o ./enums.out
sleep 1
# clear
./enums.out
