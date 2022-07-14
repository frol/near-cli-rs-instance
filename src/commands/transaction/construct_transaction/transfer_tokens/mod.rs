use strum::{EnumDiscriminants, EnumIter, EnumMessage};

// mod send_ft;
mod send_near;
// mod send_nft;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct TokensCommands {
    // ///What is your account ID?
    // owner_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(subcommand)]
    tokens_actions: TokensActions,
}

impl TokensCommands {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        self.tokens_actions
            .process(prepopulated_unsigned_transaction)
            .await
    }
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Select actions with tokens
pub enum TokensActions {
    #[strum_discriminants(strum(message = "The transfer is carried out in NEAR tokens"))]
    ///The transfer is carried out in NEAR tokens
    SendNear(self::send_near::SendNearCommand),
    #[strum_discriminants(strum(message = "The transfer is carried out in FT tokens"))]
    ///The transfer is carried out in FT tokens
    SendFt, //(self::send_ft::SendFtCommand),
    #[strum_discriminants(strum(message = "The transfer is carried out in NFT tokens"))]
    ///The transfer is carried out in NFT tokens
    SendNft, //(self::send_nft::SendNFtCommand),
}

impl TokensActions {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self {
            Self::SendNear(send_near_command) => {
                send_near_command
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
            // Self::SendFt(send_ft_command) => send_ft_command.process(owner_account_id).await,
            // Self::SendNft(send_nft_command) => send_nft_command.process(owner_account_id).await,
            _ => todo!(),
        }
    }
}
