use crate::htm::htm_model::HTMModel;
use crate::htm::temporal_keys::TemporalKey;
use prettytable::*;
use prettytable::{format, Table};
use std::thread;
use std::time::Duration;

pub fn run_temporal_key_demo() {
    println!("⏳ Running Temporal Key Demo...");
    let initial_key: Vec<u8> = vec![0; 256];

    // Creating an instance of HTMModel.
    // Ensure HTMModel::new() is a valid call and add arguments if needed.
    let htm_model = HTMModel::new();

    // Creating a TemporalKey instance with initial key and HTMModel.
    let evolution_interval = std::time::Duration::from_secs(10); // 10 seconds, for example
    let mut temporal_key = TemporalKey::new(initial_key, htm_model, evolution_interval);

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    // Add the table headers with emojis
    table.add_row(row!["📚 Description", "🔑 Key", "⏰ Generation Time"]);

    // Add the initial state of the TemporalKey
    table.add_row(row![
        "🔑 Initial Key",
        format!("{:?}", temporal_key.get_key()),
        format!("{:?}", temporal_key.generation_time)
    ]);

    // Sleep for some time and evolve the key.
    thread::sleep(Duration::from_secs(5));
    temporal_key.evolve_key(1);

    // Add the state of the TemporalKey after 5 seconds
    table.add_row(row![
        "⏲️ Key After 5 Seconds",
        format!("{:?}", temporal_key.get_key()),
        format!("{:?}", temporal_key.generation_time)
    ]);

    // Sleep for some more time and evolve the key again.
    thread::sleep(Duration::from_secs(11));
    temporal_key.evolve_key(1);

    // Add the state of the TemporalKey after an additional 11 seconds
    table.add_row(row![
        "⏲️ Key After Additional 11 Seconds",
        format!("{:?}", temporal_key.get_key()),
        format!("{:?}", temporal_key.generation_time)
    ]);

    // Print the table
    table.printstd();
}
