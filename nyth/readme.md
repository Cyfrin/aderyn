## Bot framework creation  (how to steps for development)

### Assemble the framework
cli/bot_archivegen.sh

### NOTE User can create the bot anywhere 
Cargo.toml should not contain relative paths in the final archive.zip

### Command to create a bot
cargo run --bin nyth -- init ../scarybots/spiderbot
cd ../scarybots/spiderbot

### Command to add an issue detector
cargo run --bin nyth -- new issue scarey_events

### Command to add a reusable detector
cargo run --bin nyth -- new reusable get_all_events_from_a_cotract
