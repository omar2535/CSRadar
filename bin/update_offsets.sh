#!/bin/bash

# This script updates the offsets from CS2-OFFSETS REPOSITORY
# Should be ran from the root of the repository such that you are calling the script like
# ./bin/update_offsets.sh

echo "Updating offsets!"

$GIT_REPO="https://github.com/sezzyaep/CS2-OFFSETS/raw/main"
$OFFSET_SRC_PATH="src/cs2_offsets"

wget $GIT_REPO/client.dll.rs -O $OFFSET_SRC_PATH/client_dll.rs
wget $GIT_REPO/engine2.dll.rs -O $OFFSET_SRC_PATH/engine2_dll.rs
wget $GIT_REPO/offsets.rs -O $OFFSET_SRC_PATH/offsets.rs
WGET $GIT_REPO/server.dll.rs -O $OFFSET_SRC_PATH/server_dll.rs

echo "Offsets updated! Remember to commit and push!"
