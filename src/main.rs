pub mod cli;

use cli::Metronome;

#[tokio::main]
async fn main() {
    let mut metronome = Metronome::new();
    metronome.start();
}

