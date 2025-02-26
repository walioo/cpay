use anchor_lang::prelude::*;

// 消息传输器账户，用于跨链消息的验证和处理
#[account]
#[derive(Default)]
pub struct MessageTransmitter {
    // 管理员权限地址
    pub authority: Pubkey,
    // 消息序列号
    pub nonce: u64,
    // 是否暂停服务
    pub paused: bool,
}

// 代币消息传输器账户，用于处理特定代币的跨链传输
#[account]
#[derive(Default)]
pub struct TokenMessenger {
    // 管理员权限地址
    pub authority: Pubkey,
    // 本地代币地址
    pub local_token: Pubkey,
    // 目标链域ID
    pub remote_domain: u32,
    // 消息序列号
    pub nonce: u64,
}

// 销毁代币消息账户，记录跨链转账时的代币销毁信息
#[account]
#[derive(Default)]
pub struct BurnMessage {
    // 销毁的代币数量
    pub amount: u64,
    // 代币铸造地址
    pub mint: Pubkey,
    // 目标链域ID
    pub remote_domain: u32,
    // 接收方地址（32字节）
    pub recipient: [u8; 32],
    // 消息序列号
    pub nonce: u64,
}
