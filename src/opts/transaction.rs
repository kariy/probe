use starknet::core::types::FieldElement;

#[cfg_attr(test, derive(clap::Parser))]
#[cfg_attr(not(test), derive(clap::Args))]
#[derive(Debug, Clone)]
pub struct TransactionOptions {
    #[arg(long)]
    pub nonce: Option<FieldElement>,

    #[arg(long)]
    #[arg(help = "The maximal fee that can be charged for including the transaction")]
    pub max_fee: Option<FieldElement>,

    #[arg(long)]
    #[arg(value_delimiter = ',')]
    #[arg(help = "The transaction signature")]
    pub signature: Option<Vec<FieldElement>>,

    #[arg(long)]
    #[arg(help = "Version of the transaction scheme")]
    pub version: Option<u64>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use clap::CommandFactory;
    use starknet::core::types::FieldElement;

    use super::TransactionOptions;

    #[test]
    fn parse_tx_options() {
        let cli = TransactionOptions::command().get_matches_from(&[
            "transaction_options",
            "--signature",
            "0x124142,0x3323234,0x12324131",
            "--nonce",
            "0x64",
            "--max-fee",
            "0x256",
        ]);

        assert_eq!(
            cli.get_many::<FieldElement>("signature")
                .unwrap()
                .map(|e| e.to_owned())
                .collect::<Vec<_>>(),
            vec![
                FieldElement::from_str("0x124142").unwrap(),
                FieldElement::from_str("0x3323234").unwrap(),
                FieldElement::from_str("0x12324131").unwrap(),
            ]
        );

        assert_eq!(
            cli.get_one::<FieldElement>("nonce").unwrap().to_owned(),
            FieldElement::from_str("0x64").unwrap(),
        );

        assert_eq!(
            cli.get_one::<FieldElement>("max_fee").unwrap().to_owned(),
            FieldElement::from_str("0x256").unwrap(),
        );
    }
}
