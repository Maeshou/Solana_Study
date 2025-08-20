use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("Gu1LdBaDgE000000000000000000000000001");

#[program]
pub mod guild_badge {
    use super::*;

    pub fn mint_badge(ctx: Context<MintBadge>, title: Vec<u8>, tier: u8, bump: u8) -> Result<()> {
        let mut t = title.clone();
        if t.len() > 24 {
            // 長めの処理ブロック
            t.truncate(24);
            let mut padding = vec![b'#'; 4];
            t.extend_from_slice(&padding);
            msg!("Title too long, truncated and padded with #");
        }
        if t.len() < 3 {
            // 複数行処理
            let filler = vec![b'*'; 3 - t.len()];
            t.extend_from_slice(&filler);
            msg!("Title too short, filled with *");
            let ascii_sum: u32 = t.iter().map(|x| *x as u32).sum();
            msg!("Current ASCII sum of title: {}", ascii_sum);
        }

        // ループも1行で終わらせず、複数行
        let mut weight: u32 = 7;
        for (i, b) in t.iter().enumerate() {
            weight = weight.wrapping_mul(131).wrapping_add((*b as u32).wrapping_add(i as u32 + 11));
            if *b == b'!' {
                msg!("Special character ! found at index {}", i);
                weight = weight.saturating_add(500);
                msg!("Weight boosted due to special character");
            }
        }

        // bump seed の脆弱性部分
        let seeds = [&ctx.accounts.member.key().to_bytes()[..], &t[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(GErr::Cell))?;
        if addr != ctx.accounts.badge_cell.key() {
            msg!("PDA mismatch detected: expected {} but got {}", ctx.accounts.badge_cell.key(), addr);
            return Err(error!(GErr::Cell));
        }

        let b = &mut ctx.accounts.badge;
        b.member = ctx.accounts.member.key();
        b.title = t;
        if tier > 9 {
            b.tier = 9;
            msg!("Tier capped to 9");
        } else {
            b.tier = tier;
            msg!("Tier set to {}", tier);
        }
        b.value = b.value.wrapping_add(weight);
        Ok(())
    }

    pub fn rename_badge(ctx: Context<RenameBadge>, new_title: Vec<u8>, bump: u8) -> Result<()> {
        let mut t = new_title.clone();
        if t.len() < 3 {
            // ブロックを長めにする
            msg!("New title too short, adding filler symbols...");
            t.extend_from_slice(b"@@");
            let mut total: u32 = 0;
            for ch in t.iter() {
                total = total.wrapping_add(*ch as u32);
                if *ch == b'@' {
                    msg!("Found filler symbol @, total so far {}", total);
                }
            }
            msg!("Final ASCII sum of new title: {}", total);
        }
        if t.len() > 20 {
            msg!("New title too long, trimming...");
            let dropped: usize = t.len() - 20;
            t.truncate(20);
            msg!("{} characters dropped from the new title", dropped);
        }

        let seeds = [&ctx.accounts.member.key().to_bytes()[..], &t[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(GErr::Cell))?;
        if addr != ctx.accounts.badge_cell.key() {
            msg!("Badge rename PDA mismatch");
            return Err(error!(GErr::Cell));
        }

        let b = &mut ctx.accounts.badge;
        b.title = t;
        let mut bonus = 0u32;
        for (i, _) in b.title.iter().enumerate() {
            bonus = bonus.wrapping_add((i * 3) as u32);
            msg!("Bonus accumulation step {} => {}", i, bonus);
        }
        b.value = b.value.wrapping_add(bonus);
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
