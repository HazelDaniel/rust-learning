#!/usr/bin/bash
#
rustc ./modules.rs -o ./modules.out
sleep 1
# clear
./modules.out
