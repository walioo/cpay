use anchor_lang::prelude::*;

// 资金状态枚举
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum FundStatus {
    // 闲置资金
    Idle,
    // 质押中
    Staked,
    // 已发送（跨链转出）
    Sent,
}

// 资金状态变更事件
#[event]
pub struct FundStatusChanged {
    // 资金数量
    pub amount: u64,
    // 原状态
    pub from_status: FundStatus,
    // 新状态
    pub to_status: FundStatus,
    // 相关代币账户
    pub token_account: Pubkey,
    // 时间戳
    pub timestamp: i64,
}

// 跨链资金接收事件
#[event]
pub struct CrossChainFundReceived {
    // 接收的资金数量
    pub amount: u64,
    // 来源链ID
    pub source_chain: u32,
    // 发送方地址
    pub sender: [u8; 32],
    // 时间戳
    pub timestamp: i64,
}

// 跨链资金发送事件
#[event]
pub struct CrossChainFundSent {
    // 发送的资金数量
    pub amount: u64,
    // 目标链ID
    pub destination_chain: u32,
    // 接收方地址
    pub recipient: [u8; 32],
    // 时间戳
    pub timestamp: i64,
}

// 质押状态变更事件
#[event]
pub struct StakeStatusChanged {
    // 质押金额
    pub amount: u64,
    // 是否是质押操作（false表示赎回）
    pub is_stake: bool,
    // 质押账户
    pub stake_account: Pubkey,
    // 时间戳
    pub timestamp: i64,
}

// 资金账户状态
#[account]
pub struct FundAccount {
    // 闲置资金数量
    pub idle_amount: u64,
    // 质押中的资金数量
    pub staked_amount: u64,
    // 已发送的资金数量（跨链转出）
    pub sent_amount: u64,
    // 总接收的资金数量（跨链转入）
    pub total_received_amount: u64,
    // 最后更新时间
    pub last_update_time: i64,
    // 管理员
    pub authority: Pubkey,
}

impl FundAccount {
    pub const LEN: usize = 8 + // discriminator
        8 + // idle_amount
        8 + // staked_amount
        8 + // sent_amount
        8 + // total_received_amount
        8 + // last_update_time
        32; // authority
}
