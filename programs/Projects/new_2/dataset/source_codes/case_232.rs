use anchor_lang::prelude::*;

declare_id!("VulnEx30000000000000000000000000000000000030");

#[program]
pub mod farm_pool {
    pub fn harvest(ctx: Context<Ctx10>) -> Result<()> {
        // collector は未検証
        ctx.accounts.collector.data.borrow_mut().extend_from_slice(b"harvested");
        // farm は has_one で farmer 検証済み
        let f = &mut ctx.accounts.farm;
        f.last_harvest = Clock::get()?.unix_timestamp as u64;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx10<'info> {
    #[account(mut, has_one = farmer)]
    pub farm: Account<'info, Farm>,
    pub farmer: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: 収集器アカウント、所有者検証なし
    #[account(mut)]
    pub collector: AccountInfo<'info>,
}
