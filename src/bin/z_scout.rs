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
use async_std::prelude::FutureExt;
use clap::Parser;
use zenoh::config::Config;
use zenoh::prelude::r#async::*;
use zenoh::scouting::WhatAmI;

#[derive(Parser, Debug)]
#[command()]
struct Args {
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

    /// The time for the scouting phase. (milliseconds)
    #[clap(long, default_value = "1000")]
    scout_time_ms: u64,
}

fn parse_args() -> (Args, Config) {
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

    (args, config)
}

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    let (args, config) = parse_args();

    println!("Scouting...");
    let receiver = zenoh::scout(WhatAmI::Peer | WhatAmI::Router, config)
        .res()
        .await
        .unwrap();

    let _ = async {
        while let Ok(hello) = receiver.recv_async().await {
            let mut message = format!(
                "Hello from {:?} with id {:?}\n",
                hello.whatami,
                hello
                    .zid
                    .map(|id| id.to_string())
                    .unwrap_or(String::from("N/A"))
            );
            message.push_str("  Locators:\n");
            for locator in hello.locators {
                message.push_str(&format!("  - {}\n", locator));
            }
            println!("{}", message);
        }
    }
    .timeout(std::time::Duration::from_millis(args.scout_time_ms))
    .await;

    // stop scouting
    drop(receiver);
}
