use skyseeker_core::codec::encode;
use std::path::PathBuf;

mod bsc5;

fn main() {
    let data_dir = PathBuf::from("./../data");
    let bsc5_input = data_dir.join("bsc5-all.json");
    let bsc5_output = data_dir.join("bsc5-stars.bin");

    let bsc5_data = std::fs::read_to_string(bsc5_input).unwrap();
    let stars = bsc5::parse(bsc5_data).unwrap();
    let star_data = encode(&stars).unwrap();
    std::fs::write(bsc5_output, star_data).unwrap();
}
