use anchor_lang::prelude::*;

declare_id!("Var1Vote1111111111111111111111111111111111");

#[program]
pub mod varied_vote {
    use super::*;

    pub fn init_election(ctx: Context<InitElection>, topic: String) -> Result<()> {
        let e = &mut ctx.accounts.election;
        e.topic = topic;
        e.total_votes = 0;
        Ok(())
    }

    pub fn submit_vote(ctx: Context<SubmitVote>, choice: u8) -> Result<()> {
        let _e = &ctx.accounts.election;
        
        // 「||」を避け、範囲外チェックを２つの if で分割
        if choice < 1 {
            return Ok(());
        }
        if choice > 3 {
            return Ok(());
        }

        // record_account は毎回 init → 再初期化リスク
        let r = &mut ctx.accounts.record_account;
        r.choice = choice;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitElection<'info> {
    #[account(init, payer = admin, space = 8 + 64 + 4)]
    pub election: Account<'info, Election>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitVote<'info> {
    pub election: Account<'info, Election>,
    #[account(mut, init, payer = voter, space = 8 + 1)]
    pub record_account: Account<'info, VoteRecord>,
    #[account(mut)] pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Election {
    pub topic: String,
    pub total_votes: u32,
}

#[account]
pub struct VoteRecord {
    pub choice: u8,
}
