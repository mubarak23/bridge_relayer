use ethers::prelude::*;
use crate::types::TokenLockedEvent;
use crate::config::Config;
use crate::validator::is_signature_valid;
use crate::storage::{is_processed, mark_processed};

abigen!(
    DestinationBridge,
    r#"[
        function mint(address recipient, uint256 amount)
    ]"#
);


pub async fn relay(
  event: TokenLockedEvent,
  config: &crate::config::Config,
  client: Arc<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>
) -> anyhow::Result<()> {
  let key = format!("{:?}:{:?}", event.sender, event.amount);
  if is_processed(key){
    return Ok(())
  }

  if !is_signature_valid(&event) {
    anyhow::bail!("Invalid Signature")
  }
  let contract = DestinationBridge::new(config.dest_contract.parse()?, client);
  let tx = contract.mint(event.recipient, event.amount).send().await?
  let _receipt = tx.await?;
  mark_processed(&key, &event);

  Ok(())

}
