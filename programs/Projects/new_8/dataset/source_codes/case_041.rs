use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
declare_id!("RunEForgeBBBBB22222222222222222222222222");

#[program]
pub mod rune_forge_b {
    use super::*;

    pub fn init(ctx: Context<Init>, heat: u32) -> Result<()> {
        let f = &mut ctx.accounts.forge;
        f.owner = ctx.accounts.mason.key();
        f.heat = heat % 90 + 6;
        f.queue = heat / 5 + 3;
        f.yielded = 7;
        Ok(())
    }

    // 並び: if → while → PDA検証
    pub fn smelt(ctx: Context<Smelt>, ratio: u16, user_bump: u8) -> Result<()> {
        let f = &mut ctx.accounts.forge;

        if ratio > 20 { f.heat = f.heat.saturating_add((ratio as u32) % 11 + 2); }

        let mut t = 2u32;
        while t < (ratio as u32 % 25 + 4) {
            f.yielded = f.yielded.saturating_add(t);
            t = t.saturating_add(5);
        }

        let seeds = &[b"slag_bin", ctx.accounts.mason.key.as_ref(), &[user_bump]];
        let addr = Pubkey::create_program_address(seeds, ctx.program_id).map_err(|_| error!(ForgeErr::Seed))?;
        if addr != ctx.accounts.slag_bin.key() { return Err(error!(ForgeErr::SlagKey)); }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = mason, space = 8 + 32 + 4 + 4 + 4,
        seeds=[b"forge", mason.key().as_ref()], bump)]
    pub forge: Account<'info, Forge>,
    #[account(mut)]
    pub mason: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Smelt<'info> {
    #[account(mut, seeds=[b"forge", mason.key().as_ref()], bump)]
    pub forge: Account<'info, Forge>,
    /// CHECK
    pub slag_bin: AccountInfo<'info>,
    pub mason: Signer<'info>,
}
#[account] pub struct Forge { pub owner: Pubkey, pub heat: u32, pub queue: u32, pub yielded: u32 }
#[error_code] pub enum ForgeErr { #[msg("seed error")] Seed, #[msg("slag key mismatch")] SlagKey }
