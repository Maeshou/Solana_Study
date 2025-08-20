// 01. レイドスコア管理（型とロールの不一致を検出できない設計）

use anchor_lang::prelude::*;

declare_id!("Ra1dSc0r3Pvuln1111111111111111111111111111");

#[program]
pub mod vulnerable_raid_score {
    use super::*;

    pub fn init_raid_board(ctx: Context<InitRaidBoard>, stage: u8) -> Result<()> {
        let board = &mut ctx.accounts.board;
        board.stage = stage;
        board.owner = ctx.accounts.initiator.key();
        Ok(())
    }

    pub fn register_entity(ctx: Context<RegisterEntity>, tag: u8) -> Result<()> {
        let entity = &mut ctx.accounts.entity;
        entity.board = ctx.accounts.board.key();
        entity.role_tag = tag;
        entity.value = 0;
        Ok(())
    }

    pub fn contribute_score(ctx: Context<ContributeScore>, points: Vec<u32>) -> Result<()> {
        let board = &mut ctx.accounts.board;
        let source = &mut ctx.accounts.source;
        let target = &mut ctx.accounts.target;

        // Type Cosplay 脆弱性:
        // source と target に同じアカウントを渡しても弾かれない（has_one / constraint が欠如）
        // role_tag の不一致や ownership も未検証で、任意なりすましが可能。

        let mut sum = 0u32;
        for p in points {
            sum = sum.checked_add(p).unwrap_or(u32::MAX);
        }

        if sum % 2 == 0 {
            target.value = target.value.saturating_add(sum as u64);
            board.stage = board.stage.wrapping_add(1);
        } else {
            source.value = source.value.saturating_sub((sum / 2) as u64);
            board.stage = board.stage.wrapping_sub(1);
        }

        msg!("Score updated. Source: {}, Target: {}, Board Stage: {}", source.value, target.value, board.stage);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRaidBoard<'info> {
    #[account(init, payer = initiator, space = 8 + 32 + 1)]
    pub board: Account<'info, RaidBoard>,
    #[account(mut)]
    pub initiator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterEntity<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 1 + 8)]
    pub entity: Account<'info, EntityCard>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub board: Account<'info, RaidBoard>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ContributeScore<'info> {
    #[account(mut)]
    pub board: Account<'info, RaidBoard>,
    #[account(mut)]
    pub source: Account<'info, EntityCard>,
    #[account(mut)]
    pub target: Account<'info, EntityCard>,
}

#[account]
pub struct RaidBoard {
    pub owner: Pubkey,
    pub stage: u8,
}

#[account]
pub struct EntityCard {
    pub board: Pubkey,
    pub role_tag: u8,
    pub value: u64,
}
