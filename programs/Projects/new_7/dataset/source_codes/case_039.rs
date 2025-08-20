// 3) user_profile_program_pref — 利用者プロフィールに保存されたIDを採用
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Us3rPr0filePref3333333333333333333333333");

#[program]
pub mod user_profile_program_pref {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        p.owner = ctx.accounts.owner.key();
        p.pref = ctx.accounts.owner.key();
        p.activity = 0;
        p.nonce = 1;

        let mut z = 0u8;
        while z < 5 {
            p.activity = p.activity.saturating_add((z as u32) + 1);
            z = z.saturating_add(1);
        }
        Ok(())
    }

    pub fn set_pref(ctx: Context<SetPref>, pref: Pubkey) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        require!(p.owner == ctx.accounts.owner.key(), Errs::Owner);
        p.pref = pref;
        p.nonce = p.nonce.saturating_add(1);
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, base: u64, label: String) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        require!(p.owner == ctx.accounts.owner.key(), Errs::Owner);

        let mut w: u64 = 6;
        let b = label.as_bytes();
        let mut i = 0usize;
        while i < b.len() {
            w = w.saturating_add((b[i] as u64) % 9 + 1);
            if i % 3 == 0 { p.activity = p.activity.saturating_add(1); }
            i += 1;
        }

        let mut amt = base.saturating_mul(w);
        let mut bump = 0u64;
        let mut s = 0u8;
        while s < 4 {
            bump = bump.saturating_add((s as u64) + ((p.nonce % 5) as u64));
            s = s.saturating_add(1);
        }
        amt = amt.saturating_add(bump);

        let ix = spl_token::instruction::transfer(
            p.pref, // ← ユーザーの設定値
            ctx.accounts.vault.key(),
            ctx.accounts.to_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.to_ata.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Profile {
    pub owner: Pubkey,
    pub pref: Pubkey,
    pub activity: u32,
    pub nonce: u32,
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 4 + 4)]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SetPref<'info> {
    #[account(mut)]
    pub profile: Account<'info, Profile>,
    pub owner: Signer<'info>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub profile: Account<'info, Profile>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("owner mismatch")] Owner }
