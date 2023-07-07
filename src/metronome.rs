//
// use std::process::Command;
//
// use rustyline::{Editor, history::FileHistory};
// use tokio_util::sync::CancellationToken;
//
//
// const NANO_CONVERTER:f64 = 1_000_000_000.0;
//
// pub struct Metronome {
//     prompt:Editor<(), FileHistory>,
//     cancellation_token:CancellationToken
// }
//
//
// pub async fn run(tempo:u16) {
//
//     let pause = ((60_f64 / tempo as f64) * NANO_CONVERTER) as u64;
//
//     loop {
//         let _ = Command::new("afplay").arg("sounds/pulse.wav").spawn().unwrap();
//         tokio::time::sleep(std::time::Duration::from_nanos(pause)).await;
//     }
// }
//
