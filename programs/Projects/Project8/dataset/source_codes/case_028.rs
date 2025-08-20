use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("NfTCraftSafe111111111111111111111111111111");

#[program]
pub mod nft_craft_safe {
    use super::*;

    pub fn initialize_station(ctx: Context<InitializeStation>, recipe: Pubkey, seed: u64) -> Result<()> {
        let station = &mut ctx.accounts.station;
        station.owner = ctx.accounts.crafter.key();
        station.recipe = recipe;
        station.energy = seed.rotate_left(2).wrapping_add(50);
        station.cycles = 3;

        let mut value = station.energy;
        let mut counter = 0u32;
        while value > 100 {
            counter = counter.saturating_add((value % 17) as u32);
            value = value.rotate_right(1).wrapping_sub(13);
            if counter > 20 {
                value = value.wrapping_mul(2).wrapping_add(counter as u64);
                counter = counter.saturating_sub(7);
            }
        }
        station.energy = value;
        station.cycles = station.cycles.saturating_add(counter);
        Ok(())
    }

    pub fn craft_reward(ctx: Context<CraftReward>, token_id: u64, lamports: u64) -> Result<()> {
        let ix = system_instruction::transfer(&ctx.accounts.station.key(), &ctx.accounts.player.key(), lamports);

        let bump = *ctx.bumps.get("station").ok_or(error!(CraftErr::MissingBump))?;
        let seeds: &[&[u8]] = &[
            b"station",
            ctx.accounts.crafter.key.as_ref(),
            ctx.accounts.station.recipe.as_ref(),
            &[bump],
        ];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.station.to_account_info(),
                ctx.accounts.player.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        emit!(CraftCompleted { token_id, to: ctx.accounts.player.key(), amount: lamports });
        Ok(())
    }
}

#[event]
pub struct CraftCompleted {
    pub token_id: u64,
    pub to: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct InitializeStation<'info> {
    #[account(
        init,
        payer = crafter,
        space = 8 + 32 + 32 + 8 + 4,
        seeds = [b"station", crafter.key().as_ref(), recipe.key().as_ref()],
        bump
    )]
    pub station: Account<'info, StationState>,
    #[account(mut)]
    pub crafter: Signer<'info>,
    /// CHECK: 実運用では Mint 検証を推奨
    pub recipe: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CraftReward<'info> {
    #[account(mut, seeds = [b"station", crafter.key().as_ref(), station.recipe.key().as_ref()], bump)]
    pub station: Account<'info, StationState>,
    #[account(mut)]
    pub player: SystemAccount<'info>,
    pub crafter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StationState {
    pub owner: Pubkey,
    pub recipe: Pubkey,
    pub energy: u64,
    pub cycles: u32,
}

#[error_code]
pub enum CraftErr {
    #[msg("missing bump")] MissingBump,
}
