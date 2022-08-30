use tokio::task;
use super::configs::InteractionsConfigs;

mod grpc;

pub fn start_interactions(interctions_configs: InteractionsConfigs) {
    info!("Starting interactions");
    if let Some(grpc) = interctions_configs.grpc {
        task::spawn(async {
            info!("iniciando gRPC");
            grpc::grpc_init(grpc.bind_addr.unwrap_or("127.0.0.1:3127".to_owned())).await;
        });
    }
}
