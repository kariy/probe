pub mod utils;

use self::utils::get_from_keystore;
use super::starknet::StarknetChain;
use crate::cmd::account::simple_account::SimpleAccount;

use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use eyre::Result;
use inquire::{CustomType, Select};
use starknet::core::types::FieldElement;

#[derive(Debug, Clone, Parser, Default)]
pub struct WalletOptions {
    #[clap(short, long)]
    #[clap(exclusive = true)]
    #[clap(help_heading = "WALLET OPTIONS - RAW")]
    #[clap(help = "Open an interactive prompt to enter your wallet details.")]
    pub interactive: bool,

    #[clap(long)]
    #[clap(value_name = "PRIVATE_KEY")]
    #[clap(help_heading = "WALLET OPTIONS - RAW")]
    #[clap(help = "The raw private key associated with the account contract.")]
    pub private_key: Option<FieldElement>,

    #[clap(long)]
    #[clap(value_name = "ACCOUNT_ADDRESS")]
    #[clap(help_heading = "WALLET OPTIONS - RAW")]
    #[clap(help = "Account contract to initiate the transaction from.")]
    pub from: Option<FieldElement>,

    #[clap(long = "keystore")]
    #[clap(value_name = "PATH")]
    #[clap(env = "STARKNET_KEYSTORE")]
    #[clap(help_heading = "WALLET OPTIONS - KEYSTORE")]
    #[clap(help = "Use the keystore in the given folder or file.")]
    pub keystore_path: Option<PathBuf>,

    #[clap(long = "password")]
    #[clap(value_name = "PASSWORD")]
    #[clap(requires = "keystore")]
    #[clap(help_heading = "WALLET OPTIONS - KEYSTORE")]
    #[clap(help = "The keystore password. Used with --keystore.")]
    pub keystore_password: Option<String>,

    #[clap(env = "STARKNET_KEYSTORE_PASSWORD")]
    #[clap(long = "password-file")]
    #[clap(requires = "keystore")]
    #[clap(value_name = "PASSWORD_FILE")]
    #[clap(help_heading = "WALLET OPTIONS - KEYSTORE")]
    #[clap(help = "The keystore password file path. Used with --keystore.")]
    pub keystore_password_file: Option<PathBuf>,
}

impl WalletOptions {
    pub fn interactive(&self) -> Result<Option<SimpleAccount>> {
        Ok(if self.interactive {
            let felt_prompter = |message: &'static str| {
                CustomType::new(message)
                    .with_parser(&|input| FieldElement::from_str(input).map_err(|_| ()))
                    .with_error_message("Invalid field element value.")
                    .with_help_message(
                        "Value must be smaller than 251 bits. Can be a hex or decimal number.",
                    )
            };

            let account = felt_prompter("Enter account address : ").prompt()?;
            let private_key = felt_prompter("Enter private key : ").prompt()?;

            let options = vec!["mainnet", "testnet", "testnet2"];
            let chain =
                Select::new("Please select the chain for this account.", options).prompt()?;

            Some(SimpleAccount::new(
                account,
                private_key,
                StarknetChain::from_str(chain).ok(),
            ))
        } else {
            None
        })
    }

    #[allow(unused)]
    pub fn raw(&self) -> Option<SimpleAccount> {
        match (self.from, self.private_key) {
            (Some(from), Some(pk)) => Some(SimpleAccount::new(from, pk, None)),
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn keystore(&self) -> Result<Option<SimpleAccount>> {
        get_from_keystore(
            self.from.unwrap().to_string().as_ref(),
            self.keystore_path.as_ref(),
            self.keystore_password.as_ref(),
            self.keystore_password_file.as_ref(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn account_from_keystore() {
        let account_addr = FieldElement::from_hex_be(
            "0x148A764E88277F972B6E1517A60CD6EF5FC11FF3DBC686EA932451552D0641B",
        )
        .unwrap();

        let file = Path::new("./tests/test-keys/test-key1.json");
        let opts = WalletOptions {
            from: Some(account_addr),
            keystore_path: Some(file.to_path_buf()),
            keystore_password: Some("12345".to_string()),
            ..Default::default()
        };

        let wallet = opts.keystore().unwrap().unwrap();

        assert_eq!(
            wallet.account,
            FieldElement::from_hex_be(
                "0x148A764E88277F972B6E1517A60CD6EF5FC11FF3DBC686EA932451552D0641B"
            )
            .unwrap()
        );
        assert_eq!(
            wallet.get_signing_key(),
            FieldElement::from_hex_be(
                "0x1a2e71241e4c65739c87717d99101e8ea9523126c6ad9e67f9cae703ba3dacf"
            )
            .unwrap()
        );
        assert_eq!(wallet.chain.unwrap().to_string(), "mainnet");
    }

    #[test]
    fn account_from_keystore_and_password_file() {
        let account_addr = FieldElement::from_hex_be(
            "0x148A764E88277F972B6E1517A60CD6EF5FC11FF3DBC686EA932451552D0641B",
        )
        .unwrap();

        let file = Path::new("./tests/test-keys/test-key1.json");
        let password_file = Path::new("./tests/test-keys/password1");

        let opts = WalletOptions {
            from: Some(account_addr),
            keystore_path: Some(file.to_path_buf()),
            keystore_password_file: Some(password_file.to_path_buf()),
            ..Default::default()
        };

        let wallet = opts.keystore().unwrap().unwrap();

        assert_eq!(
            wallet.account,
            FieldElement::from_hex_be(
                "0x148A764E88277F972B6E1517A60CD6EF5FC11FF3DBC686EA932451552D0641B"
            )
            .unwrap()
        );
        assert_eq!(
            wallet.get_signing_key(),
            FieldElement::from_hex_be(
                "0x1a2e71241e4c65739c87717d99101e8ea9523126c6ad9e67f9cae703ba3dacf"
            )
            .unwrap()
        );
        assert_eq!(wallet.chain.unwrap().to_string(), "mainnet");
    }

    #[test]
    fn account_from_raw() {
        let from = FieldElement::from_hex_be("").unwrap();
        let private_key = FieldElement::from_hex_be("").unwrap();

        let opts = WalletOptions {
            from: Some(from),
            private_key: Some(private_key),
            ..Default::default()
        };

        let wallet = opts.raw().unwrap();

        assert!(wallet.chain.is_none());
        assert_eq!(wallet.account, from);
        assert_eq!(wallet.get_signing_key(), private_key);
    }
}
