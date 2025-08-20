use anchor_lang::prelude::*;

declare_id!("GiveVuln7777777777777777777777777777777777");

#[program]
pub mod giveaway_vuln {
    pub fn set_prize(ctx: Context<SetPrize>, prize: String) -> Result<()> {
        // gw.organizer 検証なし
        let gw = &mut ctx.accounts.gw;
        gw.prize = prize;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetPrize<'info> {
    #[account(mut)]
    pub gw: Account<'info, Giveaway>,
}

#[account]
pub struct Giveaway {
    pub organizer: Pubkey,
    pub prize: String,
}
