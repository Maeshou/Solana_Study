use anchor_lang::prelude::*;

declare_id!("GuildQuestVuln0033333333333333333333333333");

#[program]
pub mod vulnerable_guild_quest {
    use super::*;

    pub fn init_quest(ctx: Context<InitQuest>, difficulty: u8) -> Result<()> {
        let quest = &mut ctx.accounts.quest;
        quest.giver = ctx.accounts.giver.key();
        quest.difficulty = difficulty;
        quest.result = None;
        Ok(())
    }

    pub fn report_result(ctx: Context<ReportResult>, passed: bool) -> Result<()> {
        let quest = &mut ctx.accounts.quest;
        let actor = &mut ctx.accounts.actor;

        // 脆弱: actor に giver と同じアカウントが渡せる（区別不能）
        if passed {
            quest.result = Some(100 + quest.difficulty as u32);
            actor.exp += 10;
        } else {
            quest.result = Some(0);
            actor.exp = actor.exp.saturating_sub(5);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitQuest<'info> {
    #[account(init, payer = giver, space = 8 + 32 + 1 + 4)]
    pub quest: Account<'info, Quest>,
    #[account(mut)]
    pub giver: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReportResult<'info> {
    #[account(mut)]
    pub quest: Account<'info, Quest>,
    #[account(mut)]
    pub actor: Account<'info, Player>,
}

#[account]
pub struct Quest {
    pub giver: Pubkey,
    pub difficulty: u8,
    pub result: Option<u32>,
}

#[account]
pub struct Player {
    pub exp: u32,
}
