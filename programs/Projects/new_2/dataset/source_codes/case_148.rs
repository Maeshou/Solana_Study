use anchor_lang::prelude::*;

declare_id!("VulnQuest6666666666666666666666666666666666");

#[program]
pub mod vuln_quest {
    pub fn init_quest(ctx: Context<Init>, id: u64) -> Result<()> {
        // creator の検証がないまま初期化
        let q = &mut ctx.accounts.quest;
        q.id        = id;
        q.created_by = ctx.accounts.user.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32)]
    pub quest: Account<'info, QuestData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct QuestData {
    pub id: u64,
    pub created_by: Pubkey,
}
