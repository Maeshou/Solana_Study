use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("RoylA05SplitG4nR7Lm3R8tD6W4yZ1nC5bK2hU0T305");

#[program]
pub mod creator_royalty_v1 {
    use super::*;

    pub fn init_router(ctx: Context<InitRouter>, creator_bps: u16, platform_bps: u16) -> Result<()> {
        let router = &mut ctx.accounts.router;
        router.operator = ctx.accounts.operator.key();
        router.creator_bps = creator_bps;
        router.platform_bps = platform_bps;
        router.round = 1;
        router.total_processed = 1;
        Ok(())
    }

    pub fn act_route(ctx: Context<ActRoute>, sale_amount: u64, bonus_flag: bool) -> Result<()> {
        let router = &mut ctx.accounts.router;

        // ボーナスフラグで微補正
        let mut amount = sale_amount;
        if bonus_flag { amount = amount + amount / 20; }

        let creator_cut = (amount as u128 * router.creator_bps as u128 / 10_000u128) as u64;
        let platform_cut = (amount as u128 * router.platform_bps as u128 / 10_000u128) as u64;
        let community_cut = amount - creator_cut - platform_cut;

        token::transfer(ctx.accounts.pool_to_creator(), creator_cut)?;
        token::transfer(ctx.accounts.pool_to_platform(), platform_cut)?;
        token::transfer(ctx.accounts.pool_to_community(), community_cut)?;

        router.total_processed = router.total_processed + amount;
        router.round = router.round + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRouter<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 2 + 2 + 8 + 8)]
    pub router: Account<'info, RouterState>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActRoute<'info> {
    #[account(mut, has_one = operator)]
    pub router: Account<'info, RouterState>,
    pub operator: Signer<'info>,

    #[account(mut)]
    pub sale_pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub creator_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub platform_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub community_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActRoute<'info> {
    pub fn pool_to_creator(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let c = Transfer { from:self.sale_pool_vault.to_account_info(), to:self.creator_vault.to_account_info(), authority:self.operator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
    pub fn pool_to_platform(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let c = Transfer { from:self.sale_pool_vault.to_account_info(), to:self.platform_vault.to_account_info(), authority:self.operator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
    pub fn pool_to_community(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let c = Transfer { from:self.sale_pool_vault.to_account_info(), to:self.community_vault.to_account_info(), authority:self.operator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
}
#[account]
pub struct RouterState {
    pub operator: Pubkey,
    pub creator_bps: u16,
    pub platform_bps: u16,
    pub round: u64,
    pub total_processed: u64,
}
