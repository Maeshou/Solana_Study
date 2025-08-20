use anchor_lang::prelude::*;

// 偽のOracleプログラムID（例: Chainlink）
declare_id!("Oracle1111111111111111111111111111111111");
// 自分のプログラムID
declare_id!("MyDApp11111111111111111111111111111111111");

#[program]
pub mod cpi_checker {
    use super::*;
    pub fn check_price(ctx: Context<CheckPrice>) -> Result<()> {
        // ここで価格データを読み取り、ロジックを実行する
        msg!("Price feed owner is correct. Proceeding.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckPrice<'info> {
    /// CHECK: The owner constraint is the security measure here.
    #[account(
        constraint = price_feed.owner == &oracle_program::ID @ ErrorCode::InvalidOracleOwner
    )]
    pub price_feed: AccountInfo<'info>,
    pub oracle_program: Program<'info, OracleProgram>, // Just to get the ID
}

// Oracleプログラムのインターフェースを定義
#[program]
pub mod oracle_program {
    use super::*;
    // ... Oracleの命令 ...
}


#[error_code]
pub enum ErrorCode {
    #[msg("Invalid oracle program owner.")]
    InvalidOracleOwner,
}