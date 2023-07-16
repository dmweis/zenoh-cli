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
use clap::Parser;
use zenoh::config::Config;
use zenoh::prelude::r#async::*;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// The key expression to publish onto.
    #[clap(short, long)]
    key: String,

    /// The value to publish.
    #[clap(short, long)]
    value: String,

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
    // initiate logging
    env_logger::init();

    let (_args, config, key_expr, value) = parse_args();

    println!("Opening session...");
    let session = zenoh::open(config).res().await.unwrap();

    println!("Putting Data ('{key_expr}': '{value}')...");
    session.put(&key_expr, value).res().await.unwrap();
}

fn parse_args() -> (Args, Config, String, String) {
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

    let key_expr = args.key.clone();
    let value = args.value.clone();

    (args, config, key_expr, value)
}
