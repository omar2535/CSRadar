# This script updates the offsets from CS2-OFFSETS REPOSITORY
# Should be ran from the root of the repository such that you are calling the script like
# ./bin/update_offsets.ps1

Write-Output "Updating offsets"

$GIT_REPO = "https://github.com/a2x/cs2-dumper/raw/main"
$OFFSET_SRC_PATH = "src/cs2_offsets"

Invoke-WebRequest -Uri "$GIT_REPO/output/client.dll.rs" -OutFile "$OFFSET_SRC_PATH/client_dll.rs"
Invoke-WebRequest -Uri "$GIT_REPO/output/engine2.dll.rs" -OutFile "$OFFSET_SRC_PATH/engine2_dll.rs"
Invoke-WebRequest -Uri "$GIT_REPO/output/offsets.rs" -OutFile "$OFFSET_SRC_PATH/offsets.rs"
Invoke-WebRequest -Uri "$GIT_REPO/output/server.dll.rs" -OutFile "$OFFSET_SRC_PATH/server_dll.rs"

Write-Output "Offsets updated! Remember to commit and push!"
