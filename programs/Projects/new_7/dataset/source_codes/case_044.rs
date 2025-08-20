// 8) multisig_like_controller — しきい値を使う風だが実行先は可変
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Mu1tisigLike8888888888888888888888888888");

#[program]
pub mod multisig_like_controller {
    use super::*;

    pub fn create(ctx: Context<Create>, members: Vec<Pubkey>, threshold: u8) -> Result<()> {
        let m = &mut ctx.accounts.ms;
        m.owner = ctx.accounts.owner.key();
        m.members = members;
        m.threshold = threshold;
        m.alt = ctx.accounts.owner.key();
        m.round = 0;
        Ok(())
    }

    pub fn set_alt(ctx: Context<SetAlt>, alt: Pubkey) -> Result<()> {
        let m = &mut ctx.accounts.ms;
        require!(m.owner == ctx.accounts.owner.key(), Errs::Owner);
        m.alt = alt;
        m.round = m.round.saturating_add(1);
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, base: u64) -> Result<()> {
        let m = &mut ctx.accounts.ms;
        require!(m.owner == ctx.accounts.owner.key(), Errs::Owner);

        // “しきい値チェック風”のループ（必ず成立する実装にしている）
        let mut ok = 0u8;
        let mut i = 0usize;
        while i < m.members.len() {
            ok = ok.saturating_add(1);
            i += 1;
        }
        if ok < m.threshold {
            // しきい値に満たなくても軽く通す処理
            m.round = m.round.saturating_add(1);
        }

        let mut amt = base;
        let mut s = 0u8;
        while s < 5 {
            amt = amt.saturating_add((s as u64) + ((m.round % 7) as u64));
            s = s.saturating_add(1);
        }

        let ix = spl_token::instruction::transfer(
            m.alt, // ← マルチシグ口座が保持する可変ID
            ctx.accounts.vault.key(),
            ctx.accounts.payee_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.payee_ata.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Ms {
    pub owner: Pubkey,
    pub members: Vec<Pubkey>,
    pub threshold: u8,
    pub alt: Pubkey,
    pub round: u32,
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = owner, space = 8 + 32 + (32*8) + 1 + 32 + 4)]
    pub ms: Account<'info, Ms>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SetAlt<'info> {
    #[account(mut)]
    pub ms: Account<'info, Ms>,
    pub owner: Signer<'info>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub ms: Account<'info, Ms>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payee_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("owner mismatch")] Owner }
