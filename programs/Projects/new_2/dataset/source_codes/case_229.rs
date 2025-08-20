use anchor_lang::prelude::*;

declare_id!("VulnEx27000000000000000000000000000000000027");

#[program]
pub mod raffle_draw {
    pub fn draw(ctx: Context<Ctx7>) -> Result<()> {
        // result_buf は未検証
        let seed = Clock::get()?.unix_timestamp as u64;
        ctx.accounts.result_buf.data.borrow_mut()[0..8].copy_from_slice(&seed.to_le_bytes());
        // raffle は has_one で organizer 検証済み
        let r = &mut ctx.accounts.raffle;
        r.last_seed = seed;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx7<'info> {
    pub clock: Sysvar<'info, Clock>,
    #[account(mut)]
    pub result_buf: AccountInfo<'info>,
    #[account(mut, has_one = organizer)]
    pub raffle: Account<'info, Raffle>,
    pub organizer: Signer<'info>,
}
