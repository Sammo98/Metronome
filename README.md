# tock-rs

tock-rs is a simple, cross-platform,  metronome CLI application written in Rust. 

## Features

* A REPL-like interface for controlling the metronome.
* Changing BPM
* Downbeat Toggle
* Time Signature Change (including variable time signatures)

## Usage

When running the following commands are available

* `bpm <bpm>` - Change the BPM of the metronome. Uses crochets (1/4 notes). E.g. `bpm 200`
* `ts <time_signature>` - Change the time signature. This is stored as `Vec` internally allowing for complex time signatures. E.g. `ts 5 4` would alternate between 5/4 and 4/4 time signatures.
* `db|downbeat` - Toggle whether the downbeat is signified at the start of the bar. 
* `help` - Show help information
* `q|quit|exit` - Quits the application.
