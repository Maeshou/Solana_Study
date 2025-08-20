use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount, Mint};

declare_id!("Que10stChainR5A2L9Z3X6V1N8C4M7R0T2Y1K010");

#[program]
pub mod quest_chain_v1 {
    use super::*;

    pub fn init_chain(ctx: Context<InitChain>, base_reward: u64, chain_length: u8) -> Result<()> {
        let ch = &mut ctx.accounts.chain;
        ch.admin = ctx.accounts.admin.key();
        ch.base_reward = if base_reward < 2 { 2 } else { base_reward };
        ch.chain_length = if chain_length < 3 { 3 } else { chain_length };
        ch.completed_steps = 0;
        ch.total_issued = base_reward / 2 + 1;
        ch.mode = ChainMode::Standard;
        Ok(())
    }

    pub fn act_progress(ctx: Context<ActProgress>, steps_done: u8) -> Result<()> {
        let ch = &mut ctx.accounts.chain;

        // ステップごと増分
        let mut add_units = 0u64;
        let mut s: u8 = 0;
        while s < steps_done {
            add_units = add_units + ((s as u64 % 3) + 1);
            s = s + 1;
        }

        // モード補正
        let mut grant = ch.base_reward + add_units;
        if ch.mode == ChainMode::Hard { grant = grant + grant / 4; }
        if ch.mode == ChainMode::Easy { grant = grant - grant / 6; }

        // 完了時ボーナス
        let next_steps = ch.completed_steps + steps_done as u64;
        if next_steps >= ch.chain_length as u64 {
            grant = grant + 5;
            ch.completed_steps = 0;
        }
        if next_steps < ch.chain_length as u64 {
            ch.completed_steps = next_steps;
        }

        token::mint_to(ctx.accounts.mint_ctx(), grant)?;
        ch.total_issued = ch.total_issued + grant;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitChain<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 1 + 8 + 1)]
    pub chain: Account<'info, ChainState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActProgress<'info> {
    #[account(mut, has_one = admin)]
    pub chain: Account<'info, ChainState>,
    pub admin: Signer<'info>,

    pub reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub player_reward_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActProgress<'info> {
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let m = MintTo { mint: self.reward_mint.to_account_info(), to: self.player_reward_vault.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), m)
    }
}

#[account]
pub struct ChainState {
    pub admin: Pubkey,
    pub base_reward: u64,
    pub chain_length: u8,
    pub completed_steps: u64,
    pub total_issued: u64,
    pub mode: ChainMode,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ChainMode { Easy, Standard, Hard }
