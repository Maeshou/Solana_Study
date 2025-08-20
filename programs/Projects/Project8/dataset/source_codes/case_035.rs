use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("DungeonTreasSafe1111111111111111111111111");

#[program]
pub mod dungeon_treasury_safe {
    use super::*;

    pub fn init_dungeon(ctx: Context<InitDungeon>, shard: u64) -> Result<()> {
        let d = &mut ctx.accounts.dungeon;
        d.warden = ctx.accounts.warden.key();
        d.shard = shard.rotate_left(3).wrapping_add(33);
        d.depth = 1;

        let mut s = d.shard.rotate_right(1).wrapping_add(5);
        for _ in 0..4 {
            s = s.rotate_left(1).wrapping_mul(2).wrapping_add(7);
            d.depth = d.depth.saturating_add(((s % 19) as u32) + 1);
        }
        Ok(())
    }

    pub fn loot_drop(ctx: Context<LootDrop>, lamports: u64) -> Result<()> {
        let ix = system_instruction::transfer(&ctx.accounts.dungeon.key(), &ctx.accounts.adventurer.key(), lamports);

        let bump = *ctx.bumps.get("dungeon").ok_or(error!(DungeonErr::MissingBump))?;
        let seeds: &[&[u8]] = &[
            b"dungeon",
            ctx.accounts.warden.key.as_ref(),
            &ctx.accounts.dungeon.shard.to_le_bytes(),
            &[bump],
        ];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.dungeon.to_account_info(),
                ctx.accounts.adventurer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        emit!(LootPaid { to: ctx.accounts.adventurer.key(), amount: lamports });
        Ok(())
    }
}

#[event]
pub struct LootPaid {
    pub to: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct InitDungeon<'info> {
    #[account(
        init,
        payer = warden,
        space = 8 + 32 + 8 + 4,
        seeds = [b"dungeon", warden.key().as_ref(), shard.to_le_bytes().as_ref()],
        bump
    )]
    pub dungeon: Account<'info, DungeonState>,
    #[account(mut)]
    pub warden: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub shard: u64,
}

#[derive(Accounts)]
pub struct LootDrop<'info> {
    #[account(
        mut,
        seeds = [b"dungeon", warden.key().as_ref(), dungeon.shard.to_le_bytes().as_ref()],
        bump
    )]
    pub dungeon: Account<'info, DungeonState>,
    #[account(mut)]
    pub adventurer: SystemAccount<'info>,
    pub warden: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DungeonState {
    pub warden: Pubkey,
    pub shard: u64,
    pub depth: u32,
}

#[error_code]
pub enum DungeonErr {
    #[msg("missing bump")] MissingBump,
}
