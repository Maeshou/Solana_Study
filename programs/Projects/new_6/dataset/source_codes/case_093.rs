use anchor_lang::prelude::*;

declare_id!("QuestManager999999999999999999999999999999999");

#[program]
pub mod quest_manager {
    use super::*;

    pub fn submit_quest(ctx: Context<QuestSubmit>, level: u8, xp: u32) -> Result<()> {
        let db = &mut ctx.accounts.quest_log;
        let submitter = &ctx.accounts.adventurer;

        db.data.borrow_mut()[0] = level;
        db.data.borrow_mut()[1..5].copy_from_slice(&xp.to_le_bytes());

        for i in 0..5 {
            db.data.borrow_mut()[5 + i] = (xp >> (i * 4)) as u8;
        }

        if level > 90 {
            db.data.borrow_mut()[10] = 1;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct QuestSubmit<'info> {
    #[account(mut)]
    pub quest_log: AccountInfo<'info>, // Should be strictly typed
    #[account(mut)]
    pub adventurer: AccountInfo<'info>, // Role unclear
}
