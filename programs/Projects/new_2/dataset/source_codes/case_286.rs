// 4. メタ情報クリア（条件分岐あり）
use anchor_lang::prelude::*;

#[program]
pub mod meta_cleaner {
    use super::*;
    pub fn clean(ctx: Context<Clean>, threshold: u8) -> Result<()> {
        let buf = &mut ctx.accounts.meta_data.try_borrow_mut_data()?;
        // 長さによって全クリア or 先頭1バイトのみ
        if buf.len() > threshold as usize {
            for b in buf.iter_mut() { *b = 0; }
        } else {
            if !buf.is_empty() {
                buf[0] = 0;
            }
        }
        msg!("管理者 {} が clean を実行", ctx.accounts.admin.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Clean<'info> {
    /// CHECK: メタ情報用（検証なし）
    pub meta_data: AccountInfo<'info>,
    #[account(has_one = admin)]
    pub admin_ctrl: Account<'info, AdminControl>,
    pub admin: Signer<'info>,
}

#[account]
pub struct AdminControl {
    pub admin: Pubkey,
}
