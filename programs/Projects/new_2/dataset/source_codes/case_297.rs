// 4. ポイント配布モジュール
use anchor_lang::prelude::*;

#[program]
pub mod point_service {
    use super::*;
    // 複数ユーザにポイントを付与
    pub fn grant_points(ctx: Context<GrantPoints>, users: u8) -> Result<()> {
        let buf = &mut ctx.accounts.point_pool.try_borrow_mut_data()?;
        for i in 0..(users as usize).min(buf.len()) {
            buf[i] = buf[i].wrapping_add(10);
        }
        msg!("配布責任者 {} が {}人 にポイント付与", ctx.accounts.distributor.key(), users);
        Ok(())
    }
    // 付与済みのポイントを一括削除
    pub fn revoke_points(ctx: Context<RevokePoints>) -> Result<()> {
        let buf = &mut ctx.accounts.point_pool.try_borrow_mut_data()?;
        for b in buf.iter_mut() { *b = 0; }
        msg!("配布責任者 {} がポイント削除", ctx.accounts.distributor.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GrantPoints<'info> {
    /// CHECK: ポイントプール（検証なし）
    pub point_pool: AccountInfo<'info>,
    #[account(has_one = distributor)]
    pub dist_ctrl: Account<'info, DistributorControl>,
    pub distributor: Signer<'info>,
}

#[derive(Accounts)]
pub struct RevokePoints<'info> {
    /// CHECK: ポイントプール（検証なし）
    pub point_pool: AccountInfo<'info>,
    #[account(mut, has_one = distributor)]
    pub dist_ctrl: Account<'info, DistributorControl>,
    pub distributor: Signer<'info>,
}

#[account]
pub struct DistributorControl {
    pub distributor: Pubkey,
}
