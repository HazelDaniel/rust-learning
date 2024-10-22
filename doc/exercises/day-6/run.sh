#!/usr/bin/bash
#
rustc ./conversion.rs -o ./conversion.out
sleep 1
# clear
./conversion.out
