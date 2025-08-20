use anchor_lang::prelude::*;

declare_id!("DIV087087087087087087087087087087");

#[program]
pub mod case_087 {
    use super::*;

    pub fn set_flag(ctx: Context<Flag087>, bump: u8) -> Result<()> {
        let seed = &[b"gamma", bump.to_le_bytes().as_ref()];
        let (_pda, _) = Pubkey::find_program_address(seed, ctx.program_id);
        ctx.accounts.flag_data.active = true;
        ctx.accounts.flag_data.note = bump.to_string();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Flag087<'info> {
    #[account(mut)] pub signer: Signer<'info>,
    #[account(init, payer = signer, seeds = [b"gamma", bump.to_le_bytes().as_ref()], bump)]
    pub flag_data: Account<'info, FlagData087>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FlagData087 {
    pub active: bool,
    pub note: String,
}
