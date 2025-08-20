use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Burn, Token, TokenAccount, Mint};

declare_id!("Da1lyQu3stB0nus11111111111111111111111111");

#[program]
pub mod daily_quest_bonus {
    use super::*;
    pub fn init_board(ctx: Context<InitBoard>, max_per_day: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.owner = ctx.accounts.owner.key();
        b.max_per_day = max_per_day;
        b.issued_today = 0;
        b.streak = 0;
        Ok(())
    }

    pub fn act_claim(ctx: Context<ActClaim>, tasks_done: u8, burn_proof: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;

        // burn: 証跡トークンを消費
        token::burn(ctx.accounts.burn_ctx(), burn_proof)?;

        // ループで付与量を加算
        let mut reward = 0u64;
        for _ in 0..tasks_done {
            reward = reward.saturating_add(10);
        }

        if b.issued_today.saturating_add(reward) > b.max_per_day {
            b.streak = 0;
            return Err(ErrorCode::OverDailyLimit.into());
        } else {
            b.streak = b.streak.saturating_add(1);
        }

        // mint_to: ボーナストークン付与
        token::mint_to(ctx.accounts.mint_ctx(), reward)?;
        b.issued_today = b.issued_today.saturating_add(reward);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8)]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActClaim<'info> {
    #[account(mut, has_one = owner)]
    pub board: Account<'info, Board>,
    pub owner: Signer<'info>,

    // burn 用（証跡）
    pub proof_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_proof: Account<'info, TokenAccount>,

    // 後払い（報酬）
    pub bonus_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_bonus: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActClaim<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let accs = Burn {
            mint: self.proof_mint.to_account_info(),
            from: self.user_proof.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let accs = MintTo {
            mint: self.bonus_mint.to_account_info(),
            to: self.user_bonus.to_account_info(),
            authority: self.owner.to_account_info(), // owner が mint authority
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
}

#[account]
pub struct Board {
    pub owner: Pubkey,
    pub max_per_day: u64,
    pub issued_today: u64,
    pub streak: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Over daily limit")]
    OverDailyLimit,
}
