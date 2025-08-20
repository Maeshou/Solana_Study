use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("QuesTChainGGGG7777777777777777777777777");

#[program]
pub mod quest_chain_g {
    use super::*;

    pub fn setup_chain(ctx: Context<SetupChain>, length: u32) -> Result<()> {
        let chain = &mut ctx.accounts.chain;
        chain.owner = ctx.accounts.manager.key();
        chain.length = length % 100 + 10;
        chain.progress = 0;
        chain.bonus = 5;
        Ok(())
    }

    pub fn advance(ctx: Context<Advance>, steps: u32, user_bump: u8) -> Result<()> {
        let chain = &mut ctx.accounts.chain;

        // 1. 先にループ (中身を長めに)
        let mut index = 1u32;
        while index < (steps % 30 + 5) {
            chain.progress = chain.progress.saturating_add(index);
            let mut calc = (index * 2) % 7;
            calc = calc.saturating_add(3);
            chain.bonus = chain.bonus.saturating_add(calc);
            // 追加の演算
            let derived_val = chain.progress.saturating_mul(2) / (calc + 1);
            if derived_val % 3 != 0 {
                chain.bonus = chain.bonus.saturating_add(1);
            }
            index = index.saturating_add(4);
        }

        // 2. PDA検証
        let seeds = &[b"reward_pool", ctx.accounts.manager.key.as_ref(), &[user_bump]];
        let manual = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(ChainErr::SeedProblem))?;
        if manual != ctx.accounts.reward_pool.key() {
            return Err(error!(ChainErr::PoolMismatch));
        }

        // 3. 分岐 (中身を長めに)
        if steps > 15 {
            let extra = steps % 9 + 2;
            chain.bonus = chain.bonus.saturating_add(extra);
            chain.length = chain.length.saturating_add(1);
            let mut buffer = Vec::new();
            buffer.push(chain.owner.to_bytes()[0]);
            buffer.push(extra as u8);
            if buffer.len() != 0 {
                chain.progress = chain.progress.saturating_add(buffer[0] as u32);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupChain<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 4 + 4 + 4,
        seeds=[b"chain", manager.key().as_ref()], bump)]
    pub chain: Account<'info, Chain>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Advance<'info> {
    #[account(mut, seeds=[b"chain", manager.key().as_ref()], bump)]
    pub chain: Account<'info, Chain>,
    /// CHECK: 手動 bump の reward_pool
    pub reward_pool: AccountInfo<'info>,
    pub manager: Signer<'info>,
}

#[account]
pub struct Chain {
    pub owner: Pubkey,
    pub length: u32,
    pub progress: u32,
    pub bonus: u32,
}

#[error_code]
pub enum ChainErr {
    #[msg("seed creation failed")] SeedProblem,
    #[msg("reward pool mismatch")] PoolMismatch,
}
