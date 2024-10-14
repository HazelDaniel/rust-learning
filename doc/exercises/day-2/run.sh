#!/usr/bin/bash
#
rustc ./formatting.rs -o ./formatting.out
sleep 1
# clear
./formatting.out
