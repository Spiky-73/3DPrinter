@echo off

cargo build -r

scp target/armv7-unknown-linux-gnueabihf/debug/mother_board imprimante@raspiprint.local:~/Printer/MotherBoard