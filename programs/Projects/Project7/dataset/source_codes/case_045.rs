use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Token, TokenAccount, Mint};

declare_id!("DlyJ10Wq3Er7Tu2Op9Ya4Ls6Kd1Mj8Nb5Vc0J010");

#[program]
pub mod daily_quests_multiplier_v1 {
    use super::*;

    pub fn init_board(ctx: Context<InitBoard>, daily_cap: u64, weekly_base: u64) -> Result<()> {
        let board_state = &mut ctx.accounts.board_state;
        board_state.owner = ctx.accounts.owner.key();
        board_state.daily_cap = if daily_cap < 9 { 9 } else { daily_cap };
        board_state.issued_today = weekly_base / 5 + 2;
        board_state.weekly_base = if weekly_base < 12 { 12 } else { weekly_base };
        board_state.multiplier_percent = 100;
        Ok(())
    }

    pub fn act_claim(ctx: Context<ActClaim>, task_count: u8, proof_units: u64) -> Result<()> {
        let board_state = &mut ctx.accounts.board_state;

        let burn_units = if proof_units < 1 { 1 } else { proof_units };
        token::burn(ctx.accounts.burn_ctx(), burn_units)?;

        // 段階的な倍率上昇（最大130%）
        if task_count >= 3 {
            board_state.multiplier_percent = board_state.multiplier_percent + 5;
        }
        if task_count >= 5 {
            board_state.multiplier_percent = board_state.multiplier_percent + 10;
        }
        if board_state.multiplier_percent > 130 {
            board_state.multiplier_percent = 130;
        }

        // ベース算定（タスクごとに増分を追加）
        let mut base_reward_units: u64 = board_state.weekly_base / 25 + 1;
        let mut task_cursor: u8 = 0;
        while task_cursor < task_count {
            let step_bonus = (task_cursor as u64 % 3) + 1;
            base_reward_units = base_reward_units + step_bonus;
            task_cursor = task_cursor + 1;
        }

        let scaled_reward: u64 = (base_reward_units as u128 * board_state.multiplier_percent as u128 / 100u128) as u64;
        let projected_total: u64 = board_state.issued_today + scaled_reward;

        if projected_total > board_state.daily_cap {
            let carry_over: u64 = projected_total - board_state.daily_cap;
            board_state.weekly_base = board_state.weekly_base + carry_over;
            board_state.multiplier_percent = 100;
            return Err(QuestErr::OverDailyCap.into());
        }

        token::mint_to(ctx.accounts.mint_ctx(), scaled_reward)?;
        board_state.issued_today = projected_total;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 4)]
    pub board_state: Account<'info, BoardState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActClaim<'info> {
    #[account(mut, has_one = owner)]
    pub board_state: Account<'info, BoardState>,
    pub owner: Signer<'info>,

    pub proof_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_proof_vault: Account<'info, TokenAccount>,

    pub reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_reward_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActClaim<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let b = Burn {
            mint: self.proof_mint.to_account_info(),
            from: self.user_proof_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), b)
    }
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let m = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.user_reward_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), m)
    }
}
#[account]
pub struct BoardState {
    pub owner: Pubkey,
    pub daily_cap: u64,
    pub issued_today: u64,
    pub weekly_base: u64,
    pub multiplier_percent: u64,
}
#[error_code]
pub enum QuestErr {
    #[msg("daily cap exceeded")]
    OverDailyCap,
}
