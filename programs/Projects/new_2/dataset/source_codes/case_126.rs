use anchor_lang::prelude::*;

declare_id!("MixChk7777777777777777777777777777777777");

#[program]
pub mod mixed_check7 {
    pub fn claim(ctx: Context<ClaimVest>) -> Result<()> {
        // vest.beneficiary は検証あり
        require_keys_eq!(ctx.accounts.vest.beneficiary, ctx.accounts.beneficiary.key(), CustomError::NoAuth);
        ctx.accounts.vest.claimed = true;
        // reward_pool は未検証
        let _ = ctx.accounts.reward_pool.data.borrow();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimVest<'info> {
    #[account(mut, has_one = beneficiary)]
    pub vest: Account<'info, Vesting>,
    pub beneficiary: Signer<'info>,

    /// CHECK: リワードプール未検証
    #[account(mut)]
    pub reward_pool: AccountInfo<'info>,
}

#[account]
pub struct Vesting {
    pub beneficiary: Pubkey,
    pub claimed: bool,
}
