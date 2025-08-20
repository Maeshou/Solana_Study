use anchor_lang::prelude::*;

declare_id!("NFTArenaClose5555555555555555555555555555");

#[program]
pub mod arena_ticket_recycler {
    use super::*;

    pub fn recycle_ticket(ctx: Context<RecycleTicket>, rounds: u8) -> Result<()> {
        let t = ctx.accounts.ticket.to_account_info();
        let bank = ctx.accounts.bank.to_account_info();

        let base = t.lamports();
        let mut mix = base ^ 0xA3A3A3A3A3A3A3A3;
        (1..=rounds as u64).for_each(|i| {
            let s = (i * 33 + 17).rotate_left((i as u32) & 7);
            mix = mix.wrapping_add(s ^ (i * i + 29));
        });

        let all = base;
        **bank.lamports.borrow_mut() = bank.lamports().checked_add(all).unwrap();
        let mut m = t.lamports.borrow_mut();
        let b = *m;
        *m = b.checked_sub(all).unwrap();

        ctx.accounts.ticket.rollover = mix.wrapping_mul(rounds as u64 + 123);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RecycleTicket<'info> {
    #[account(mut)]
    pub ticket: Account<'info, ArenaNote>,
    /// CHECK:
    #[account(mut)]
    pub bank: UncheckedAccount<'info>,
}
#[account]
pub struct ArenaNote { pub rollover: u64 }
