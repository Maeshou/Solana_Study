use anchor_lang::prelude::*;

declare_id!("DIV092092092092092092092092092092");

#[program]
pub mod case_092 {
    use super::*;

    pub fn set_flag(ctx: Context<Flag092>, bump: u8) -> Result<()> {
        let seed = &[b"gamma", bump.to_le_bytes().as_ref()];
        let (_pda, _) = Pubkey::find_program_address(seed, ctx.program_id);
        ctx.accounts.flag_data.active = true;
        ctx.accounts.flag_data.note = bump.to_string();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Flag092<'info> {
    #[account(mut)] pub signer: Signer<'info>,
    #[account(init, payer = signer, seeds = [b"gamma", bump.to_le_bytes().as_ref()], bump)]
    pub flag_data: Account<'info, FlagData092>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FlagData092 {
    pub active: bool,
    pub note: String,
}
