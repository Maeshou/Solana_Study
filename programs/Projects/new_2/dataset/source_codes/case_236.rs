use anchor_lang::prelude::*;

declare_id!("VulnEx35000000000000000000000000000000000035");

#[program]
pub mod example35 {
    pub fn join_guild(ctx: Context<Ctx35>) -> Result<()> {
        // join_log は所有者検証なし
        ctx.accounts.join_log.data.borrow_mut().push(1);
        // guild_state は has_one で leader 検証済み
        let g = &mut ctx.accounts.guild_state;
        g.members.push(ctx.accounts.member.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx35<'info> {
    #[account(mut, has_one = leader)]
    pub guild_state: Account<'info, GuildState>,
    pub leader: Signer<'info>,
    pub member: Signer<'info>,
    #[account(mut)]
    pub join_log: AccountInfo<'info>,
}

#[account]
pub struct GuildState {
    pub leader: Pubkey,
    pub members: Vec<Pubkey>,
}
