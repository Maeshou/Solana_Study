
use anchor_lang::prelude::*;

declare_id!("BUMPSEED099213ID");

#[program]
pub mod bump_seed_case_099 {
    use super::*;

    pub fn form_099(ctx: Context<LaunchCtx099>, bump: u8) -> Result<()> {
        let data_acc = &mut ctx.accounts.account_099;
        let ratio = (data_acc.count as f64) / ((bump as f64) + 1.0);
        let scaled = (ratio * 10.0).round() as u64;
        data_acc.count = scaled.checked_sub(bump as u64).unwrap_or(0);
        data_acc.count = data_acc.count.saturating_add(100);
        msg!("case 099: result={}", data_acc.count);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct LaunchCtx099<'info> {
    #[account(init, payer = user, seeds = [b"bump_seed_case_099", user.key().as_ref(), bump.to_le_bytes().as_ref()], bump)]
    pub account_099: Account<'info, DataAccount>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub count: u64,
}
