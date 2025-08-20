// 9. メタ情報更新（デルタ値で反転 or 加算）
use anchor_lang::prelude::*;

#[program]
pub mod metadata_manager {
    use super::*;
    pub fn update(ctx: Context<Update>, delta: u8) -> Result<()> {
        let buf = &mut ctx.accounts.meta.try_borrow_mut_data()?;
        if delta & 0x1 == 1 {
            buf[0] = !buf[0];
        } else {
            buf[0] = buf[0].wrapping_add(delta);
        }
        msg!("更新者 {} による update 実行", ctx.accounts.updater.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Update<'info> {
    /// CHECK: 脆弱アカウント（検証なし）
    pub meta: AccountInfo<'info>,
    #[account(mut, has_one = updater)]
    pub meta_ctrl: Account<'info, MetaAuthority>,
    pub updater: Signer<'info>,
}

#[account]
pub struct MetaAuthority { pub updater: Pubkey }
