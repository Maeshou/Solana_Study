use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Token, TokenAccount, Mint};

declare_id!("ArenaA08LossP5nN7Lm3R8tD6W4yZ1nC5bK2hU0W308");

#[program]
pub mod arena_loss_settlement_v1 {
    use super::*;

    pub fn init_arena(ctx: Context<InitArena>, penalty_base_input: u64) -> Result<()> {
        let arena = &mut ctx.accounts.arena;
        arena.referee = ctx.accounts.referee.key();
        arena.penalty_base = penalty_base_input;
        if arena.penalty_base < 1 { arena.penalty_base = 1; }
        arena.losses = 0;
        arena.consolation_issued = 0;
        Ok(())
    }

    pub fn act_settle(ctx: Context<ActSettle>, streak_losses: u8, consolation_mint: bool) -> Result<()> {
        let arena = &mut ctx.accounts.arena;

        // 連敗でペナルティ増加
        let mut penalty = arena.penalty_base;
        let mut i: u8 = 0;
        while i < streak_losses {
            penalty = penalty + (arena.penalty_base / 10) + 1;
            i = i + 1;
        }

        token::burn(ctx.accounts.burn_ctx(), penalty)?;
        arena.losses = arena.losses + 1;

        // 慰労ミント
        if consolation_mint {
            let mut consolation = penalty / 5 + 1;
            if streak_losses >= 3 { consolation = consolation + 2; }
            token::mint_to(ctx.accounts.mint_ctx(), consolation)?;
            arena.consolation_issued = arena.consolation_issued + consolation;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitArena<'info> {
    #[account(init, payer = referee, space = 8 + 32 + 8 + 8 + 8)]
    pub arena: Account<'info, ArenaState>,
    #[account(mut)]
    pub referee: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActSettle<'info> {
    #[account(mut, has_one = referee)]
    pub arena: Account<'info, ArenaState>,
    pub referee: Signer<'info>,

    pub penalty_mint: Account<'info, Mint>,
    #[account(mut)]
    pub player_penalty_vault: Account<'info, TokenAccount>,

    pub consolation_mint_acc: Account<'info, Mint>,
    #[account(mut)]
    pub player_consolation_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActSettle<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let b = Burn { mint:self.penalty_mint.to_account_info(), from:self.player_penalty_vault.to_account_info(), authority:self.referee.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), b)
    }
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let m = MintTo { mint:self.consolation_mint_acc.to_account_info(), to:self.player_consolation_vault.to_account_info(), authority:self.referee.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), m)
    }
}
#[account]
pub struct ArenaState {
    pub referee: Pubkey,
    pub penalty_base: u64,
    pub losses: u64,
    pub consolation_issued: u64,
}
