use temp_dir::TempDir;
extern crate reqwest;

use std::fs::{self, File};
use std::io::Error;
use std::io::Result;
use std::{io, process};

fn main() -> Result<()> {
    let marketstore_proto_dir = TempDir::new().unwrap();

    let mut marketstore_poto = reqwest::blocking::get(
        "https://raw.githubusercontent.com/alpacahq/marketstore/master/proto/marketstore.proto",
    )
    .expect("failed to downalod marketstore.proto");
    let marketstore_poto_local_path = marketstore_proto_dir.child("marketstore.proto");

    let mut marketstore_poto_file = File::create(&marketstore_poto_local_path.to_str().unwrap())
        .expect("failed to create file");
    io::copy(&mut marketstore_poto, &mut marketstore_poto_file).expect("failed to copy content");

    fs::create_dir_all("src")?;

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir(&"src")
        .compile(
            &[marketstore_poto_local_path],
            &[marketstore_proto_dir.path()],
        )
}
