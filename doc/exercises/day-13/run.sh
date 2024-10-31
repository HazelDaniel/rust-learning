#!/usr/bin/bash
#
rustc ./libraries.rs --extern class=libclass.rlib -o ./libraries.out
sleep 1
# clear
./libraries.out
