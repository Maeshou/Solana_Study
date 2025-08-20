use anchor_lang::prelude::*;

declare_id!("NFTStableClose3333333333333333333333333333");

#[program]
pub mod pet_stable_disposer {
    use super::*;

    pub fn retire_stable(ctx: Context<RetireStable>, salt: u64) -> Result<()> {
        let stable_ai = ctx.accounts.stable.to_account_info();
        let treasury_ai = ctx.accounts.treasury.to_account_info();

        let l = stable_ai.lamports();
        let curve = (0..12u32).fold(salt ^ l, |acc, i| acc.rotate_left(i & 15).wrapping_add((i as u64 + 21) * 131));
        let waves = curve ^ (l.rotate_right(7));

        let move_all = l;
        **treasury_ai.lamports.borrow_mut() = treasury_ai.lamports().checked_add(move_all).unwrap();
        let mut s = stable_ai.lamports.borrow_mut();
        let p = *s;
        *s = p.checked_sub(move_all).unwrap();

        ctx.accounts.stable.tallies = waves.to_le_bytes();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RetireStable<'info> {
    #[account(mut)]
    pub stable: Account<'info, StableBook>,
    /// CHECK: 金庫
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
}
#[account]
pub struct StableBook { pub tallies: [u8; 8] }
