use std::{str::FromStr, sync::Arc, time::Duration};
use clap::{Arg, ArgAction, Command};

use kaspa_consensus_core::{constants::MAX_SOMPI, subnets::SubnetworkId, tx::Transaction};
use kaspa_core::{info, kaspad_env::version, time::unix_now, warn};
use kaspa_grpc_client::{ClientPool, GrpcClient};
use kaspa_grpc_core::ops::KaspadPayloadOps;
use kaspa_hashes::Hash;
use kaspa_notify::{
    connection::{ChannelConnection, ChannelType},
    scope::{
        BlockAddedScope, FinalityConflictScope, NewBlockTemplateScope, PruningPointUtxoSetOverrideScope, Scope,
        SinkBlueScoreChangedScope, UtxosChangedScope, VirtualChainChangedScope, VirtualDaaScoreChangedScope,
    },
};
// use kaspa_notify::subscription::context::SubscriptionContext;

use kaspa_rpc_core::{api::rpc::RpcApi, model::*, Notification};
use tokio::time::{interval, MissedTickBehavior};


pub struct Args {
    pub rpc_server: String,
}

impl Args {
    fn parse() -> Self {
        let m = cli().get_matches();
        Args {
            rpc_server: m.get_one::<String>("rpcserver").cloned().unwrap_or("localhost:16210".to_owned()),
        }
    }
}

pub fn cli() -> Command {
    Command::new("sponge")
        .about(format!("{} (sponge) v{}", env!("CARGO_PKG_DESCRIPTION"), version()))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("rpcserver")
                .long("rpcserver")
                .short('s')
                .value_name("rpcserver")
                .default_value("localhost:16210")
                .help("RPC server"),
        )
}

async fn new_rpc_client(subscription_context: &SubscriptionContext, address: &str) -> GrpcClient {
    GrpcClient::connect_with_args(
        NotificationMode::Direct,
        format!("grpc://{}", address),
        Some(subscription_context.clone()),
        true,
        None,
        false,
        Some(500_000),
        Default::default(),
    )
    .await
    .unwrap()
}
fn main() {
    println!("Hello, world!");
}
