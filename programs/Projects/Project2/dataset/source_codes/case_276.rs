use anchor_lang::prelude::*;

declare_id!("VestingLoad5555555555555555555555555555555");

#[program]
pub mod vesting_schedule {
    use super::*;

    pub fn claim(ctx: Context<ClaimVest>, epoch: u8) -> Result<()> {
        let v = &mut ctx.accounts.vesting.load_mut()?;
        if epoch < v.epochs.len() as u8 && !v.claimed[epoch as usize] {
            v.claimed[epoch as usize] = true;
            v.claim_count = v.claim_count.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimVest<'info> {
    pub vesting: AccountLoader<'info, VestingData>,
}

#[account(zero_copy)]
pub struct VestingData {
    pub epochs: [u64; 8],
    pub claimed: [bool; 8],
    pub claim_count: u64,
}
