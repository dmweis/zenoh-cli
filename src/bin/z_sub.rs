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
use zenoh_cli::CommonArgs;

#[tokio::main]
async fn main() {
    // Initiate logging
    zenoh_util::try_init_log_from_env();

    let (mut config, key_expr, raw) = parse_args();

    // A probing procedure for shared memory is performed upon session opening. To enable `z_pub_shm` to operate
    // over shared memory (and to not fallback on network mode), shared memory needs to be enabled also on the
    // subscriber side. By doing so, the probing procedure will succeed and shared memory will operate as expected.
    config.transport.shared_memory.set_enabled(true).unwrap();

    if !raw {
        println!("Opening session...");
    }
    let session = zenoh::open(config).res().await.unwrap();

    if !raw {
        println!("Declaring Subscriber on '{}'...", &key_expr);
    }

    let subscriber = session.declare_subscriber(&key_expr).res().await.unwrap();

    if !raw {
        println!("Press CTRL-C to quit...");
    }
    while let Ok(sample) = subscriber.recv_async().await {
        if raw {
            println!("{}", sample.value);
        } else {
            println!(
                ">> [Subscriber] Received {} ('{}': '{}')",
                sample.kind,
                sample.key_expr.as_str(),
                sample.value
            );
        }
    }
}

#[derive(clap::Parser, Clone, PartialEq, Eq, Hash, Debug)]
struct SubArgs {
    #[arg(short, long, default_value = "demo/example/**")]
    /// The Key Expression to subscribe to.
    key: KeyExpr<'static>,
    #[command(flatten)]
    common: CommonArgs,

    /// print the output only
    #[clap(long)]
    raw: bool,
}

fn parse_args() -> (Config, KeyExpr<'static>, bool) {
    let args = SubArgs::parse();
    (args.common.into(), args.key, args.raw)
}
