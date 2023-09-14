extern crate proc_macro;
use std::collections::{HashMap, HashSet};

use proc_macro::{token_stream, TokenStream, TokenTree};

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    let mut result = r#"
    pub mod @@package_name@@_grpc {
        tonic::include_proto!("@@package_name@@");
    }
    
    pub mod @@package_name@@_grpc_client {
        #[my_grpc_extensions::client::generate_grpc_client(
            proto_file: "./proto-files/@@file_name@@.proto",
            crate_ns: "crate::@@package_name@@_grpc",
            retries: 3,
            request_timeout_sec: 1,
            ping_timeout_sec: 1,
            ping_interval_sec: 3,
        )]
        pub struct @@client_name@@ {
            channel: my_grpc_extensions::GrpcChannel<TGrpcService>,
        }
    }"#
    .to_string();

    let key_value = _item
        .to_string()
        .split(",")
        .map(|x| {
            let res = x.split("=").collect::<Vec<&str>>();

            (res[0].trim().to_string(), res[1].trim().replace("\"", ""))
        })
        .collect::<Vec<(String, String)>>();

    for (key, value) in key_value {
        result = result.replace(&format!("@@{}@@", key), &value);
    }

    result.parse().unwrap()
}
