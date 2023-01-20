use temp_dir::TempDir;
extern crate reqwest;

use std::fs::{self, File};
use std::io::Error;
use std::io::Result;
use std::path::Path;
use std::{io, process};

trait TonicBuilderExtended {
    fn field_attributes(self: Self, paths: Vec<&str>, attributes: Vec<&str>) -> Self;
    fn serda_rename(
        self: Self,
        path: &str,
        attributes: Vec<&str>,
        transform: fn(&str) -> String,
    ) -> Self;
}

impl TonicBuilderExtended for tonic_build::Builder {
    fn field_attributes(self: Self, paths: Vec<&str>, attributes: Vec<&str>) -> Self {
        let mut instance: Self = self;

        for path in paths {
            for attribute in &attributes {
                instance = instance.field_attribute(path, attribute)
            }
        }

        return instance;
    }

    fn serda_rename(
        self: Self,
        path: &str,
        attributes: Vec<&str>,
        transform: fn(&str) -> String,
    ) -> Self {
        let mut instance: Self = self;

        for attribute in attributes {
            instance = instance.field_attribute(
                format!("{}.{}", path, attribute),
                format!("#[serde(rename = \"{}\")]", transform(attribute)),
            )
        }

        instance
    }
}

fn main() -> Result<()> {
    if !Path::new("src/proto.rs").exists() {
        let marketstore_proto_dir = TempDir::new().unwrap();

        let mut marketstore_poto = reqwest::blocking::get(
            "https://raw.githubusercontent.com/alpacahq/marketstore/master/proto/marketstore.proto",
        )
        .expect("failed to downalod marketstore.proto");
        let marketstore_poto_local_path = marketstore_proto_dir.child("marketstore.proto");

        let mut marketstore_poto_file =
            File::create(&marketstore_poto_local_path.to_str().unwrap())
                .expect("failed to create file");
        io::copy(&mut marketstore_poto, &mut marketstore_poto_file)
            .expect("failed to copy content");

        fs::create_dir_all("src")?;

        tonic_build::configure()
            .build_server(false)
            .build_client(true)
            .out_dir(&"src")
            .compile(
                &[marketstore_poto_local_path],
                &[marketstore_proto_dir.path()],
            )?;
    }

    tonic_build::configure()
        .build_server(false)
        .build_client(false)
        .type_attribute(".", "#[derive(serde::Deserialize)]")
        .serda_rename(
            "Candle",
            vec!["open", "high", "low", "close", "volume", "epoch"],
            change_case::title_case,
        )
        .out_dir(&"examples/ohlcv/candle")
        .include_file("mod.rs")
        .compile(&["examples/candle.proto"], &["examples/"])?;

    tonic_build::configure()
        .build_server(false)
        .build_client(false)
        .type_attribute(".", "#[derive(serde::Deserialize)]")
        .serda_rename(
            "Candle",
            vec!["open", "high", "low", "close", "volume", "epoch"],
            change_case::title_case,
        )
        .out_dir(&"examples/stream/candle")
        .include_file("mod.rs")
        .compile(&["examples/candle.proto"], &["examples/"])
}
