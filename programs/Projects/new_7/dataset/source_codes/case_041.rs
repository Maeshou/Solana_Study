// 5) pda_control_program — PDAの設定値に従って実行先を選ぶ
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("PdaC0ntr0lProg55555555555555555555555555");

#[program]
pub mod pda_control_program {
    use super::*;

    pub fn init(ctx: Context<Init>, id: Pubkey) -> Result<()> {
        let c = &mut ctx.accounts.cfg;
        c.admin = ctx.accounts.admin.key();
        c.target = id;
        c.bump = *ctx.bumps.get("cfg").unwrap();
        c.count = 0;
        Ok(())
    }

    pub fn set_target(ctx: Context<SetTarget>, id: Pubkey) -> Result<()> {
        let c = &mut ctx.accounts.cfg;
        require!(c.admin == ctx.accounts.admin.key(), Errs::Admin);
        c.target = id;
        c.count = c.count.saturating_add(1);
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, base: u64) -> Result<()> {
        let c = &mut ctx.accounts.cfg;
        require!(c.admin == ctx.accounts.admin.key(), Errs::Admin);

        let mut amt = base;
        let mut i = 0u8;
        while i < 6 {
            amt = amt.saturating_add((i as u64) + ((c.count % 9) as u64));
            i = i.saturating_add(1);
        }

        let ix = spl_token::instruction::transfer(
            c.target, // ← PDAに保持される値
            ctx.accounts.vault.key(),
            ctx.accounts.member_ata.key(),
            ctx.accounts.admin.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.member_ata.to_account_info(),
            ctx.accounts.admin.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct Cfg {
    pub admin: Pubkey,
    pub target: Pubkey,
    pub bump: u8,
    pub count: u32,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 32 + 1 + 4, seeds=[b"cfg", admin.key().as_ref()], bump)]
    pub cfg: Account<'info, Cfg>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SetTarget<'info> {
    #[account(mut)]
    pub cfg: Account<'info, Cfg>,
    pub admin: Signer<'info>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub cfg: Account<'info, Cfg>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub member_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("admin mismatch")] Admin }
