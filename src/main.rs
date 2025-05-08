mod config;
mod storage;
mod event_listener;
mod types;
mod relayer;
mod validator;


#[tokio::main]
asyn fn main() -> anyhow::Result<()> {
    let config = load_config();
    let (tx, mut rx) = mpsc::channel(100); // channel buffer capacity set at 100, but can be change to suit need
    // tx is the sender. rx is the reciever 

    let signer = config.private_key.parse::<LocalWallet>()?.with_chain_id(137) // chain id of polygon is user, this can be configurable 
    let provider = Provider::<http>::try_from(config.dest_rpc_url.clone())?;
    let client = Arc::new(SignerMiddleware::new(provider, signer));

    // listen and send ERC20 Lock Token Event 
    tokio::spawn(event_listener::listen_for_events(&config, tx));

    while let Some(event) = rx.recv().await {
        if let Err(err) = relayer::relay(event, &config, client.clone()).await {
            eprintln!("Relay error: {:?}", err); // there could be a better way to handle this
        }
    }

    Ok(())
}
