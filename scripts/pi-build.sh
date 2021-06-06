#!/bin/bash

set -e

cross build --target arm-unknown-linux-gnueabihf --release
rsync -az target/arm-unknown-linux-gnueabihf/release/eink pi@192.168.86.30:/home/pi/eink --info=progress2