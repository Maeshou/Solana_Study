use anchor_lang::prelude::*;

declare_id!("MANuaL111111111111111111111111111111111111");

#[program]
pub mod manual_owner_check {
    use super::*;
    pub fn process(ctx: Context<Process>) -> Result<()> {
        // 手動でオーナーチェック
        let account_owner = *ctx.accounts.dynamic_account.owner;
        if account_owner != crate::ID && account_owner != some_other_program::ID {
            return err!(MyError::InvalidOwner);
        }
        // オーナーが正しければ処理を続行
        // ...
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Process<'info> {
    // Anchorによるチェックを意図的にスキップ
    pub dynamic_account: UncheckedAccount<'info>,
    // 他のアカウント...
}

// 別のプログラムIDを仮定
mod some_other_program {
    use super::*;
    declare_id!("OTHER111111111111111111111111111111111111");
}

#[error_code]
pub enum MyError {
    #[msg("The account owner is not valid.")]
    InvalidOwner,
}