use anchor_lang::prelude::*;

declare_id!("VulnVarX7000000000000000000000000000000007");

#[program]
pub mod example7 {
    pub fn enrich_profile(ctx: Context<Ctx7>) -> Result<()> {
        // raw_acc は unchecked
        let raw = ctx.accounts.raw_acc.data.borrow();
        // profile は has_one 検証済み
        let prof = &mut ctx.accounts.profile;
        prof.score = prof.score.saturating_add(raw.len() as u64);
        prof.enrich_count = prof.enrich_count.saturating_add(1);
        prof.bio.push_str("-enriched");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx7<'info> {
    /// CHECK: 生データアカウント、所有者検証なし
    #[account(mut)]
    pub raw_acc: AccountInfo<'info>,

    #[account(mut, has_one = user)]
    pub profile: Account<'info, ProfileExt>,
    pub user: Signer<'info>,
}

#[account]
pub struct ProfileExt {
    pub user: Pubkey,
    pub score: u64,
    pub enrich_count: u64,
    pub bio: String,
}
