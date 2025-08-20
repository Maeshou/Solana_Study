// 5. 報酬配布（ループ＋分岐）
use anchor_lang::prelude::*;

#[program]
pub mod reward_distributor {
    use super::*;
    pub fn distribute(ctx: Context<Distribute>, users: u8) -> Result<()> {
        let buf = &mut ctx.accounts.reward_pool.try_borrow_mut_data()?;
        for i in 0..users as usize {
            // ユーザ数が8未満なら先頭8バイト、以上なら全体
            if users < 8 {
                if buf.len() > i {
                    buf[i] = buf[i].wrapping_add(5);
                }
            } else {
                for b in buf.iter_mut() { *b = b.wrapping_add(1); }
                break;
            }
        }
        msg!("配布責任者 {} が distribute を実行", ctx.accounts.distributor.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    /// CHECK: 報酬プール用（検証なし）
    pub reward_pool: AccountInfo<'info>,
    #[account(mut, has_one = distributor)]
    pub dist_ctrl: Account<'info, DistributorControl>,
    pub distributor: Signer<'info>,
}

#[account]
pub struct DistributorControl {
    pub distributor: Pubkey,
}
