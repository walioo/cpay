use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::instructions;
use crate::state::*;
use drift_vaults::state::{Vault, VaultDepositor};
use drift::program::Drift;
use drift::state::user::User;
use crate::error::ErrorCode;

pub(crate) fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(ctx.accounts.global_state.admin.key().eq(ctx.accounts.payer.key), ErrorCode::UnauthorizedAdmin);

    // 验证闲置资金是否足够
    require!(
        ctx.accounts.fund_account.idle_amount >= amount,
        ErrorCode::InsufficientIdleFunds
    );

    // 构造deposit指令所需的账户
    let deposit_accounts = drift_vaults::cpi::accounts::Deposit {
        vault: ctx.accounts.vault.to_account_info(),
        vault_depositor: ctx.accounts.vault_depositor.to_account_info(),
        authority: ctx.accounts.global_state.to_account_info(),
        vault_token_account: ctx.accounts.vault_token_account.to_account_info(),
        drift_user_stats: ctx.accounts.drift_user_stats.to_account_info(),
        drift_user: ctx.accounts.drift_user.to_account_info(),
        drift_state: ctx.accounts.drift_state.to_account_info(),
        drift_spot_market_vault: ctx.accounts.drift_spot_market_vault.to_account_info(),
        user_token_account: ctx.accounts.program_token_account.to_account_info(),
        drift_program: ctx.accounts.drift_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    };

    // 调用drift_vaults的deposit指令
    drift_vaults::cpi::deposit(CpiContext::new_with_signer(
        ctx.accounts.drift_program.to_account_info(),
        deposit_accounts,
        &[
            &[
                "program_token_account".as_bytes(),
                &[ctx.bumps.global_state]
            ],
        ]
        ),
        amount,
    )?;

    // 更新资金账户状态
    let fund_account = &mut ctx.accounts.fund_account;
    fund_account.idle_amount = fund_account.idle_amount.checked_sub(amount)
        .ok_or(ErrorCode::MathOverflow)?;
    fund_account.staked_amount = fund_account.staked_amount.checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;
    fund_account.last_update_time = Clock::get()?.unix_timestamp;

    // 发出质押状态变更事件
    emit!(StakeStatusChanged {
        amount,
        is_stake: true,
        stake_account: ctx.accounts.vault_depositor.key(),
        timestamp: fund_account.last_update_time,
    });

    // 发出资金状态变更事件
    emit!(FundStatusChanged {
        amount,
        from_status: FundStatus::Idle,   // 从闲置状态
        to_status: FundStatus::Staked,   // 变为质押状态
        token_account: ctx.accounts.program_token_account.key(),
        timestamp: fund_account.last_update_time,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// 全局状态账户，存储管理员地址等信息
    #[account(mut,seeds = ["global_state".as_bytes()],bump)]
    pub global_state: Box<Account<'info, instructions::GlobalState>>,

    /// 资金账户，用于跟踪资金状态（闲置、质押等）
    #[account(
        mut,
        seeds = [b"fund_account", program_token_account.key().as_ref()],
        bump,
    )]
    pub fund_account: Account<'info, FundAccount>,

    /// USDC代币铸造账户
    pub mint: Box<Account<'info, Mint>>,

    /// Drift金库账户，用于管理质押资金
    #[account(mut)]
    pub vault: AccountLoader<'info, Vault>,

    /// 程序的代币账户，用于存储用户存入的代币
    #[account(
        mut,
        token::authority = global_state,
    )]
    pub program_token_account: Box<Account<'info, TokenAccount>>,

    /// 金库的代币账户，用于接收质押的代币
    #[account(mut)]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,

    /// 金库存款人账户，记录质押份额等信息
    #[account(mut)]
    pub vault_depositor: AccountLoader<'info, VaultDepositor>,

    /// Drift用户统计账户  CHECK: drift program
    #[account(mut)]
    pub drift_user_stats: AccountInfo<'info>,

    /// Drift用户账户 CHECK: drift program
    #[account(mut)]
    pub drift_user: AccountLoader<'info, User>,

    /// Drift状态账户 CHECK: drift program
    #[account(mut)]
    pub drift_state: AccountInfo<'info>,

    /// Drift现货市场金库账户 CHECK: drift program
    #[account(mut)]
    pub drift_spot_market_vault: AccountInfo<'info>,

    /// Drift程序ID
    pub drift_program: Program<'info, Drift>,

    /// Token程序ID
    pub token_program: Program<'info, Token>,

    /// 交易发起人，必须是管理员
    #[account(mut)]
    pub payer: Signer<'info>,
}
