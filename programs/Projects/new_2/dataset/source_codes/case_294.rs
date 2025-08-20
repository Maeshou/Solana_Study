// 1. プロフィール統計管理モジュール
use anchor_lang::prelude::*;

#[program]
pub mod profile_stats {
    use super::*;
    // 統計値の加算
    pub fn update_stats(ctx: Context<UpdateStats>, increment: u8) -> Result<()> {
        let data = &mut ctx.accounts.user_profile.try_borrow_mut_data()?;
        if data.len() >= 4 {
            let mut val = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            val = val.wrapping_add(increment as u32);
            let bytes = val.to_le_bytes();
            data[0..4].copy_from_slice(&bytes);
        }
        msg!("管理者 {} が {} を加算", ctx.accounts.admin.key(), increment);
        Ok(())
    }
    // 統計値のリセット
    pub fn reset_stats(ctx: Context<ResetStats>) -> Result<()> {
        let data = &mut ctx.accounts.user_profile.try_borrow_mut_data()?;
        for b in data.iter_mut().take(4) { *b = 0; }
        msg!("管理者 {} が統計をリセット", ctx.accounts.admin.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateStats<'info> {
    /// CHECK: プロファイル統計データ（検証なし）
    pub user_profile: AccountInfo<'info>,
    #[account(mut, has_one = admin)]
    pub admin_data: Account<'info, AdminData>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResetStats<'info> {
    /// CHECK: プロファイル統計データ（検証なし）
    pub user_profile: AccountInfo<'info>,
    #[account(mut, has_one = admin)]
    pub admin_data: Account<'info, AdminData>,
    pub admin: Signer<'info>,
}

#[account]
pub struct AdminData {
    pub admin: Pubkey,
}
