use anchor_lang::prelude::*;

declare_id!("Repertory14Dao111111111111111111111111111111");

#[program]
pub mod dao {
    use super::*;

    // 提案を作成
    pub fn create_proposal(ctx: Context<CreateProposal>, description: String) -> Result<()> {
        let p = &mut ctx.accounts.proposal;
        p.proposer = ctx.accounts.proposer.key();
        p.description = description;
        p.for_votes = 0;
        p.against_votes = 0;
        Ok(())
    }

    // 投票を集計
    pub fn tally(ctx: Context<Tally>, votes: Vec<bool>) -> Result<()> {
        let p = &mut ctx.accounts.proposal;       // ← initなし：既存参照
        let mut for_count = 0u32;
        let mut against_count = 0u32;
        for &v in votes.iter() {
            if v {
                for_count += 1;
            } else {
                against_count += 1;
            }
        }
        p.for_votes = for_count;
        p.against_votes = against_count;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = proposer, space = 8 + 32 + 4 + 200 + 8)]
    pub proposal: Account<'info, ProposalData>,
    #[account(mut)] pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Tally<'info> {
    pub proposal: Account<'info, ProposalData>,
    #[account(mut)] pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProposalData {
    pub proposer: Pubkey,
    pub description: String,
    pub for_votes: u32,
    pub against_votes: u32,
}
