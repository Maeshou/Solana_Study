use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("En3rgyForgeB2222222222222222222222222222");

#[program]
pub mod energy_forge_b {
    use super::*;

    pub fn boot_forge(ctx: Context<BootForge>, initial_heat: u32) -> Result<()> {
        let forge = &mut ctx.accounts.forge;
        forge.admin = ctx.accounts.admin.key();
        forge.heat = initial_heat % 50 + 5;
        forge.output = initial_heat / 3 + 4;
        forge.sent = 7;
        if forge.output < 3 { forge.output = 3; }
        Ok(())
    }

    // 手動 bump を別PDA fuel_cell に使用
    pub fn pump(ctx: Context<Pump>, portion: u16, user_bump: u8) -> Result<()> {
        let forge = &mut ctx.accounts.forge;

        let seeds = &[b"fuel_cell", ctx.accounts.admin.key.as_ref(), &[user_bump]];
        let expect = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(ForgeErr::SeedCalc))?;
        if expect != ctx.accounts.fuel_cell.key() {
            return Err(error!(ForgeErr::FuelMismatch));
        }

        let mut rounds = portion as u32;
        if rounds < 3 { rounds = 3; }
        if rounds > 20 { rounds = 20; }

        let mut stepper = 1u32;
        while stepper < rounds {
            forge.heat = forge.heat.saturating_add(stepper);
            if forge.heat % 4 != 2 { forge.output = forge.output.saturating_add(1); }
            stepper = stepper.saturating_add(3);
        }
        forge.sent = forge.sent.saturating_add(5);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BootForge<'info> {
    #[account(
        init, payer = admin, space = 8 + 32 + 4 + 4 + 4,
        seeds=[b"forge", admin.key().as_ref()], bump
    )]
    pub forge: Account<'info, Forge>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Pump<'info> {
    #[account(
        mut,
        seeds=[b"forge", admin.key().as_ref()], bump
    )]
    pub forge: Account<'info, Forge>,
    /// CHECK: 手動 bump の別PDA
    pub fuel_cell: AccountInfo<'info>,
    pub admin: Signer<'info>,
}

#[account]
pub struct Forge {
    pub admin: Pubkey,
    pub heat: u32,
    pub output: u32,
    pub sent: u32,
}

#[error_code]
pub enum ForgeErr {
    #[msg("seed calculation failed")]
    SeedCalc,
    #[msg("fuel cell key mismatch")]
    FuelMismatch,
}
