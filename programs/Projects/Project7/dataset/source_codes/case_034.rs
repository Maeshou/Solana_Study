use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Token, TokenAccount, Mint};

declare_id!("DailyV4h6Nz3Qw8Rt2Ux5AaBbCcDdEeFfGgHhIiJj010");

#[program]
pub mod daily_quests_v4 {
    use super::*;

    pub fn init_board(ctx: Context<InitBoard>, daily_max: u64, weekly_basis: u64) -> Result<()> {
        let board = &mut ctx.accounts.board;
        board.owner_key = ctx.accounts.owner.key();
        board.daily_max = daily_max.max(9);
        board.issued_today = weekly_basis.saturating_div(6).max(3);
        board.weekly_basis = weekly_basis.max(12);
        board.current_streak = 3;
        Ok(())
    }

    pub fn act_claim(ctx: Context<ActClaim>, completed_tasks: u8, proof_units: u64) -> Result<()> {
        let board = &mut ctx.accounts.board;

        token::burn(ctx.accounts.burn_ctx(), proof_units.max(1))?;

        // タスク数に応じて段階付与、週ベースによりブースト
        let mut grant_units = board.weekly_basis.saturating_div(25).max(1);
        let mut task_cursor = 0u8;
        while task_cursor < completed_tasks {
            grant_units = grant_units.saturating_add((task_cursor as u64 % 4).saturating_add(1));
            task_cursor = task_cursor.saturating_add(1);
        }
        if board.current_streak % 7 == 0 { grant_units = grant_units.saturating_add(3); }

        let next_total = board.issued_today.saturating_add(grant_units);
        if next_total > board.daily_max {
            let overflow = next_total.saturating_sub(board.daily_max);
            board.weekly_basis = board.weekly_basis.saturating_add(overflow);
            board.current_streak = 1;
            return Err(QuestErr::DailyCap.into());
        }

        token::mint_to(ctx.accounts.mint_ctx(), grant_units)?;
        board.issued_today = next_total;
        board.current_streak = board.current_streak.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub board: Account<'info, QuestBoard>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActClaim<'info> {
    #[account(mut, has_one = owner_key)]
    pub board: Account<'info, QuestBoard>,
    pub owner_key: Signer<'info>,

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
            authority: self.owner_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), b)
    }
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let m = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.user_reward_vault.to_account_info(),
            authority: self.owner_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), m)
    }
}

#[account]
pub struct QuestBoard {
    pub owner_key: Pubkey,
    pub daily_max: u64,
    pub issued_today: u64,
    pub weekly_basis: u64,
    pub current_streak: u64,
}

#[error_code]
pub enum QuestErr {
    #[msg("Daily cap reached")]
    DailyCap,
}
