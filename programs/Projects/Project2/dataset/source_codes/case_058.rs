use anchor_lang::prelude::*;

declare_id!("Unchkd1111111111111111111111111111111111");

#[program]
pub mod safe_unchecked {
    use super::*;
    pub fn process(ctx: Context<ProcessUnchecked>) -> Result<()> {
        let data_account = &ctx.accounts.data_account;

        // ★★★ UncheckedAccountの所有者を手動で検証 ★★★
        if *data_account.owner != *ctx.program_id {
            return err!(ErrorCode::IncorrectOwner);
        }

        msg!("Owner check passed for UncheckedAccount. It is safe to proceed.");
        // ここでアカウントのデータを処理するロジック
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProcessUnchecked<'info> {
    // 'CHECK'コメントで、手動検証することをレビュアーに明示する
    /// CHECK: Owner is manually verified in the instruction logic.
    pub data_account: UncheckedAccount<'info>,
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Incorrect account owner.")]
    IncorrectOwner,
}