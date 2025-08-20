use anchor_lang::prelude::*;

declare_id!("VulnEx77000000000000000000000000000000000077");

#[program]
pub mod example77 {
    pub fn count_occurrences(ctx: Context<Ctx77>, target: u8) -> Result<()> {
        // data_acc: OWNER CHECK SKIPPED
        let data = ctx.accounts.data_acc.data.borrow();
        let count = data.iter().filter(|&&b| b == target).count() as u64;

        // count_state: has_one = keeper
        ctx.accounts.count_state.last_count = count;
        ctx.accounts.count_state.update_count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx77<'info> {
    #[account(mut)]
    pub data_acc: AccountInfo<'info>,  // unchecked
    #[account(mut, has_one = keeper)]
    pub count_state: Account<'info, CountState>,
    pub keeper: Signer<'info>,
}

#[account]
pub struct CountState {
    pub keeper: Pubkey,
    pub last_count: u64,
    pub update_count: u64,
}
