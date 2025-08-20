use anchor_lang::prelude::*;

declare_id!("NFTCraftClose6666666666666666666666666666");

#[program]
pub mod craft_queue_wiper {
    use super::*;

    pub fn wipe_queue(ctx: Context<WipeQueue>, entropy: u64) -> Result<()> {
        let q = ctx.accounts.queue.to_account_info();
        let pay = ctx.accounts.payout.to_account_info();

        let lq = q.lamports();
        let stream = (0..16u64)
            .map(|i| (entropy ^ lq).rotate_left((i & 15) as u32).wrapping_add(i * 55 + 7))
            .fold(0u64, |a, x| a.wrapping_add(x ^ (a.rotate_right(3))));

        let move_all = lq;
        **pay.lamports.borrow_mut() = pay.lamports().checked_add(move_all).unwrap();
        let mut ls = q.lamports.borrow_mut();
        let pv = *ls;
        *ls = pv.checked_sub(move_all).unwrap();

        ctx.accounts.queue.hashline = stream;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WipeQueue<'info> {
    #[account(mut)]
    pub queue: Account<'info, CraftQueue>,
    /// CHECK:
    #[account(mut)]
    pub payout: UncheckedAccount<'info>,
}
#[account]
pub struct CraftQueue { pub hashline: u64 }
