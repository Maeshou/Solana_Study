use anchor_lang::prelude::*;

declare_id!("DIV032032032032032032032032032032");

#[program]
pub mod case_032 {
    use super::*;

    pub fn set_flag(ctx: Context<Flag032>, bump: u8) -> Result<()> {
        let seed = &[b"gamma", bump.to_le_bytes().as_ref()];
        let (_pda, _) = Pubkey::find_program_address(seed, ctx.program_id);
        ctx.accounts.flag_data.active = true;
        ctx.accounts.flag_data.note = bump.to_string();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Flag032<'info> {
    #[account(mut)] pub signer: Signer<'info>,
    #[account(init, payer = signer, seeds = [b"gamma", bump.to_le_bytes().as_ref()], bump)]
    pub flag_data: Account<'info, FlagData032>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FlagData032 {
    pub active: bool,
    pub note: String,
}
