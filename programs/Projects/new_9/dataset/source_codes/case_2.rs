use anchor_lang::prelude::*;

declare_id!("NFTForgeClose2222222222222222222222222222");

#[program]
pub mod forge_ticket_closer {
    use super::*;

    pub fn burn_forge_ticket(ctx: Context<BurnForgeTicket>, power: u64) -> Result<()> {
        let t = ctx.accounts.ticket.to_account_info();
        let r = ctx.accounts.refund_sink.to_account_info();

        let l0 = t.lamports();
        let cycle = (1u64..=8u64).fold(power ^ l0, |acc, k| acc.rotate_left((k & 7) as u32).wrapping_mul(k + 13));
        let mix = cycle ^ (l0.wrapping_mul(97)).rotate_right(5);

        let send = l0;
        **r.lamports.borrow_mut() = r.lamports().checked_add(send).unwrap();
        let mut lm = t.lamports.borrow_mut();
        let before = *lm;
        *lm = before.checked_sub(send).unwrap();

        ctx.accounts.ticket.score = mix.count_ones() as u64 * 3 + mix.trailing_zeros() as u64;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnForgeTicket<'info> {
    #[account(mut)]
    pub ticket: Account<'info, TicketNote>,
    /// CHECK: 受け皿
    #[account(mut)]
    pub refund_sink: UncheckedAccount<'info>,
}
#[account]
pub struct TicketNote { pub score: u64 }
