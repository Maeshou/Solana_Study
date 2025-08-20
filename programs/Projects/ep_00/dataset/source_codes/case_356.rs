use anchor_lang::prelude::*;

declare_id!("S4593FBB37A5A93E31301B801DA69DB8E");

#[program]
pub mod case_356_module {
    use super::*;

    pub fn initialize(ctx: Context<Initialize356>, value: u64) -> Result<()> {
        let record = &mut ctx.accounts.record;
        // Set the owner to signer
        record.owner = ctx.accounts.user.key();
        // Assign initial value plus offset
        let temp = value.checked_add(3).unwrap();
        record.count = temp;
        // Log the initialization
        msg!("Initialized {} with count {}", record.owner, record.count);
        Ok(())
    }

    pub fn modify(ctx: Context<Modify356>, new_value: u64) -> Result<()> {
        let record = &mut ctx.accounts.record;
        // Check ownership
        require_keys_eq!(record.owner, ctx.accounts.user.key(), CustomError::NotAuthorized);
        // Compute updated value with rotation
        let rotated = new_value.rotate_left(2);
        record.count = rotated;
        // Store a derived flag (bitwise)
        record.flag = (rotated & 1) == 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize356<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 1)]
    pub record: Account<'info, Case_356State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify356<'info> {
    #[account(mut, has_one = user)]
    pub record: Account<'info, Case_356State>,
    #[account(signer)]
    pub user: Signer<'info>,
}

#[account]
pub struct Case_356State {
    pub owner: Pubkey,
    pub count: u64,
    pub flag: bool,
}

#[error_code]
pub enum CustomError {
    #[msg("Not authorized to modify.")]
    NotAuthorized,
}
