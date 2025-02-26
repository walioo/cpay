use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use std::mem::size_of;
use crate::error::ErrorCode;

// 初始化指令的处理函数
pub(crate) fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    require!(!global_state.initialized, ErrorCode::RepeatedInit);
    global_state.initialized = true;
    global_state.admin = ctx.accounts.payer.key();
    global_state.token_account = ctx.accounts.program_token_account.key();
    global_state.total_funds = 0;
    global_state.staked_funds = 0;
    global_state.idle_funds = 0;
    Ok(())
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
    init,
    seeds = ["global_state".as_bytes()],
    bump, payer = payer, space = size_of::<GlobalState>()+8)
    ]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        init,
        seeds = [b"program_token_account".as_ref(), global_state.key().as_ref()],
        bump,
        payer = payer,
        token::mint = mint,
        token::authority = global_state
    )]
    pub program_token_account: Box<Account<'info, TokenAccount>>,

    pub mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

}


#[account]
pub struct GlobalState {
    pub initialized: bool,
    pub admin: Pubkey,
    pub token_account: Pubkey,

    pub total_funds: u64,

    pub staked_funds: u64,

    pub idle_funds: u64,

}

impl GlobalState {
    // 账户大小常量
    pub const LEN: usize = 32 + // admin pubkey
        32 + // token_account pubkey
        8 +  // total_funds
        8 +  // staked_funds
        8 +  // idle_funds
        1;   // bump
}