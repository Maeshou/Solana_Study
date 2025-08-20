use anchor_lang::prelude::*;

declare_id!("DaoProg111111111111111111111111111111111");

#[program]
pub mod dao_manager {
    use super::*;
    pub fn create_proposal(ctx: Context<CreateProposal>, description: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.author = *ctx.accounts.author.key;
        proposal.description = description;
        proposal.votes = 0;
        proposal.dao_config = ctx.accounts.dao_config.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = author, space = 8 + 32 + 4 + 500 + 8 + 32)]
    pub proposal: Account<'info, Proposal>,

    // DAOの設定アカウント。これもプログラムが所有。
    #[account(seeds = [b"dao"], bump)]
    pub dao_config: Account<'info, DaoConfig>,

    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DaoConfig {
    pub name: String,
    pub admin: Pubkey,
}

#[account]
pub struct Proposal {
    pub author: Pubkey,
    pub description: String,
    pub votes: u64,
    pub dao_config: Pubkey,
}