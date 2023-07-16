//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use async_std::task::sleep;
use clap::Parser;
use futures::prelude::*;
use futures::select;
use std::convert::TryFrom;
use std::time::Duration;
use zenoh::config::Config;
use zenoh::prelude::r#async::*;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// The key expression to publish onto.
    #[clap(short, long)]
    key: String,

    /// The zenoh session mode (peer by default).
    #[clap(short, long)]
    mode: Option<zenoh::scouting::WhatAmI>,

    /// Endpoints to connect to.
    #[clap(short = 'e', long)]
    connect: Vec<zenoh_config::EndPoint>,

    /// Endpoints to listen on.
    #[clap(long)]
    listen: Vec<zenoh_config::EndPoint>,

    /// A configuration file.
    #[clap(short, long)]
    config: Option<String>,

    /// Disable the multicast-based scouting mechanism.
    #[clap(long)]
    no_multicast_scouting: bool,
}

#[async_std::main]
async fn main() {
    // Initiate logging
    env_logger::init();

    let (config, key_expr) = parse_args();

    println!("Opening session...");
    let session = zenoh::open(config).res().await.unwrap();

    println!("Declaring Subscriber on '{}'...", &key_expr);

    let subscriber = session.declare_subscriber(&key_expr).res().await.unwrap();

    println!("Enter 'q' to quit...");
    let mut stdin = async_std::io::stdin();
    let mut input = [0_u8];
    loop {
        select!(
            sample = subscriber.recv_async() => {
                let sample = sample.unwrap();
                println!("{} ('{}': '{}')",
                    sample.kind, sample.key_expr.as_str(), sample.value);
            },

            _ = stdin.read_exact(&mut input).fuse() => {
                match input[0] {
                    b'q' => break,
                    0 => sleep(Duration::from_secs(1)).await,
                    _ => (),
                }
            }
        );
    }
}

fn parse_args() -> (Config, KeyExpr<'static>) {
    let args: Args = Args::parse();

    let mut config = if let Some(conf_file) = &args.config {
        Config::from_file(conf_file).unwrap()
    } else {
        Config::default()
    };
    if let Some(mode) = args.mode {
        config.set_mode(Some(mode)).unwrap();
    }
    if !args.connect.is_empty() {
        config.connect.endpoints = args.connect.clone();
    }
    if !args.listen.is_empty() {
        config.listen.endpoints = args.listen.clone();
    }
    if args.no_multicast_scouting {
        config.scouting.multicast.set_enabled(Some(false)).unwrap();
    }

    let key_expr = KeyExpr::try_from(&args.key).unwrap().into_owned();

    (config, key_expr)
}
