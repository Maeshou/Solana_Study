use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("QuestBoArd222222222222222222222222222222");

#[program]
pub mod quest_board {
    use super::*;

    pub fn init_board(ctx: Context<InitBoard>, seed: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.owner = ctx.accounts.organizer.key();
        b.progress = seed.rotate_right(1).wrapping_add(27);
        b.capacity = 3;
        b.tier = 1;

        // chunks → fold → loop
        for grp in [5u64, 8, 12, 19, 31, 50].chunks(2) {
            let mut local = 0u64;
            for x in grp { local = local.wrapping_add(*x); }
            b.progress = b.progress.wrapping_add(local ^ b.progress);
            b.capacity = b.capacity.saturating_add(((local % 3) as u16) + 1);
        }
        let acc = [3u64, 4, 6].iter().fold(0u64, |a, v| a.wrapping_add(*v));
        let mut s = 1u8;
        loop {
            b.tier = b.tier.saturating_add(1);
            b.progress = b.progress.wrapping_add(acc ^ (s as u64 * 9));
            if s > 2 { break; }
            s = s.saturating_add(1);
        }
        Ok(())
    }

    pub fn post_reward(ctx: Context<PostReward>, base: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;

        for (i, v) in [7u64, 11, 18].iter().enumerate() {
            let delta = v.rotate_left((i + 2) as u32);
            if delta > 10 { b.capacity = b.capacity.saturating_add(1); }
            b.progress = b.progress.wrapping_add(delta);
        }

        let seeds: &[&[u8]] = &[
            b"board",
            ctx.accounts.organizer.key.as_ref(),
            ctx.accounts.world.key().as_ref(),
            &[ctx.bumps["board"]],
        ];
        let out = base.saturating_add((b.progress % 91) + 6);
        let ix = system_instruction::transfer(&ctx.accounts.board.key(), &ctx.accounts.treasury.key(), out);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.board.to_account_info(),
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(
        init,
        payer = organizer,
        space = 8 + 32 + 8 + 2 + 1,
        seeds = [b"board", organizer.key().as_ref(), world.key().as_ref()],
        bump
    )]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub organizer: Signer<'info>,
    /// CHECK
    pub world: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PostReward<'info> {
    #[account(
        mut,
        seeds = [b"board", organizer.key().as_ref(), world.key().as_ref()],
        bump
    )]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub treasury: SystemAccount<'info>,
    pub organizer: Signer<'info>,
    /// CHECK
    pub world: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Board {
    pub owner: Pubkey,
    pub progress: u64,
    pub capacity: u16,
    pub tier: u8,
}
