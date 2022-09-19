use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum SwapBankIntruction {
    Initialize {},
    Swap { amount: f64 },
}
