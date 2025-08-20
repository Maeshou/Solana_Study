use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
declare_id!("BaTtleLadderCCCC444444444444444444444444");

#[program]
pub mod battle_ladder_c {
    use super::*;

    pub fn build_ladder(ctx: Context<BuildLadder>, base: u32) -> Result<()> {
        let b = &mut ctx.accounts.ladder;
        b.owner = ctx.accounts.host.key();
        b.rank = base % 20 + 3;
        b.wins = base / 6 + 5;
        b.loss = 2;
        Ok(())
    }

    pub fn report(ctx: Context<Report>, points: u32, user_bump: u8) -> Result<()> {
        let b = &mut ctx.accounts.ladder;

        // 1) PDA検証
        let seeds = &[b"ladder_bank", ctx.accounts.host.key.as_ref(), &[user_bump]];
        let manual = Pubkey::create_program_address(seeds, ctx.program_id).map_err(|_| error!(LadErr::SeedFail))?;
        if manual != ctx.accounts.ladder_bank.key() { return Err(error!(LadErr::BankKey)); }

        // 2) if（長め）
        if points > 50 {
            let adj = points % 11 + 2;
            b.rank = b.rank.saturating_add(adj);
            let mut tmp = Vec::<u8>::new();
            tmp.push((adj % 7) as u8);
            if tmp.len() != 0 { b.wins = b.wins.saturating_add(tmp[0] as u32); }
        }

        // 3) while（長め）
        let mut round = 3u32;
        while round < (points % 33 + 7) {
            b.wins = b.wins.saturating_add(round);
            b.loss = b.loss.saturating_add((round % 5) + 1);
            round = round.saturating_add(5);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuildLadder<'info> {
    #[account(init, payer = host, space = 8 + 32 + 4 + 4 + 4,
        seeds=[b"ladder", host.key().as_ref()], bump)]
    pub ladder: Account<'info, Ladder>,
    #[account(mut)]
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Report<'info> {
    #[account(mut, seeds=[b"ladder", host.key().as_ref()], bump)]
    pub ladder: Account<'info, Ladder>,
    /// CHECK
    pub ladder_bank: AccountInfo<'info>,
    pub host: Signer<'info>,
}
#[account] pub struct Ladder { pub owner: Pubkey, pub rank: u32, pub wins: u32, pub loss: u32 }
#[error_code] pub enum LadErr { #[msg("seed failed")] SeedFail, #[msg("bank key mismatch")] BankKey }
