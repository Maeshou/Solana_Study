use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Transfer, Token, TokenAccount, Mint};

declare_id!("Vouch10Redeem8Lm2Qp4Xr6Zt7Na1Ub3Hs5Kd9We910");

#[program]
pub mod voucher_redeem_v1 {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, min_burn_input: u64, max_transfer_input: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.operator = ctx.accounts.operator.key();
        p.min_burn = min_burn_input;
        if p.min_burn < 1 { p.min_burn = 1; }
        p.max_transfer = max_transfer_input;
        if p.max_transfer < p.min_burn { p.max_transfer = p.min_burn + 5; }
        p.redeem_round = 1;
        Ok(())
    }

    pub fn act_redeem(ctx: Context<ActRedeem>, voucher_units: u64, user_level: u8) -> Result<()> {
        let p = &mut ctx.accounts.pool;

        // 焼却量チェックと調整
        let mut burn_units = voucher_units;
        if burn_units < p.min_burn { burn_units = p.min_burn; }
        token::burn(ctx.accounts.burn_ctx(), burn_units)?;

        // レベルによる出力量
        let mut redeem_units = burn_units / 2 + 1;
        let mut i: u8 = 0;
        while i < user_level {
            redeem_units = redeem_units + 1;
            i = i + 1;
        }
        if redeem_units > p.max_transfer { redeem_units = p.max_transfer; }

        token::transfer(ctx.accounts.stable_pool_to_user(), redeem_units)?;
        p.redeem_round = p.redeem_round + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8)]
    pub pool: Account<'info, RedeemPool>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActRedeem<'info> {
    #[account(mut, has_one = operator)]
    pub pool: Account<'info, RedeemPool>,
    pub operator: Signer<'info>,

    pub voucher_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_voucher_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub stable_pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_stable_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActRedeem<'info> {
    pub fn burn_ctx(&self)->CpiContext<'_, '_, '_, 'info, Burn<'info>>{
        let b=Burn{mint:self.voucher_mint.to_account_info(),from:self.user_voucher_vault.to_account_info(),authority:self.operator.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),b)
    }
    pub fn stable_pool_to_user(&self)->CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let t=Transfer{from:self.stable_pool_vault.to_account_info(),to:self.user_stable_vault.to_account_info(),authority:self.operator.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),t)
    }
}
#[account]
pub struct RedeemPool {
    pub operator: Pubkey,
    pub min_burn: u64,
    pub max_transfer: u64,
    pub redeem_round: u64,
}
