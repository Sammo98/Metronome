pub mod parse;
pub mod metronome;

use metronome::Metronome;

#[tokio::main]
async fn main() {
    let mut metronome = Metronome::default();
    metronome.start_metronome();
}
