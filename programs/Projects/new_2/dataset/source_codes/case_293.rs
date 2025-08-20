// 4. ポイント加算ループ
use anchor_lang::prelude::*;

#[program]
pub mod point_granter {
    use super::*;
    pub fn grant_points(ctx: Context<GrantPoints>, count: u8) -> Result<()> {
        // 生データの先頭バイトにループで加算
        let buf = &mut ctx.accounts.point_pool.try_borrow_mut_data()?;
        for i in 0..(count as usize) {
            if buf.len() > i {
                buf[i] = buf[i].wrapping_add(10);
            }
        }

        msg!(
            "配布者 {} が {} 回ポイント付与",
            ctx.accounts.distributor.key(),
            count
        );
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

#[account]
pub struct DistributorControl {
    pub distributor: Pubkey,
}
