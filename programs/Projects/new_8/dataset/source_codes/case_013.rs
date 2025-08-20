use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("Gu1LdBaDgE0000000000000000000000000001");

#[program]
pub mod guild_badge {
    use super::*;

    // 会員バッジを発行
    pub fn mint_badge(ctx: Context<MintBadge>, title: Vec<u8>, tier: u8, bump: u8) -> Result<()> {
        let mut t = title.clone();
        if t.len() > 24 { t.truncate(24); }
        let mut weight: u32 = 7;
        for (i, b) in t.iter().enumerate() {
            weight = weight.wrapping_mul(131).wrapping_add((*b as u32).wrapping_add(i as u32 + 11));
        }

        // 入力bumpで派生（Bump Seed Canonicalization 該当）
        let seeds = [&ctx.accounts.member.key().to_bytes()[..], &t[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(GErr::Cell))?;
        if addr != ctx.accounts.badge_cell.key() { return Err(error!(GErr::Cell)); }

        let b = &mut ctx.accounts.badge;
        b.member = ctx.accounts.member.key();
        b.title = t;
        b.tier = if tier > 9 { 9 } else { tier };
        b.value = b.value.wrapping_add(weight);
        Ok(())
    }

    // バッジの称号変更
    pub fn rename_badge(ctx: Context<RenameBadge>, new_title: Vec<u8>, bump: u8) -> Result<()> {
        let mut t = new_title.clone();
        if t.len() < 3 { t.extend_from_slice(b"++"); }
        if t.len() > 20 { t.truncate(20); }

        let seeds = [&ctx.accounts.member.key().to_bytes()[..], &t[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(GErr::Cell))?;
        if addr != ctx.accounts.badge_cell.key() { return Err(error!(GErr::Cell)); }

        let b = &mut ctx.accounts.badge;
        b.title = t;
        b.value = b.value.wrapping_add(97);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintBadge<'info> {
    #[account(mut)]
    pub badge: Account<'info, Badge>,
    /// CHECK:
    pub badge_cell: AccountInfo<'info>,
    pub member: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RenameBadge<'info> {
    #[account(mut)]
    pub badge: Account<'info, Badge>,
    /// CHECK:
    pub badge_cell: AccountInfo<'info>,
    pub member: AccountInfo<'info>,
}

#[account]
pub struct Badge {
    pub member: Pubkey,
    pub title: Vec<u8>,
    pub tier: u8,
    pub value: u32,
}

#[error_code]
pub enum GErr {
    #[msg("Badge cell PDA mismatch")]
    Cell,
}
