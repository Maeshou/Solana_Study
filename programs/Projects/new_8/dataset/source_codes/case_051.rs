use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
declare_id!("HarVEstFestHHHH9999999999999999999999999");

#[program]
pub mod harvest_fest_h {
    use super::*;

    pub fn start_fest(ctx: Context<StartFest>, plots: u16) -> Result<()> {
        let f = &mut ctx.accounts.fest;
        f.owner = ctx.accounts.host.key();
        f.plots = plots % 90 + 9;
        f.crates = 5;
        f.guests = 2;
        Ok(())
    }

    pub fn gather(ctx: Context<Gather>, amount: u16, user_bump: u8) -> Result<()> {
        let f = &mut ctx.accounts.fest;

        // 1) if（長め）
        if amount > 25 {
            let gain = (amount % 11) as u32 + 3;
            f.crates = f.crates.saturating_add(gain);
            let head = f.owner.to_bytes()[2];
            f.guests = f.guests.saturating_add(head as u32 % 5 + 1);
        }

        // 2) PDA検証
        let seeds = &[b"storage_bin", ctx.accounts.host.key.as_ref(), &[user_bump]];
        let s = Pubkey::create_program_address(seeds, ctx.program_id).map_err(|_| error!(FestErr::SeedIssue))?;
        if s != ctx.accounts.storage_bin.key() { return Err(error!(FestErr::StorageKey)); }

        // 3) while（長め）
        let mut loopc = 1u32;
        while loopc < (amount as u32 % 27 + 6) {
            f.plots = f.plots.saturating_add(1);
            f.crates = f.crates.saturating_add(loopc);
            let push = (f.crates % 7) + 2;
            f.guests = f.guests.saturating_add(push);
            loopc = loopc.saturating_add(5);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartFest<'info> {
    #[account(init, payer = host, space = 8 + 32 + 2 + 4 + 4,
        seeds=[b"fest", host.key().as_ref()], bump)]
    pub fest: Account<'info, Fest>,
    #[account(mut)]
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Gather<'info> {
    #[account(mut, seeds=[b"fest", host.key().as_ref()], bump)]
    pub fest: Account<'info, Fest>,
    /// CHECK
    pub storage_bin: AccountInfo<'info>,
    pub host: Signer<'info>,
}
#[account] pub struct Fest { pub owner: Pubkey, pub plots: u16, pub crates: u32, pub guests: u32 }
#[error_code] pub enum FestErr { #[msg("seed issue")] SeedIssue, #[msg("storage key mismatch")] StorageKey }
