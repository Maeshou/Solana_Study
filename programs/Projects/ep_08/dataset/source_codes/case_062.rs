use anchor_lang::prelude::*;

declare_id!("DIV062062062062062062062062062062");

#[program]
pub mod case_062 {
    use super::*;

    pub fn set_flag(ctx: Context<Flag062>, bump: u8) -> Result<()> {
        let seed = &[b"gamma", bump.to_le_bytes().as_ref()];
        let (_pda, _) = Pubkey::find_program_address(seed, ctx.program_id);
        ctx.accounts.flag_data.active = true;
        ctx.accounts.flag_data.note = bump.to_string();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Flag062<'info> {
    #[account(mut)] pub signer: Signer<'info>,
    #[account(init, payer = signer, seeds = [b"gamma", bump.to_le_bytes().as_ref()], bump)]
    pub flag_data: Account<'info, FlagData062>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FlagData062 {
    pub active: bool,
    pub note: String,
}
