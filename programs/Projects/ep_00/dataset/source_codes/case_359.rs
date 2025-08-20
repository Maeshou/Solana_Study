use anchor_lang::prelude::*;

declare_id!("S309E08E27250FE1625FEA9211C034946");

#[program]
pub mod case_359_bitwise {
    use super::*;

    pub fn create(ctx: Context<Create359>, flag: u8, initial: u64) -> Result<()> {
        let rec = &mut ctx.accounts.rec;
        rec.manager = ctx.accounts.admin.key();
        rec.flag = flag;
        // Perform bitwise rotate
        let rot = initial.rotate_right(3);
        rec.data = rot;
        rec.summary = rot.checked_add(flag as u64).unwrap();
        Ok(())
    }

    pub fn adjust(ctx: Context<Adjust359>, delta: u64) -> Result<()> {
        let rec = &mut ctx.accounts.rec;
        require_keys_eq!(rec.manager, ctx.accounts.admin.key(), CustomError::NotAllowed);
        let new_data = rec.data.checked_add(delta).unwrap();
        rec.data = new_data;
        rec.summary = new_data.checked_sub(rec.flag as u64).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create359<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 1 + 8 + 8)]
    pub rec: Account<'info, Case_359State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Adjust359<'info> {
    #[account(mut, has_one = admin)]
    pub rec: Account<'info, Case_359State>,
    #[account(signer)]
    pub admin: Signer<'info>,
}

#[account]
pub struct Case_359State {
    pub manager: Pubkey,
    pub flag: u8,
    pub data: u64,
    pub summary: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Not allowed.")]
    NotAllowed,
}
