//
// Copyright 2021 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Oak Functions ABI test backend server.

use anyhow::{anyhow, Context};
use futures::future::FutureExt;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use log::info;
use maplit::hashmap;
use oak_functions_abitest_common::{TEST_STORAGE_GET, TEST_STORAGE_GET_RESPONSE};
use structopt::StructOpt;

#[derive(StructOpt, Clone)]
#[structopt(about = "Oak Functions ABI test backend server")]
pub struct Opt {
    #[structopt(
        long,
        help = "Address to listen on for the HTTP server.",
        default_value = "[::]:8081"
    )]
    http_listen_address: String,
}

async fn service(_request: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    info!("Received request");
    let response = test_utils::serialize_entries(hashmap! {
        TEST_STORAGE_GET.as_bytes().to_vec() => TEST_STORAGE_GET_RESPONSE.as_bytes().to_vec(),
    });
    Ok(Response::new(Body::from(response)))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let opt = Opt::from_args();

    let address = &opt
        .http_listen_address
        .parse()
        .context("Couldn't parse address")?;
    let make_service =
        make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(service)) });

    let server = Server::bind(&address).serve(make_service);
    info!(
        "Oak Functions ABI test backend server is listening on http://{}",
        &opt.http_listen_address
    );
    server
        .with_graceful_shutdown(tokio::signal::ctrl_c().map(|r| r.unwrap()))
        .await
        .map_err(|error| anyhow!("Couldn't run server: {:?}", error))?;

    Ok(())
}