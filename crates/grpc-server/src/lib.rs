use {
    anyhow::Context,
    cnidarium::rpc::{
        proto::v1::query_service_server::QueryServiceServer as StorageQueryServiceServer,
        Server as StorageServer,
    },
    ibc_proto::ibc::core::{
        channel::v1::query_server::QueryServer as ChannelQueryServer,
        client::v1::query_server::QueryServer as ClientQueryServer,
        connection::v1::query_server::QueryServer as ConnectionQueryServer,
    },
    penumbra_app::{rpc::Server as AppServer, PenumbraHost},
    penumbra_compact_block::component::rpc::Server as CompactBlockServer,
    penumbra_fee::component::rpc::Server as FeeServer,
    penumbra_governance::component::rpc::Server as GovernanceServer,
    penumbra_proto::{
        core::{
            app::v1::query_service_server::QueryServiceServer as AppQueryServiceServer,
            component::{
                compact_block::v1::query_service_server::QueryServiceServer as CompactBlockQueryServiceServer,
                fee::v1::query_service_server::QueryServiceServer as FeeQueryServiceServer,
                governance::v1::query_service_server::QueryServiceServer as GovernanceQueryServiceServer,
                sct::v1::query_service_server::QueryServiceServer as SctQueryServiceServer,
                shielded_pool::v1::query_service_server::QueryServiceServer as ShieldedPoolQueryServiceServer,
                stake::v1::query_service_server::QueryServiceServer as StakeQueryServiceServer,
            },
        },
        util::tendermint_proxy::v1::tendermint_proxy_service_server::TendermintProxyServiceServer,
    },
    penumbra_sct::component::rpc::Server as SctServer,
    penumbra_shielded_pool::component::rpc::Server as ShieldedPoolServer,
    penumbra_stake::component::rpc::Server as StakeServer,
    penumbra_tendermint_proxy::TendermintProxy,
    penumbra_tower_trace::remote_addr,
    tonic::transport::Server,
    tonic_web::enable as we,
};

pub fn new(
    storage: &cnidarium::Storage,
    tm_proxy: &TendermintProxy,
) -> Result<tonic::transport::server::Router, anyhow::Error> {
    // TODO: Once we migrate to Tonic 0.10.0, we'll be able to use the
    // `Routes` structure to have each component define a method that
    // returns a `Routes` with all of its query services bundled inside.
    //
    // This means we won't have to import all this shit and recite every
    // single service -- we can e.g., have the app crate assemble all of
    // its components' query services into a single `Routes` and then
    // just add that to the gRPC server.

    let ibc = penumbra_ibc::component::rpc::IbcQuery::<PenumbraHost>::new(storage.clone());
    let grpc_server = Server::builder()
        .trace_fn(|req| match remote_addr(req) {
            Some(remote_addr) => {
                tracing::error_span!("grpc", ?remote_addr)
            }
            None => tracing::error_span!("grpc"),
        })
        // Allow HTTP/1, which will be used by grpc-web connections.
        // This is particularly important when running locally, as gRPC
        // typically uses HTTP/2, which requires HTTPS. Accepting HTTP/2
        // allows local applications such as web browsers to talk to pd.
        .accept_http1(true)
        // As part of #2932, we are disabling all timeouts until we circle back to our
        // performance story.
        // Sets a timeout for all gRPC requests, but note that in the case of streaming
        // requests, the timeout is only applied to the initial request. This means that
        // this does not prevent long lived streams, for example to allow clients to obtain
        // new blocks.
        // .timeout(std::time::Duration::from_secs(7))
        // Wrap each of the gRPC services in a tonic-web proxy:
        .add_service(we(StorageQueryServiceServer::new(StorageServer::new(
            storage.clone(),
        ))))
        .add_service(we(AppQueryServiceServer::new(AppServer::new(
            storage.clone(),
        ))))
        .add_service(we(CompactBlockQueryServiceServer::new(
            CompactBlockServer::new(storage.clone()),
        )))
        .add_service(we(FeeQueryServiceServer::new(FeeServer::new(
            storage.clone(),
        ))))
        .add_service(we(GovernanceQueryServiceServer::new(
            GovernanceServer::new(storage.clone()),
        )))
        .add_service(we(SctQueryServiceServer::new(SctServer::new(
            storage.clone(),
        ))))
        .add_service(we(ShieldedPoolQueryServiceServer::new(
            ShieldedPoolServer::new(storage.clone()),
        )))
        .add_service(we(StakeQueryServiceServer::new(StakeServer::new(
            storage.clone(),
        ))))
        .add_service(we(ClientQueryServer::new(ibc.clone())))
        .add_service(we(ChannelQueryServer::new(ibc.clone())))
        .add_service(we(ConnectionQueryServer::new(ibc.clone())))
        .add_service(we(TendermintProxyServiceServer::new(tm_proxy.clone())))
        .add_service(we(tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(penumbra_proto::FILE_DESCRIPTOR_SET)
            .build()
            .with_context(|| "could not configure grpc reflection service")?));

    Ok(grpc_server)
}