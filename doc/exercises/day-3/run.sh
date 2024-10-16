#!/usr/bin/bash
#
rustc ./structs.rs -o ./structs.out
sleep 1
# clear
./structs.out
