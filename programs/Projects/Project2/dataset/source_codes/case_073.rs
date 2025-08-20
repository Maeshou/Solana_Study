use anchor_lang::prelude::*;

declare_id!("VaultSeed33333333333333333333333333333333");

#[program]
pub mod vault_seed {
    use super::*;

    pub fn init_pda(ctx: Context<InitPda>, amount: u64) -> Result<()> {
        let account = &mut ctx.accounts.pda_data;
        account.count = amount;
        account.bump = *ctx.bumps.get("pda_data").unwrap();
        emit!(PdaInitialized {
            creator: ctx.accounts.user.key(),
            amount
        });
        Ok(())
    }

    pub fn increment_pda(ctx: Context<ModifyPda>) -> Result<()> {
        let account = &mut ctx.accounts.pda_data;
        account.count = account.count.checked_add(1).unwrap();
        emit!(PdaIncremented {
            new_count: account.count
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPda<'info> {
    #[account(init, payer = user, seeds = [b"vault", user.key().as_ref()], bump, space = 8 + 8 + 1)]
    pub pda_data: Account<'info, PdaData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyPda<'info> {
    #[account(mut, seeds = [b"vault", user.key().as_ref()], bump)]
    pub pda_data: Account<'info, PdaData>,
    pub user: Signer<'info>,
}

#[account]
pub struct PdaData {
    pub count: u64,
    pub bump: u8,
}

#[event]
pub struct PdaInitialized {
    pub creator: Pubkey,
    pub amount: u64,
}

#[event]
pub struct PdaIncremented {
    pub new_count: u64,
}
