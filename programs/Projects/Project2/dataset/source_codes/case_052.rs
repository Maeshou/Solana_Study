use anchor_lang::prelude::*;

declare_id!("Gamerz111111111111111111111111111111111111");

#[program]
pub mod game_engine {
    use super::*;
    pub fn submit_score(ctx: Context<UpdateScore>, score: u64) -> Result<()> {
        let player_account = &mut ctx.accounts.player_score;
        if score > player_account.high_score {
            player_account.high_score = score;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateScore<'info> {
    #[account(
        init_if_needed,
        payer = player,
        space = 8 + 32 + 8, // Discriminator + player pubkey + score
        seeds = [b"score", player.key().as_ref()],
        bump
    )]
    pub player_score: Account<'info, PlayerScore>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PlayerScore {
    pub player: Pubkey,
    pub high_score: u64,
}