use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::instructions;
use crate::state::*;
use drift_vaults::state::{Vault, VaultDepositor};
use drift::program::Drift;
use drift::state::user::User;
use crate::error::ErrorCode;
use drift_vaults::state::traits::VaultDepositorBase;

pub(crate) fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    require!(ctx.accounts.global_state.admin.key().eq(ctx.accounts.payer.key), ErrorCode::UnauthorizedAdmin);

    // 获取当前质押金额
    let vault_depositor = ctx.accounts.vault_depositor.load()?;
    let withdrawal_amount = u64::try_from(vault_depositor.get_vault_shares())
        .map_err(|_| error!(ErrorCode::MathOverflow))?;

    // 构造withdraw指令所需的账户
    let cpi_accounts = drift_vaults::cpi::accounts::Withdraw {
        vault: ctx.accounts.vault.to_account_info(),
        vault_depositor: ctx.accounts.vault_depositor.to_account_info(),
        authority: ctx.accounts.global_state.to_account_info(),
        vault_token_account: ctx.accounts.vault_token_account.to_account_info(),
        drift_user_stats: ctx.accounts.drift_user_stats.to_account_info(),
        drift_user: ctx.accounts.drift_user.to_account_info(),
        drift_state: ctx.accounts.drift_state.to_account_info(),
        drift_spot_market_vault: ctx.accounts.drift_spot_market_vault.to_account_info(),
        user_token_account: ctx.accounts.user_token_account.to_account_info(),
        drift_program: ctx.accounts.drift_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        drift_signer: ctx.accounts.drift_signer.to_account_info(),
    };

    drift_vaults::cpi::withdraw(CpiContext::new_with_signer(
        ctx.accounts.drift_program.to_account_info(),
        cpi_accounts,
        &[
            &[
                "program_token_account".as_bytes(),
                &[ctx.bumps.global_state]
            ],
        ])
    )?;

    // 更新资金账户状态
    let fund_account = &mut ctx.accounts.fund_account;
    fund_account.staked_amount = fund_account.staked_amount.checked_sub(withdrawal_amount)
        .ok_or(ErrorCode::MathOverflow)?;
    fund_account.idle_amount = fund_account.idle_amount.checked_add(withdrawal_amount)
        .ok_or(ErrorCode::MathOverflow)?;
    fund_account.last_update_time = Clock::get()?.unix_timestamp;

    // 发出质押状态变更事件
    emit!(StakeStatusChanged {
        amount: withdrawal_amount,
        is_stake: false,
        stake_account: ctx.accounts.vault_depositor.key(),
        timestamp: fund_account.last_update_time,
    });

    // 发出资金状态变更事件
    emit!(FundStatusChanged {
        amount: withdrawal_amount,
        from_status: FundStatus::Staked,  // 从质押状态
        to_status: FundStatus::Idle,      // 变为闲置状态
        token_account: ctx.accounts.user_token_account.key(),
        timestamp: fund_account.last_update_time,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// 全局状态账户，存储管理员地址等信息
    #[account(mut,seeds = ["global_state".as_bytes()],bump)]
    pub global_state: Box<Account<'info, instructions::GlobalState>>,

    /// 资金账户，用于跟踪资金状态（闲置、质押等）
    #[account(
        mut,
        seeds = [b"fund_account", user_token_account.key().as_ref()],
        bump,
    )]
    pub fund_account: Account<'info, FundAccount>,

    /// USDC代币铸造账户
    pub mint: Box<Account<'info, Mint>>,

    /// Drift金库账户，用于管理质押资金
    #[account(mut)]
    pub vault: AccountLoader<'info, Vault>,

    /// 金库的代币账户，用于转出质押的代币
    #[account(mut)]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,

    /// 金库存款人账户，记录质押份额等信息
    #[account(mut)]
    pub vault_depositor: AccountLoader<'info, VaultDepositor>,

    /// 用户的代币账户，用于接收取回的代币
    #[account(mut)]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    /// Drift用户统计账户  CHECK: drift program
    #[account(mut)]
    pub drift_user_stats: AccountInfo<'info>,

    /// Drift用户账户 CHECK: drift program
    #[account(mut)]
    pub drift_user: AccountLoader<'info, User>,

    /// Drift状态账户 CHECK: drift program
    #[account(mut)]
    pub drift_state: AccountInfo<'info>,

    /// Drift现货市场金库账户  CHECK: drift program
    #[account(mut)]
    pub drift_spot_market_vault: AccountInfo<'info>,

    /// Drift签名者账户，用于验证交易 CHECK: drift program
    #[account(mut)]
    pub drift_signer: AccountInfo<'info>,

    /// Drift程序ID
    pub drift_program: Program<'info, Drift>,

    /// Token程序ID
    pub token_program: Program<'info, Token>,

    /// 交易发起人，必须是管理员
    #[account(mut)]
    pub payer: Signer<'info>,
}
