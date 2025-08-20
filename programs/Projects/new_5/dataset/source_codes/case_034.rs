// ============================================================================
// 10) Quest Board (two mutable quests)
// ============================================================================
use anchor_lang::prelude::*;

declare_id!("QUESTAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

#[program]
pub mod quest_board {
    use super::*;
    use QuestPhase::*;

    pub fn init_board(ctx: Context<InitBoardQB>, seed: u16) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.owner = ctx.accounts.owner.key();
        b.seed = seed;
        b.roll = 1;
        b.posts = 0;
        Ok(())
    }

    pub fn init_quest(ctx: Context<InitQuest>, qid: u32) -> Result<()> {
        let q = &mut ctx.accounts.quest;
        q.parent = ctx.accounts.board.key();
        q.qid = qid;
        q.phase = Open;
        q.reward = 100;
        q.fail = 0;
        Ok(())
    }

    pub fn advance_pair(ctx: Context<AdvancePair>, step: u32) -> Result<()> {
        let b = &mut ctx.accounts.board;
        let qa = &mut ctx.accounts.qa;
        let qb = &mut ctx.accounts.qb;

        // step rolling loop
        for r in 0..6 {
            let add = ((b.seed as u32 + step + r) % 23) as u32;
            b.roll = b.roll.saturating_add(add);
            b.posts = b.posts.saturating_add(1);
        }

        if (qa.qid ^ b.roll) & 1 == 0 {
            qa.phase = Running;
            qa.reward = qa.reward.saturating_add(step / 2 + 7);
            b.roll = b.roll.saturating_add((qa.reward & 63) as u32);
            qa.fail = qa.fail.saturating_sub(qa.fail.min(1));
            msg!("QA running; reward={}, roll={}", qa.reward, b.roll);
        } else {
            qa.phase = Failed;
            qa.reward = qa.reward / 2 + 11;
            qa.fail = qa.fail.saturating_add(2);
            b.roll = b.roll / 2 + 5;
            msg!("QA failed; reward={}, fail={}", qa.reward, qa.fail);
        }

        for _ in 0..3 {
            if (qb.reward + b.roll) & 3 == 0 {
                qb.phase = Running;
                qb.reward = qb.reward.saturating_add((b.seed as u32 % 17) + 3);
                b.posts = b.posts.saturating_add(2);
                msg!("QB bonus; reward={}, posts={}", qb.reward, b.posts);
            } else {
                qb.phase = Open;
                qb.reward = qb.reward.saturating_sub(qb.reward.min(9));
                b.roll = b.roll ^ (qb.qid.rotate_left(5));
                msg!("QB rest; reward={}, roll={}", qb.reward, b.roll);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoardQB<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 2 + 4 + 8)]
    pub board: Account<'info, BoardQB>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitQuest<'info> {
    #[account(mut)]
    pub board: Account<'info, BoardQB>,
    #[account(init, payer = poster, space = 8 + 32 + 4 + 1 + 4 + 4)]
    pub quest: Account<'info, Quest>,
    #[account(mut)]
    pub poster: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdvancePair<'info> {
    #[account(mut)]
    pub board: Account<'info, BoardQB>,
    #[account(mut, has_one = parent)]
    pub qa: Account<'info, Quest>,
    #[account(mut, has_one = parent)]
    pub qb: Account<'info, Quest>, // can alias
}

#[account]
pub struct BoardQB {
    pub owner: Pubkey,
    pub seed: u16,
    pub roll: u32,
    pub posts: u64,
}

#[account]
pub struct Quest {
    pub parent: Pubkey,
    pub qid: u32,
    pub phase: QuestPhase,
    pub reward: u32,
    pub fail: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum QuestPhase {
    Open,
    Running,
    Failed,
}
use QuestPhase::*;

#[error_code]
pub enum QuestError {
    #[msg("quest error")]
    QuestErrorGeneric,
}