use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount, Mint};

declare_id!("Label8Reward6Pt2Qm4Lx6Zp8Vt1Na3Ur5Hs9Kd908");

#[program]
pub mod labeling_reward_v1 {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, daily_cap_input: u64, base_reward_input: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.owner = ctx.accounts.owner.key();
        p.daily_cap = daily_cap_input;
        if p.daily_cap < 50 { p.daily_cap = 50; }
        p.base_reward = base_reward_input;
        if p.base_reward < 1 { p.base_reward = 1; }
        p.issued_today = 0;
        p.tier = 1;
        Ok(())
    }

    pub fn act_reward(ctx: Context<ActReward>, tasks: u16, quality_bits: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;

        // 品質ビット評価
        let mut score = 0u64;
        let mut idx: u8 = 0;
        while idx < 16 {
            let bit = (quality_bits >> idx) & 1;
            if bit == 1 { score = score + (idx as u64 + 1); }
            idx = idx + 1;
        }

        // 作業量係数
        let mut work = p.base_reward + (tasks as u64 / 3) + score / 5;
        if p.tier >= 3 { work = work + 2; }
        if p.tier >= 5 { work = work + 3; }

        let next = p.issued_today + work;
        if next > p.daily_cap {
            let rest = p.daily_cap - p.issued_today;
            if rest > 0 { token::mint_to(ctx.accounts.mint_ctx(), rest)?; }
            p.issued_today = p.daily_cap;
            p.tier = 1;
            return Err(LabErr::Daily.into());
        }

        token::mint_to(ctx.accounts.mint_ctx(), work)?;
        p.issued_today = next;
        p.tier = p.tier + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub pool: Account<'info, LabelPool>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActReward<'info> {
    #[account(mut, has_one = owner)]
    pub pool: Account<'info, LabelPool>,
    pub owner: Signer<'info>,

    pub reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub worker_reward_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActReward<'info> {
    pub fn mint_ctx(&self)->CpiContext<'_, '_, '_, 'info, MintTo<'info>>{
        let m=MintTo{mint:self.reward_mint.to_account_info(),to:self.worker_reward_vault.to_account_info(),authority:self.owner.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),m)
    }
}
#[account]
pub struct LabelPool {
    pub owner: Pubkey,
    pub daily_cap: u64,
    pub base_reward: u64,
    pub issued_today: u64,
    pub tier: u64,
}
#[error_code]
pub enum LabErr{ #[msg("daily cap reached")] Daily }
