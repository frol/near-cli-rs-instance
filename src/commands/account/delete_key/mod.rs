#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct DeleteKeyCommand {
    ///Which account should You delete the access key for?
    owner_account_id: crate::types::account_id::AccountId,
    ///Enter the public key You wish to delete
    pub public_key: crate::types::public_key::PublicKey,
    #[interactive_clap(named_arg)]
    ///Select online mode
    network: crate::network_for_transaction::NetworkForTransactionArgs,
}

impl DeleteKeyCommand {
    pub async fn process(&self) -> crate::CliResult {
        let prepopulated_unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: self.owner_account_id.clone().into(),
            public_key: near_crypto::PublicKey::empty(near_crypto::KeyType::ED25519),
            nonce: 0,
            receiver_id: self.owner_account_id.clone().into(),
            block_hash: Default::default(),
            actions: vec![near_primitives::transaction::Action::DeleteKey(
                near_primitives::transaction::DeleteKeyAction {
                    public_key: self.public_key.clone().into(),
                },
            )],
        };

        match self.network.get_sign_option() {
            crate::transaction_signature_options::SignWith::SignWithPlaintextPrivateKey(
                sign_private_key,
            ) => {
                sign_private_key
                    .process(
                        prepopulated_unsigned_transaction,
                        self.network.get_connection_config(),
                    )
                    .await
            }
            crate::transaction_signature_options::SignWith::SignWithKeychain(sign_keychain) => {
                sign_keychain
                    .process(
                        prepopulated_unsigned_transaction,
                        self.network.get_connection_config(),
                    )
                    .await
            }
            crate::transaction_signature_options::SignWith::SignWithLedger(sign_ledger) => {
                sign_ledger
                    .process(
                        prepopulated_unsigned_transaction,
                        self.network.get_connection_config(),
                    )
                    .await
            }
        }
    }
}