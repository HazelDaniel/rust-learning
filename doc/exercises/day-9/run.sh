#!/usr/bin/bash
#
rustc ./functions.rs -o ./functions.out
sleep 1
# clear
./functions.out
