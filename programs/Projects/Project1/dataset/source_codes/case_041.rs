use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA29mvTWf");

#[program]
pub mod nft_player_xp_003 {
    use super::*;

    pub fn gain_xp(ctx: Context<Ctx003>, earned: u64) -> Result<()> {
        let player = &mut ctx.accounts.player_data;
        player.xp += earned;
        Ok(())
    }

    pub fn show_xp(ctx: Context<Ctx003>) -> Result<()> {
        msg!("XP: {}", ctx.accounts.player_data.xp);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = player)]
    pub player_data: Account<'info, PlayerData>,
    #[account(signer)]
    pub player: Signer<'info>,
}

#[account]
pub struct PlayerData {
    pub player: Pubkey,
    pub xp: u64,
}
