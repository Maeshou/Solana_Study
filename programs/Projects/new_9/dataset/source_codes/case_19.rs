use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("NFTGameVuln7777777777777777777777777777777");

#[program]
pub mod tournament_prize_revival_demo {
    use super::*;

    pub fn distribute_tournament_prizes(ctx: Context<DistributeTournamentPrizes>) -> Result<()> {
        // トーナメントの賞金を勝者に分配
        Ok(())
    }

    pub fn restart_tournament_same_tx(
        ctx: Context<RestartTournamentSameTx>,
        prize_pool_size: u64,
        entry_fee: u32,
    ) -> Result<()> {
        let tournament_account = ctx.accounts.tournament_prize_addr.to_account_info();
        let tournament_organizer = ctx.accounts.tournament_organizer.to_account_info();

        let mut accumulated_fees = 0u64;
        let mut participant_count = 1u32;
        while accumulated_fees < prize_pool_size {
            let fee_amount = entry_fee as u64 * participant_count as u64;
            accumulated_fees += fee_amount;
            participant_count += 1;
        }

        let restart_funding = system_instruction::transfer(
            &tournament_organizer.key(),
            &tournament_account.key(),
            accumulated_fees
        );
        anchor_lang::solana_program::program::invoke(
            &restart_funding,
            &[tournament_organizer.clone(), tournament_account.clone()],
        )?;

        let setup_tournament_data = system_instruction::allocate(&tournament_account.key(), prize_pool_size);
        anchor_lang::solana_program::program::invoke(
            &setup_tournament_data,
            &[tournament_account.clone()]
        )?;

        let manage_tournament = system_instruction::assign(&tournament_account.key(), &crate::id());
        anchor_lang::solana_program::program::invoke(
            &manage_tournament,
            &[tournament_account.clone()]
        )?;

        let mut tournament_data = tournament_account.try_borrow_mut_data()?;
        let fee_bytes = entry_fee.to_ne_bytes();
        let count_bytes = participant_count.to_ne_bytes();
        
        tournament_data[..4].copy_from_slice(&fee_bytes);
        tournament_data[4..8].copy_from_slice(&count_bytes);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DistributeTournamentPrizes<'info> {
    #[account(mut, close = champion_wallet)]
    pub tournament_prize: Account<'info, TournamentPrizeData>,
    #[account(mut)]
    pub champion_wallet: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct RestartTournamentSameTx<'info> {
    #[account(mut)]
    pub tournament_prize_addr: UncheckedAccount<'info>,
    #[account(mut)]
    pub tournament_organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TournamentPrizeData {
    pub prize_amount: u64,
    pub winner_count: u16,
}