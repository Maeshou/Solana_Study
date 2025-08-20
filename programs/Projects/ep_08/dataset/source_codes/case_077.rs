use anchor_lang::prelude::*;

declare_id!("DIV077077077077077077077077077077");

#[program]
pub mod case_077 {
    use super::*;

    pub fn set_flag(ctx: Context<Flag077>, bump: u8) -> Result<()> {
        let seed = &[b"gamma", bump.to_le_bytes().as_ref()];
        let (_pda, _) = Pubkey::find_program_address(seed, ctx.program_id);
        ctx.accounts.flag_data.active = true;
        ctx.accounts.flag_data.note = bump.to_string();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Flag077<'info> {
    #[account(mut)] pub signer: Signer<'info>,
    #[account(init, payer = signer, seeds = [b"gamma", bump.to_le_bytes().as_ref()], bump)]
    pub flag_data: Account<'info, FlagData077>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FlagData077 {
    pub active: bool,
    pub note: String,
}
