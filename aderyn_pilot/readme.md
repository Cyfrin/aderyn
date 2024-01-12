## Bot framework creation  (how to steps for development)

### Assemble the framework
cli/bot_starter_pack_archivegen.sh

### NOTE User can create the bot anywhere 
Cargo.toml should not contain relative paths in the final archive.zip

### Command to create a bot
cargo run --bin aderyn_pilot -- new ../scarybots/spiderbot

### Command to add a detector (todo 2/3 done)
cargo run --bin aderyn_pilot -- generate ../scarybots/spiderbot/bot_starter_pack/src/scarey_events

### Command to assemble, run and test - te 
