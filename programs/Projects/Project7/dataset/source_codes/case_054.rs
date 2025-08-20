use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer, Token, TokenAccount, Mint};

declare_id!("Cb8ackBuyR3wrdT9E2X5M1L7V4N6C0Z3P8Y808");

#[program]
pub mod event_cashback_v1 {
    use super::*;

    pub fn init_program(ctx: Context<InitProgram>, cashback_bps: u16) -> Result<()> {
        let prog = &mut ctx.accounts.program_state;
        prog.operator = ctx.accounts.operator.key();
        prog.cashback_bps = clamp_u16(cashback_bps, 100, 3000);
        prog.loyalty_points = 2;
        Ok(())
    }

    pub fn act_purchase(ctx: Context<ActPurchase>, ticket_price: u64, quantity: u64) -> Result<()> {
        let prog = &mut ctx.accounts.program_state;

        let total = ticket_price * quantity;
        token::transfer(ctx.accounts.buyer_to_merchant(), total)?;

        // ロイヤリティ係数
        let mut factor = 100u64; // %
        let mut loop_i: u8 = 0;
        while loop_i < (prog.loyalty_points % 5) as u8 {
            factor = factor + 5;
            loop_i = loop_i + 1;
        }

        let base_cashback = total * prog.cashback_bps as u64 / 10_000;
        let scaled = (base_cashback as u128 * factor as u128 / 100u128) as u64;

        token::mint_to(ctx.accounts.mint_reward(), scaled)?;
        prog.loyalty_points = prog.loyalty_points + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitProgram<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 2 + 8)]
    pub program_state: Account<'info, CashbackState>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActPurchase<'info> {
    #[account(mut, has_one = operator)]
    pub program_state: Account<'info, CashbackState>,
    pub operator: Signer<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub merchant_vault: Account<'info, TokenAccount>,

    pub reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub buyer_reward_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActPurchase<'info> {
    pub fn buyer_to_merchant(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.buyer_vault.to_account_info(), to: self.merchant_vault.to_account_info(), authority: self.buyer.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    pub fn mint_reward(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let m = MintTo { mint: self.reward_mint.to_account_info(), to: self.buyer_reward_vault.to_account_info(), authority: self.operator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), m)
    }
}

#[account]
pub struct CashbackState {
    pub operator: Pubkey,
    pub cashback_bps: u16,
    pub loyalty_points: u64,
}

fn clamp_u16(v: u16, lo: u16, hi: u16) -> u16 { let mut o=v; if o<lo{o=lo;} if o>hi{o=hi;} o }
