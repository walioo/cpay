use anchor_lang::prelude::*;

// 程序错误码定义
#[error_code]
pub enum ErrorCode {
    // 数学运算溢出错误
    #[msg("repeated init")]
    RepeatedInit,
    // 数学运算溢出错误
    #[msg("Math operation overflow")]
    MathOverflow,
    
    // 闲置资金不足错误
    #[msg("Insufficient idle funds")]
    InsufficientIdleFunds,
    
    // 未授权的管理员操作错误
    #[msg("Unauthorized admin")]
    UnauthorizedAdmin,
    
    // 无效参数错误
    #[msg("Invalid argument")]
    InvalidArgument,
    
    // 无效指令数据错误
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
    
    // 账户数据太小错误
    #[msg("Account data is too small")]
    AccountDataTooSmall,
    
    // 账户数据太大错误
    #[msg("Account data is too large")]
    AccountDataTooLarge,
    
    // 账户未初始化错误
    #[msg("Account is not initialized")]
    AccountNotInitialized,
    
    // 账户已经初始化错误
    #[msg("Account is already initialized")]
    AccountAlreadyInitialized,
    
    // 无效账户所有者错误
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    
    // 无效账户判别符错误
    #[msg("Invalid account discriminator")]
    InvalidAccountDiscriminator,
    
    // 账户不可写错误
    #[msg("Account is not writable")]
    AccountNotWritable,
    
    // 账户不可读错误
    #[msg("Account is not readable")]
    AccountNotReadable,
    
    // 无效种子错误
    #[msg("Invalid seeds")]
    InvalidSeeds,
    
    // 无效程序地址错误
    #[msg("Invalid program address")]
    InvalidProgramAddress,
    
    // 无效程序ID错误
    #[msg("Invalid program id")]
    InvalidProgramId,
    
    // 缺少必需签名错误
    #[msg("Missing required signature")]
    MissingRequiredSignature,
    
    // 无效签名错误
    #[msg("Invalid signature")]
    InvalidSignature,
    
    // 无效指令错误
    #[msg("Invalid instruction")]
    InvalidInstruction,
    
    // 账户未初始化错误
    #[msg("Uninitialized account")]
    UninitializedAccount,
    
    // 账户已经初始化错误
    #[msg("Already initialized account")]
    AlreadyInitializedAccount,
    
    // 程序ID不正确错误
    #[msg("Incorrect program id")]
    IncorrectProgramId,
    
    // 缺少必需键错误
    #[msg("Missing required key")]
    MissingRequiredKey,
    
    // 账户借入不匹配错误
    #[msg("Account borrowed does not match")]
    AccountBorrowedDoesNotMatch,
    
    // 借入账户不匹配错误
    #[msg("Borrowed account does not match")]
    BorrowedAccountDoesNotMatch,
    
    // 无效账户借入错误
    #[msg("Invalid account borrow")]
    InvalidAccountBorrow,
    
    // 无效账户借入可变错误
    #[msg("Invalid account borrow mut")]
    InvalidAccountBorrowMut,
    
    // 无效账户拥有错误
    #[msg("Invalid account has one")]
    InvalidAccountHasOne,
    
    // 无效账户拥有可变错误
    #[msg("Invalid account has one mut")]
    InvalidAccountHasOneMut,
    
    // 无效种子长度错误
    #[msg("Invalid seeds length")]
    InvalidSeedsLength,
    
    // 无效种子索引错误
    #[msg("Invalid seeds index")]
    InvalidSeedsIndex,
    
    // 无效程序地址长度错误
    #[msg("Invalid program address length")]
    InvalidProgramAddressLength,
    
    // 无效程序ID长度错误
    #[msg("Invalid program id length")]
    InvalidProgramIdLength,
    
    // 无效签名长度错误
    #[msg("Invalid signature length")]
    InvalidSignatureLength,
    
    // 无效指令数据长度错误
    #[msg("Invalid instruction data length")]
    InvalidInstructionDataLength,
    
    // 无效账户数据长度错误
    #[msg("Invalid account data length")]
    InvalidAccountDataLength,
    
    // 无效账户判别符长度错误
    #[msg("Invalid account discriminator length")]
    InvalidAccountDiscriminatorLength,
    
    // 无效种子长度为账户错误
    #[msg("Invalid seeds length for account")]
    InvalidSeedsLengthForAccount,
    
    // 无效种子索引为账户错误
    #[msg("Invalid seeds index for account")]
    InvalidSeedsIndexForAccount,
    
    // 无效程序地址为账户错误
    #[msg("Invalid program address for account")]
    InvalidProgramAddressForAccount,
    
    // 无效程序ID为账户错误
    #[msg("Invalid program id for account")]
    InvalidProgramIdForAccount,
    
    // 无效签名为账户错误
    #[msg("Invalid signature for account")]
    InvalidSignatureForAccount,
    
    // 无效指令数据为账户错误
    #[msg("Invalid instruction data for account")]
    InvalidInstructionDataForAccount,
    
    // 无效账户数据为账户错误
    #[msg("Invalid account data for account")]
    InvalidAccountDataForAccount,
    
    // 无效账户判别符为账户错误
    #[msg("Invalid account discriminator for account")]
    InvalidAccountDiscriminatorForAccount,
    
    // 无效种子为账户错误
    #[msg("Invalid seeds for account")]
    InvalidSeedsForAccount,
    
    // 无效程序地址为指令错误
    #[msg("Invalid program address for instruction")]
    InvalidProgramAddressForInstruction,
    
    // 无效程序ID为指令错误
    #[msg("Invalid program id for instruction")]
    InvalidProgramIdForInstruction,
    
    // 无效签名为指令错误
    #[msg("Invalid signature for instruction")]
    InvalidSignatureForInstruction,
    
    // 无效指令数据为指令错误
    #[msg("Invalid instruction data for instruction")]
    InvalidInstructionDataForInstruction,
    
    // 无效账户数据为指令错误
    #[msg("Invalid account data for instruction")]
    InvalidAccountDataForInstruction,
    
    // 无效账户判别符为指令错误
    #[msg("Invalid account discriminator for instruction")]
    InvalidAccountDiscriminatorForInstruction,
    
    // 无效种子为指令错误
    #[msg("Invalid seeds for instruction")]
    InvalidSeedsForInstruction,
    
    // Circle CCTP 错误
    #[msg("Message transmitter is paused")]
    TransmitterPaused,

    #[msg("Invalid message signatures")]
    InvalidSignatures,
}
