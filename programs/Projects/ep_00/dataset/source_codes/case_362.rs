use anchor_lang::prelude::*;

declare_id!("S9E9223CC622FD14A75291F4FC8FF23D2");

#[program]
pub mod case_362_math {
    use super::*;

    pub fn start(ctx: Context<Start362>, a: u64, b: u64) -> Result<()> {
        let data = &mut ctx.accounts.data;
        data.owner = ctx.accounts.user.key();
        // Perform multiple math ops
        let sum_val = a.checked_add(b).unwrap();
        let xor_val = sum_val ^ 0xA;
        data.val1 = xor_val;
        data.val2 = xor_val.checked_mul(3).unwrap();
        Ok(())
    }

    pub fn swap_values(ctx: Context<Swap362>) -> Result<()> {
        let data = &mut ctx.accounts.data;
        require_keys_eq!(data.owner, ctx.accounts.user.key(), CustomError::AccessDenied);
        // Swap two fields without loops
        let temp = data.val1;
        data.val1 = data.val2;
        data.val2 = temp;
        msg!("Values swapped: {} and {}", data.val1, data.val2);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Start362<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 8)]
    pub data: Account<'info, Case_362State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Swap362<'info> {
    #[account(mut, has_one = user)]
    pub data: Account<'info, Case_362State>,
    #[account(signer)]
    pub user: Signer<'info>,
}

#[account]
pub struct Case_362State {
    pub owner: Pubkey,
    pub val1: u64,
    pub val2: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Access denied.")]
    AccessDenied,
}
