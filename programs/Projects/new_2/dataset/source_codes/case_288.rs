// 7. レコードアーカイブ（while ループ）
use anchor_lang::prelude::*;

#[program]
pub mod record_archiver {
    use super::*;
    pub fn archive(ctx: Context<Archive>) -> Result<()> {
        let buf = &mut ctx.accounts.record_data.try_borrow_mut_data()?;
        let mut idx = 0;
        while idx < buf.len() {
            // 古いレコードをゼロ化
            buf[idx] = 0;
            idx += 1;
        }
        msg!("アーカイブ責任者 {} が archive を実行", ctx.accounts.archiver.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Archive<'info> {
    /// CHECK: レコード用（検証なし）
    pub record_data: AccountInfo<'info>,
    #[account(mut, has_one = archiver)]
    pub arch_ctrl: Account<'info, ArchiverControl>,
    pub archiver: Signer<'info>,
}

#[account]
pub struct ArchiverControl {
    pub archiver: Pubkey,
}
