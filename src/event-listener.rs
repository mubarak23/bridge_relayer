use ether::prelude::*;
use crate::types::TokenLockedEvent;
use crate::config::Config;


abigen!(
    BridgeContract,
    r#"[
        event TokenLocked(address indexed sender, uint256 amount, string destinationChain, address recipient)
    ]"#
);


pub async fn listener_for_events(config: &Config, tx: tokio::sync::mpsc::Sender<TokenLockedEvent>) -> anyhow::Result<()> {
  let client = Provider::<Ws>::connect(&config.eth_ws_url).await?;
  let contract = BridgeContract::new(config.bridge_contract.parse()?, client);
  let mut stream = contract.token_locked_filter().stream().await?;

  while let Some(Ok(event)) = stream.next().await {
    let data = TokenLockedEvent {
      sender: event.sender,
      amount: event.amount,
      recipient: event.recipient
    };
    tx.send(data).await?;
  }
  Ok(())
}