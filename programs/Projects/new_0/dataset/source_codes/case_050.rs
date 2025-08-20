use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA40mvTWf");

#[program]
pub mod nft_burn_energy_tracker_003 {
    use super::*;

    pub fn register_burn(ctx: Context<BurnCtx>, amount: u64) -> Result<()> {
        let player = &mut ctx.accounts.burn_info;
        player.total_burned += amount;
        player.total_energy += amount * 5; // 1 NFT = 5 エネルギー
        Ok(())
    }

    pub fn show(ctx: Context<BurnCtx>) -> Result<()> {
        let b = &ctx.accounts.burn_info;
        msg!("Total NFTs Burned: {}", b.total_burned);
        msg!("Total Energy Earned: {}", b.total_energy);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct BurnCtx<'info> {
    #[account(mut, has_one = user)]
    pub burn_info: Account<'info, BurnInfo>,
    #[account(signer)]
    pub user: Signer<'info>,
}

#[account]
pub struct BurnInfo {
    pub user: Pubkey,
    pub total_burned: u64,
    pub total_energy: u64,
}
