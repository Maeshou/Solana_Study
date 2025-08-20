use anchor_lang::prelude::*;

declare_id!("Qst02Game0000000000000000000000000000002");

#[program]
pub mod quest_tracker {
    use super::*;

    pub fn init_quest(ctx: Context<InitQuest>, quest_id: u64) -> Result<()> {
        let q = &mut ctx.accounts.quest;
        q.id = quest_id;
        q.completed = false;
        Ok(())
    }

    pub fn complete_quest(ctx: Context<ModifyQuest>) -> Result<()> {
        let q = &mut ctx.accounts.quest;
        q.completed = true;
        Ok(())
    }

    pub fn status(ctx: Context<ViewQuest>) -> Result<QuestStatus> {
        let q = &ctx.accounts.quest;
        Ok(QuestStatus { id: q.id, done: q.completed })
    }
}

#[derive(Accounts)]
pub struct InitQuest<'info> {
    #[account(
        init,
        seeds = [b"quest", user.key().as_ref(), &quest_id.to_le_bytes()],
        bump,
        payer = user,
        space = 8 + 8 + 1
    )]
    pub quest: Account<'info, QuestData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyQuest<'info> {
    #[account(mut, seeds = [b"quest", user.key().as_ref(), &quest.id.to_le_bytes()], bump)]
    pub quest: Account<'info, QuestData>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ViewQuest<'info> {
    pub quest: Account<'info, QuestData>,
}

#[account]
pub struct QuestData {
    pub id: u64,
    pub completed: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct QuestStatus {
    pub id: u64,
    pub done: bool,
}
