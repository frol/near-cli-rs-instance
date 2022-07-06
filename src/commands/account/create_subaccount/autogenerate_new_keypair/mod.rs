use std::str::FromStr;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod print_keypair_to_terminal;
mod save_keypair_to_keychain;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
pub struct GenerateKeypair {
    #[interactive_clap(subcommand)]
    save_mode: SaveMode,
}

impl GenerateKeypair {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        self.save_mode
            .process(prepopulated_unsigned_transaction)
            .await
    }
}

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Add a full access key for the sub-account
pub enum SaveMode {
    #[strum_discriminants(strum(message = "Save automatically generated key pair to keychain"))]
    ///Save automatically generated key pair to keychain
    SaveToKeychain(self::save_keypair_to_keychain::SaveKeypairToKeychain),
    #[strum_discriminants(strum(message = "Print automatically generated key pair in terminal"))]
    ///Print automatically generated key pair in terminal
    PrintToTerminal(self::print_keypair_to_terminal::PrintKeypairToTerminal),
}

impl SaveMode {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let key_pair_properties: crate::common::KeyPairProperties =
            crate::common::generate_keypair().await?;
        let access_key: near_primitives::account::AccessKey = near_primitives::account::AccessKey {
            nonce: 0,
            permission: near_primitives::account::AccessKeyPermission::FullAccess,
        };
        let action = near_primitives::transaction::Action::AddKey(
            near_primitives::transaction::AddKeyAction {
                public_key: near_crypto::PublicKey::from_str(&key_pair_properties.public_key_str)?,
                access_key,
            },
        );
        let mut actions = prepopulated_unsigned_transaction.actions.clone();
        actions.push(action);
        let prepopulated_unsigned_transaction = near_primitives::transaction::Transaction {
            actions,
            ..prepopulated_unsigned_transaction
        };
        match self {
            SaveMode::SaveToKeychain(save_keypair_to_keychain) => {
                save_keypair_to_keychain
                    .process(key_pair_properties, prepopulated_unsigned_transaction)
                    .await
            }
            SaveMode::PrintToTerminal(print_keypair_to_terminal) => {
                print_keypair_to_terminal
                    .process(key_pair_properties, prepopulated_unsigned_transaction)
                    .await
            }
        }
    }
}