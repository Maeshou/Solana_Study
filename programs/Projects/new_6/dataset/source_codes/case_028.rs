use anchor_lang::prelude::*;

declare_id!("GldChlngEvnt000000000000000000000000000000");

#[program]
pub mod guild_challenge {
    use super::*;

    pub fn create_challenge(ctx: Context<CreateChallenge>, difficulty: u8) -> Result<()> {
        let challenge = &mut ctx.accounts.challenge;
        challenge.creator = ctx.accounts.creator.key();
        challenge.difficulty = difficulty;
        challenge.score = 0;
        Ok(())
    }

    pub fn evaluate(ctx: Context<EvaluateChallenge>, bonus: u32) -> Result<()> {
        let challenge = &mut ctx.accounts.challenge;
        let evaluator = &mut ctx.accounts.evaluator;

        // evaluator と creator が同一でもチェックされない
        if bonus % 2 == 0 {
            challenge.score += bonus;
            evaluator.reputation += 1;
        } else {
            evaluator.reputation = evaluator.reputation.saturating_sub(1);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateChallenge<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 1 + 4)]
    pub challenge: Account<'info, Challenge>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EvaluateChallenge<'info> {
    #[account(mut)]
    pub challenge: Account<'info, Challenge>,
    #[account(mut)]
    pub evaluator: Account<'info, Member>,
}

#[account]
pub struct Challenge {
    pub creator: Pubkey,
    pub difficulty: u8,
    pub score: u32,
}

#[account]
pub struct Member {
    pub reputation: u32,
}
