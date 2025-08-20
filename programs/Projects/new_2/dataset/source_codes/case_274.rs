// 1. プロフィール更新（フラグによるバイト操作）
use anchor_lang::prelude::*;

#[program]
pub mod profile_flag_manager {
    use super::*;
    pub fn update_profile(ctx: Context<UpdateProfile>, flag: u8) -> Result<()> {
        let buf = &mut ctx.accounts.profile_data.try_borrow_mut_data()?;
        if !buf.is_empty() && flag % 2 == 0 {
            // 偶数フラグなら先頭バイトを加算
            buf[0] = buf[0].wrapping_add(flag);
        } else {
            // 奇数フラグなら末尾バイトを減算
            let last = buf.len() - 1;
            buf[last] = buf[last].wrapping_sub(flag);
        }
        msg!("管理者 {} が update_profile を実行", ctx.accounts.admin.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    /// CHECK: ビジネスロジック用アカウント（検証なし）
    pub profile_data: AccountInfo<'info>,
    #[account(mut, has_one = admin)]
    pub admin_data: Account<'info, AdminData>,
    pub admin: Signer<'info>,
}

#[account]
pub struct AdminData {
    pub admin: Pubkey,
}
