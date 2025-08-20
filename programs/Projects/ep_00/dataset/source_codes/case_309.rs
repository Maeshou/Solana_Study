use anchor_lang::prelude::*;

declare_id!("S81DE90EEF0B3C8E09E6EE24DC30B39C3");

#[program]
pub mod case_309_pda_module {
    use super::*;

    pub fn setup(ctx: Context<Setup309>, bump: u8, base: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        // Assign creator and bump
        state.creator = ctx.accounts.creator.key();
        state.bump = bump;
        // Subtract a constant
        let dec_val = base.checked_sub(2).unwrap();
        state.value = dec_val;
        msg!("PDA setup for {} with value {}", state.creator, state.value);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment309>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        // Ownership check
        require_keys_eq!(state.creator, ctx.accounts.creator.key(), CustomError::InvalidUser);
        // Add a fixed amount
        let add_val = state.value.checked_add(5).unwrap();
        state.value = add_val;
        msg!("Incremented to {}", state.value);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Setup309<'info> {
    #[account(init, payer = creator, seeds = [b"seedcase_309", creator.key().as_ref()], bump, space = 8 + 32 + 1 + 8)]
    pub state: Account<'info, Case_309State>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment309<'info> {
    #[account(mut, has_one = creator)]
    pub state: Account<'info, Case_309State>,
    #[account(signer)]
    pub creator: Signer<'info>,
}

#[account]
pub struct Case_309State {
    pub creator: Pubkey,
    pub bump: u8,
    pub value: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Invalid user.")]
    InvalidUser,
}
