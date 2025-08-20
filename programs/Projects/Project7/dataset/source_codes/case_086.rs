use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Sav9VaultInt5Rk2Qm4Lx6Zp8Vt1Na3Ur5Hs9Kd909");

#[program]
pub mod savings_vault_interest_v1 {
    use super::*;

    pub fn init_vault(ctx: Context<InitVault>, base_apr_bps_input: u16, fee_bps_input: u16) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        v.bank = ctx.accounts.bank.key();
        v.base_apr_bps = clamp_u16(base_apr_bps_input, 100, 2000);
        v.fee_bps = clamp_u16(fee_bps_input, 0, 500);
        v.epoch = 1;
        Ok(())
    }

    pub fn act_distribute(ctx: Context<ActDistribute>, principal_units: u64, epochs: u8) -> Result<()> {
        let v = &mut ctx.accounts.vault;

        // 近似複利（各エポックで1/12年分）
        let mut accrued: u64 = 0;
        let mut e: u8 = 0;
        while e < epochs {
            let year_interest = principal_units * v.base_apr_bps as u64 / 10_000;
            accrued = accrued + year_interest / 12 + e as u64 % 2;
            e = e + 1;
        }

        let fee = accrued * v.fee_bps as u64 / 10_000;
        let net = accrued - fee;

        token::transfer(ctx.accounts.bank_pool_to_user(), net)?;
        token::transfer(ctx.accounts.bank_pool_to_fee(), fee)?;

        v.epoch = v.epoch + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(init, payer = bank, space = 8 + 32 + 2 + 2 + 8)]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub bank: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActDistribute<'info> {
    #[account(mut, has_one = bank)]
    pub vault: Account<'info, VaultState>,
    pub bank: Signer<'info>,

    #[account(mut)]
    pub bank_pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_interest_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActDistribute<'info> {
    pub fn bank_pool_to_user(&self)->CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let t=Transfer{from:self.bank_pool_vault.to_account_info(),to:self.user_interest_vault.to_account_info(),authority:self.bank.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),t)
    }
    pub fn bank_pool_to_fee(&self)->CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let t=Transfer{from:self.bank_pool_vault.to_account_info(),to:self.fee_vault.to_account_info(),authority:self.bank.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),t)
    }
}
#[account]
pub struct VaultState {
    pub bank: Pubkey,
    pub base_apr_bps: u16,
    pub fee_bps: u16,
    pub epoch: u64,
}
fn clamp_u16(v:u16,lo:u16,hi:u16)->u16{let mut o=v;if o<lo{o=lo_
