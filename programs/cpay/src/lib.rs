use anchor_lang::prelude::*;

pub mod circle;
pub mod error;
pub mod instructions;
pub mod state;

use circle::*;
use instructions::*;

declare_id!("FNDpC3ADhBZSAhwHbh8ex9sGfGWQWBeDzWVcE1hhQehL");

#[program]
pub mod cpay {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        instructions::withdraw(ctx)
    }

    pub fn deposit_for_burn(
        ctx: Context<DepositForBurn>,
        amount: u64,
        destination_domain: u32,
        mint_recipient: [u8; 32],
    ) -> Result<()> {
        circle::deposit_for_burn(ctx, amount, destination_domain, mint_recipient)
    }

    pub fn receive_message(
        ctx: Context<ReceiveMessage>,
        message: MessageData,
        signatures: Vec<[u8; 65]>,
    ) -> Result<()> {
        circle::receive_message(ctx, message, signatures)
    }
}
