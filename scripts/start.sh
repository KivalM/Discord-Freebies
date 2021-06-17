#! /bin/bash

cd /home/ubuntu/code/rustington
java -jar selenium-server-standalone-3.141.59.jar &
sleep 5
cargo run --release >> logs.txt