#!/usr/bin/bash
#
rustc ./control.rs -o ./control.out
sleep 1
# clear
./control.out
