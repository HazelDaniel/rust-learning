#!/usr/bin/bash
#
rustc ./conversion.rs -o ./conversion.out
./conversion.out
sleep 1

echo "--------------------------------------------------->"

rustc ./conditionals.rs -o ./conditionals.out
./conditionals.out
sleep 1
