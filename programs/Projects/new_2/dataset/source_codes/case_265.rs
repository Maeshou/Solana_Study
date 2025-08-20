use anchor_lang::prelude::*;

declare_id!("VulnEx79000000000000000000000000000000000079");

#[program]
pub mod example79 {
    pub fn accumulate(ctx: Context<Ctx79>) -> Result<()> {
        // fees_map: has_one = admin
        for (&k, &v) in ctx.accounts.fees_map.fees.iter() {
            ctx.accounts.accumulator.total = ctx.accounts.accumulator.total.saturating_add(v);
        }
        ctx.accounts.accumulator.rounds += 1;

        // log_acc: OWNER CHECK SKIPPED
        ctx.accounts.log_acc.data.borrow_mut().extend_from_slice(&ctx.accounts.accumulator.total.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx79<'info> {
    #[account(mut, has_one = admin)]
    pub fees_map: Account<'info, FeesMap>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub log_acc: AccountInfo<'info>,  // unchecked
    #[account(mut)]
    pub accumulator: Account<'info, Accumulator>,
}

#[account]
pub struct FeesMap {
    pub admin: Pubkey,
    pub fees: std::collections::BTreeMap<u64, u64>,
}

#[account]
pub struct Accumulator {
    pub total: u64,
    pub rounds: u64,
}
