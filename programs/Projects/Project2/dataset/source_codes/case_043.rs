use anchor_lang::prelude::*;

declare_id!("VaultSeed33333333333333333333333333333333");

#[program]
pub mod vault_seed {
    use super::*;

    pub fn init_pda(ctx: Context<InitPda>, amount: u64) -> Result<()> {
        let d = &mut ctx.accounts.pda_data;
        d.count = amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPda<'info> {
    #[account(init, payer = user, seeds = [b"vault", user.key().as_ref()], bump, space = 8 + 8)]
    pub pda_data: Account<'info, PdaData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PdaData {
    pub count: u64,
}
