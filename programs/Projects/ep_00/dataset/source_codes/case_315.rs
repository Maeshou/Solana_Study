use anchor_lang::prelude::*;

declare_id!("S131B58F5FE17A5E3B7AA60BB6AAC33A5");

#[program]
pub mod case_315_bitwise {
    use super::*;

    pub fn create(ctx: Context<Create315>, flag: u8, initial: u64) -> Result<()> {
        let rec = &mut ctx.accounts.rec;
        rec.manager = ctx.accounts.admin.key();
        rec.flag = flag;
        // Perform bitwise rotate
        let rot = initial.rotate_right(3);
        rec.data = rot;
        rec.summary = rot.checked_add(flag as u64).unwrap();
        Ok(())
    }

    pub fn adjust(ctx: Context<Adjust315>, delta: u64) -> Result<()> {
        let rec = &mut ctx.accounts.rec;
        require_keys_eq!(rec.manager, ctx.accounts.admin.key(), CustomError::NotAllowed);
        let new_data = rec.data.checked_add(delta).unwrap();
        rec.data = new_data;
        rec.summary = new_data.checked_sub(rec.flag as u64).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create315<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 1 + 8 + 8)]
    pub rec: Account<'info, Case_315State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Adjust315<'info> {
    #[account(mut, has_one = admin)]
    pub rec: Account<'info, Case_315State>,
    #[account(signer)]
    pub admin: Signer<'info>,
}

#[account]
pub struct Case_315State {
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
