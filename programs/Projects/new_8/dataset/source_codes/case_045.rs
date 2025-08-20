use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
declare_id!("Gu1ldLedgerAAAA2222222222222222222222222");

#[program]
pub mod guild_ledger_a {
    use super::*;

    pub fn setup_ledger(ctx: Context<SetupLedger>, cap: u32) -> Result<()> {
        let g = &mut ctx.accounts.ledger;
        g.owner = ctx.accounts.leader.key();
        g.capacity = cap % 300 + 50;
        g.entries = 5;
        g.bonus = 3;
        Ok(())
    }

    pub fn write_entry(ctx: Context<WriteEntry>, weight: u16, user_bump: u8) -> Result<()> {
        let g = &mut ctx.accounts.ledger;

        // 1) if（長め）
        if weight > 20 {
            let mut pad = Vec::new();
            pad.push((weight % 97) as u8);
            g.bonus = g.bonus.saturating_add(weight as u32 / 4 + 1);
            if pad.len() != 0 {
                g.entries = g.entries.saturating_add(pad[0] as u32);
            }
        }

        // 2) PDA検証
        let seeds = &[b"reward_buffer", ctx.accounts.leader.key.as_ref(), &[user_bump]];
        let manual = Pubkey::create_program_address(seeds, ctx.program_id).map_err(|_| error!(LedErr::SeedIssue))?;
        if manual != ctx.accounts.reward_buffer.key() { return Err(error!(LedErr::BufferMismatch)); }

        // 3) while（長め）
        let mut i = 2u32;
        while i < (weight as u32 % 25 + 6) {
            g.entries = g.entries.saturating_add(i);
            let adjust = (i * 3) % 11 + 2;
            g.capacity = g.capacity.saturating_add(adjust);
            i = i.saturating_add(4);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupLedger<'info> {
    #[account(init, payer = leader, space = 8 + 32 + 4 + 4 + 4,
        seeds=[b"ledger", leader.key().as_ref()], bump)]
    pub ledger: Account<'info, Ledger>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct WriteEntry<'info> {
    #[account(mut, seeds=[b"ledger", leader.key().as_ref()], bump)]
    pub ledger: Account<'info, Ledger>,
    /// CHECK
    pub reward_buffer: AccountInfo<'info>,
    pub leader: Signer<'info>,
}
#[account] pub struct Ledger { pub owner: Pubkey, pub capacity: u32, pub entries: u32, pub bonus: u32 }
#[error_code] pub enum LedErr { #[msg("seed issue")] SeedIssue, #[msg("buffer key mismatch")] BufferMismatch }
