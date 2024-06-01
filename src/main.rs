// use alloy_provider::ProviderBuilder;
// use foundry_evm::fork::SharedBackend;

// #[tokio::main]
// async fn main() {
//     let ws = WsConnect::new(conf::wss());
//     let provider = ProviderBuilder::new()
//         .on_ws(ws)
//         .await
//         .expect("Failed to create provider");

//     let backend = SharedBackend::spawn_backend_thread(
//         client.clone(),
//         BlockchainDb::new(
//             BlockchainDbMeta {
//                 cfg_env: Default::default(),
//                 block_env: Default::default(),
//                 hosts: BTreeSet::from(["".to_string()]),
//             },
//             None,
//         ),
//         Some((target_block.number.unwrap_or(U64::one()) - 1).into()),
//     );
// }
