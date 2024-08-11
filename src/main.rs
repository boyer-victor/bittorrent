mod bencode;
mod torrent;

use std::env;

// Usage: run.sh command <arg1> ...
fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "decode" => {
            let mut encoded_value = args[2].as_str();
            let decoded_value = bencode::decode(&mut encoded_value);
            println!("{}", decoded_value.to_string());
        },
        "info" => {
            let file_name = args[2].as_str();
            let torrent = torrent::read_file(file_name);
            println!("Tracker URL: {}", torrent.announce.to_string());
            println!("Length: {}", torrent.length().to_string());
        },
        _ => {
            println!("unknown command: {}", args[1]);
        }
    }
}
