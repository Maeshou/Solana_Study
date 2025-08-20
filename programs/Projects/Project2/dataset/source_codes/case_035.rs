use anchor_lang::prelude::*;

// 信頼するOracleプログラムのID
const TRUSTED_ORACLE_PROGRAM_ID: Pubkey = pubkey!("oracleid1111111111111111111111111111111111");

declare_id!("EXAMPLe111111111111111111111111111111111111");

#[program]
pub mod safe_cpi_with_account_info {
    use super::*;
    pub fn call_oracle(ctx: Context<CallOracle>) -> Result<()> {
        // ここでOracleプログラムへのCPIを実行
        // ...
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CallOracle<'info> {
    // AccountInfoを使っているが、以下の制約で安全性を担保
    #[account(
        executable, // このアカウントが実行可能ファイル（プログラム）であることを検証
        constraint = oracle_program.key() == TRUSTED_ORACLE_PROGRAM_ID @ MyError::InvalidOracleProgram
    )]
    pub oracle_program: AccountInfo<'info>,
    // 他のアカウント...
}

#[error_code]
pub enum MyError {
    #[msg("Invalid Oracle Program ID")]
    InvalidOracleProgram,
}