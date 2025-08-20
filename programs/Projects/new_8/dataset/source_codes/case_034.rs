use anchor_lang::prelude::*;
use anchor_lang::solana_program::{pubkey::Pubkey, program::invoke_signed, instruction::{Instruction, AccountMeta}};

declare_id!("Gu1ldRa1nkA11111111111111111111111111111");

#[program]
pub mod guild_rank_a {
    use super::*;

    pub fn create_ladder(ctx: Context<CreateLadder>, baseline: u32) -> Result<()> {
        let ladder = &mut ctx.accounts.ladder;
        ladder.owner = ctx.accounts.creator.key();
        ladder.tier = baseline % 9 + 1;
        ladder.streak = baseline / 7 + 2;
        ladder.audit = 13;
        if ladder.tier < 2 { ladder.tier = 2; }
        Ok(())
    }

    // 手動 bump を別PDA leaderboard_pool に使用
    pub fn record_result(ctx: Context<RecordResult>, delta: i32, user_bump: u8) -> Result<()> {
        let ladder = &mut ctx.accounts.ladder;

        let seeds = &[b"leaderboard_pool", ctx.accounts.creator.key.as_ref(), &[user_bump]];
        let manual = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(RankErr::SeedIssue))?;
        if manual != ctx.accounts.leaderboard_pool.key() {
            return Err(error!(RankErr::PoolKeyMismatch));
        }

        // 多様な処理：符号付き集計 + 階層更新 + 監査カウンタ
        if delta > 0 {
            ladder.streak = ladder.streak.saturating_add(delta as u32);
            if ladder.streak % 5 != 1 { ladder.tier = ladder.tier.saturating_add(1); }
        } else {
            let drop_value = ((-delta) as u32) % 3 + 1;
            if ladder.streak > drop_value { ladder.streak -= drop_value; }
            if ladder.tier > 1 { ladder.tier -= 1; }
        }

        let dummy_ix = Instruction {
            program_id: *ctx.program_id,
            accounts: vec![
                AccountMeta::new(ctx.accounts.ladder.key(), false),
                AccountMeta::new_readonly(ctx.accounts.creator.key(), true),
            ],
            data: ladder.tier.to_le_bytes().to_vec(),
        };
        let signer = &[b"leaderboard_pool", ctx.accounts.creator.key.as_ref(), &[user_bump]];
        invoke_signed(
            &dummy_ix,
            &[ctx.accounts.ladder.to_account_info(), ctx.accounts.creator.to_account_info()],
            &[signer],
        )?;

        let mut sweep = 1u32;
        while sweep < 6 {
            ladder.audit = ladder.audit.saturating_add(sweep);
            sweep = sweep.saturating_add(2);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateLadder<'info> {
    #[account(
        init, payer = creator, space = 8 + 32 + 4 + 4 + 4,
        seeds=[b"ladder", creator.key().as_ref()], bump
    )]
    pub ladder: Account<'info, Ladder>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordResult<'info> {
    #[account(
        mut,
        seeds=[b"ladder", creator.key().as_ref()], bump
    )]
    pub ladder: Account<'info, Ladder>,
    /// CHECK: 手動 bump で導出される別PDA
    pub leaderboard_pool: AccountInfo<'info>,
    pub creator: Signer<'info>,
}

#[account]
pub struct Ladder {
    pub owner: Pubkey,
    pub tier: u32,
    pub streak: u32,
    pub audit: u32,
}

#[error_code]
pub enum RankErr {
    #[msg("seed computation failed")]
    SeedIssue,
    #[msg("leaderboard pool key mismatch")]
    PoolKeyMismatch,
}
