use anchor_lang::prelude::*;

declare_id!("QChain04444444444444444444444444444444444");

#[program]
pub mod quest_chain {
    use super::*;

    pub fn init_chain(ctx: Context<InitChain>, ids: [u64; 5]) -> Result<()> {
        let qc = &mut ctx.accounts.chain;
        qc.chain = ids;
        qc.index = 0;
        qc.completions = [0u8; 5];
        Ok(())
    }

    pub fn complete_current(ctx: Context<ModifyChain>) -> Result<()> {
        let qc = &mut ctx.accounts.chain;
        let idx = qc.index as usize;
        qc.completions[idx] = 1;
        qc.index = (qc.index + 1).min(4);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitChain<'info> {
    #[account(init, payer = user, space = 8 + 8*5 + 1 + 5)]
    pub chain: Account<'info, ChainData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyChain<'info> {
    #[account(mut)]
    pub chain: Account<'info, ChainData>,
}

#[account]
pub struct ChainData {
    pub chain: [u64; 5],
    pub index: u8,
    pub completions: [u8; 5],
}
