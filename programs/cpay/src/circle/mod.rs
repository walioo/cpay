// Circle CCTP (Cross-Chain Transfer Protocol) 模块
pub mod deposit_for_burn;   // 销毁代币并创建跨链转账消息
pub mod receive_message;    // 接收跨链消息并铸造代币
pub mod state;             // 状态账户定义

// 导出所有子模块内容
pub use deposit_for_burn::*;
pub use receive_message::*;
pub use state::*;
