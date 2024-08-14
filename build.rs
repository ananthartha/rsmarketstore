use temp_dir::TempDir;
extern crate reqwest;

use std::fs;
use std::io;
use std::io::Result;
use std::path::Path;

use quote::quote;
use syn::visit_mut::{self, VisitMut};
use syn::{parse_quote, Expr, File, Lit, LitInt};

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

struct ProstBehindFeature;

impl VisitMut for ProstBehindFeature {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Lit(expr) = &node {
            if let Lit::Int(int) = &expr.lit {
                if int.suffix() == "u256" {
                    let digits = int.base10_digits();
                    let unsuffixed: LitInt = syn::parse_str(digits).unwrap();
                    *node = parse_quote!(bigint::u256!(#unsuffixed));
                    return;
                }
            }
        }

        // Delegate to the default impl to visit nested expressions.
        visit_mut::visit_expr_mut(self, node);
    }

    // #[cfg(not(target_arch = "wasm32"))]
    // visit_item_mod_mut

    // #[cfg_attr(not(target_arch = "wasm32"), derive(::prost::Message))]
    // visit_meta_mut
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
            fs::File::create(&marketstore_poto_local_path.to_str().unwrap())
                .expect("failed to create file");
        io::copy(&mut marketstore_poto, &mut marketstore_poto_file)
            .expect("failed to copy content");

        fs::create_dir_all("src")?;

        let mut prost_config = prost_build::Config::new();
        prost_config.prost_path("crate::prelude::prost");

        tonic_build::configure()
            .build_server(false)
            .build_client(true)
            .out_dir(&"src")
            .compile_with_config(
                prost_config,
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
