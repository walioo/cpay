use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount, Mint};
use crate::error::ErrorCode;
use crate::state::*;
use super::state::*;

// 销毁代币并创建跨链转账消息的指令账户结构
#[derive(Accounts)]
pub struct DepositForBurn<'info> {
    // 交易发起人，必须签名
    #[account(mut)]
    pub authority: Signer<'info>,
    
    // 程序的USDC代币账户，通过PDA派生
    #[account(
        mut,
        seeds = [b"program_token_account", mint.key().as_ref()],
        bump,
    )]
    pub program_token_account: Account<'info, TokenAccount>,

    // 资金账户
    #[account(
        mut,
        seeds = [b"fund_account", program_token_account.key().as_ref()],
        bump,
    )]
    pub fund_account: Account<'info, FundAccount>,
    
    // 代币铸造账户
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    // 代币消息传输器账户，通过PDA派生
    #[account(
        mut,
        seeds = [b"token_messenger", mint.key().as_ref()],
        bump,
    )]
    pub token_messenger: Account<'info, TokenMessenger>,
    
    // 销毁消息账户，用于记录本次跨链转账信息
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<BurnMessage>(),
        seeds = [b"burn_message", token_messenger.nonce.to_le_bytes().as_ref()],
        bump
    )]
    pub burn_message: Account<'info, BurnMessage>,
    
    // SPL代币程序
    pub token_program: Program<'info, Token>,
    // 系统程序
    pub system_program: Program<'info, System>,
}

// 销毁代币并创建跨链转账消息的处理函数
pub fn deposit_for_burn(
    ctx: Context<DepositForBurn>,
    amount: u64,                    // 要销毁的代币数量
    destination_domain: u32,        // 目标链域ID
    mint_recipient: [u8; 32],       // 目标链接收方地址
) -> Result<()> {
    let token_messenger = &mut ctx.accounts.token_messenger;
    let burn_message = &mut ctx.accounts.burn_message;
    let fund_account = &mut ctx.accounts.fund_account;
    
    // 验证闲置资金是否足够
    require!(
        fund_account.idle_amount >= amount,
        ErrorCode::InsufficientIdleFunds
    );
    
    // 创建销毁消息记录
    burn_message.amount = amount;
    burn_message.mint = ctx.accounts.mint.key();
    burn_message.remote_domain = destination_domain;
    burn_message.recipient = mint_recipient;
    burn_message.nonce = token_messenger.nonce;
    
    // 增加消息序列号
    token_messenger.nonce = token_messenger.nonce.checked_add(1).ok_or(ErrorCode::MathOverflow)?;
    
    // 获取mint的公钥
    let mint_key = ctx.accounts.mint.key();
    
    // 准备程序PDA的签名种子
    let program_token_seeds: &[&[&[u8]]] = &[&[
        b"program_token_account",
        mint_key.as_ref(),
        &[ctx.bumps.program_token_account],
    ]];
    
    // 构建销毁代币的CPI上下文，使用程序PDA签名
    let burn_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.program_token_account.to_account_info(),
            authority: ctx.accounts.program_token_account.to_account_info(), // 程序PDA作为authority
        },
        program_token_seeds,
    );
    
    // 执行代币销毁
    token::burn(burn_ctx, amount)?;

    // 更新资金账户状态
    fund_account.idle_amount = fund_account.idle_amount.checked_sub(amount)
        .ok_or(ErrorCode::MathOverflow)?;
    fund_account.sent_amount = fund_account.sent_amount.checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;
    fund_account.last_update_time = Clock::get()?.unix_timestamp;

    // 发出跨链资金发送事件
    emit!(CrossChainFundSent {
        amount,
        destination_chain: destination_domain,
        recipient: mint_recipient,
        timestamp: fund_account.last_update_time,
    });

    // 发出资金状态变更事件
    emit!(FundStatusChanged {
        amount,
        from_status: FundStatus::Idle,  // 从闲置状态
        to_status: FundStatus::Sent,    // 变为已发送状态
        token_account: ctx.accounts.program_token_account.key(),
        timestamp: fund_account.last_update_time,
    });
    
    Ok(())
}
