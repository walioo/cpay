use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::error::ErrorCode;
use super::state::*;

// 接收跨链消息并铸造代币的指令账户结构
#[derive(Accounts)]
pub struct ReceiveMessage<'info> {
    // 交易发起人，必须签名
    #[account(mut)]
    pub authority: Signer<'info>,
    
    // 消息传输器账户，通过PDA派生
    #[account(
        mut,
        seeds = [b"message_transmitter"],
        bump,
    )]
    pub message_transmitter: Account<'info, MessageTransmitter>,
    
    // 程序的USDC代币账户，通过PDA派生
    #[account(
        mut,
        seeds = [b"program_token_account", mint.key().as_ref()],
        bump,
    )]
    pub program_token_account: Account<'info, TokenAccount>,
    
    // USDC代币铸造账户
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    // SPL代币程序
    pub token_program: Program<'info, Token>,
    // 系统程序
    pub system_program: Program<'info, System>,
}

// 跨链消息数据结构
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MessageData {
    // 源链域ID
    pub source_domain: u32,
    // 发送方地址
    pub sender: [u8; 32],
    // 接收方地址
    pub recipient: Pubkey,
    // 代币数量
    pub amount: u64,
    // 消息体数据
    pub message_body: Vec<u8>,
}

// 接收跨链消息并铸造代币的处理函数
pub fn receive_message(
    ctx: Context<ReceiveMessage>, 
    message: MessageData,           // 跨链消息数据
    signatures: Vec<[u8; 65]>,      // 验证人签名列表
) -> Result<()> {
    let message_transmitter = &mut ctx.accounts.message_transmitter;
    
    // 验证消息传输器未暂停
    require!(!message_transmitter.paused, ErrorCode::TransmitterPaused);
    
    // 验证签名列表非空（在生产环境中，这里需要验证签名是否来自授权验证人）
    require!(!signatures.is_empty(), ErrorCode::InvalidSignatures);
    
    // 准备签名者的种子
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"message_transmitter".as_ref(),
        &[ctx.bumps.message_transmitter],
    ]];

    // 准备所需的账户信息
    let token_program = ctx.accounts.token_program.to_account_info();
    let mint = ctx.accounts.mint.to_account_info();
    let program_token_account = ctx.accounts.program_token_account.to_account_info();
    let message_transmitter = ctx.accounts.message_transmitter.to_account_info();
    
    // 构建铸造代币的CPI上下文，使用PDA签名
    let mint_to_ctx = CpiContext::new_with_signer(
        token_program,
        token::MintTo {
            mint,
            to: program_token_account,  // 铸造到程序的PDA token account
            authority: message_transmitter,
        },
        signer_seeds,
    );
    
    // 执行代币铸造到程序的PDA token account
    token::mint_to(mint_to_ctx, message.amount)?;
    
    Ok(())
}
