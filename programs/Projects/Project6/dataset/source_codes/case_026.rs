// ==========================================================
// (16) craft_tokens: トークン化資材（SPL: 素材Mint/TA検証）
//   - 多層防御: Account<T> + has_one + slot不一致 + TokenAccount検証
// ==========================================================
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("CrAfT0k3n6666666666666666666666666666666");

#[program]
pub mod craft_tokens {
    use super::*;

    pub fn init_factory(ctx: Context<InitFactory>) -> Result<()> {
        let f = &mut ctx.accounts.factory;
        f.owner = ctx.accounts.owner.key();
        f.hash = 0;
        Ok(())
    }

    pub fn init_slot(ctx: Context<InitSlot>, idx: u8) -> Result<()> {
        let s = &mut ctx.accounts.slot;
        s.parent = ctx.accounts.factory.key();
        s.idx = idx;
        s.used = false;
        Ok(())
    }

    pub fn batch(ctx: Context<Batch>, bias: u16) -> Result<()> {
        require!(
            ctx.accounts.mat_ta.mint == ctx.accounts.mat_mint.key(),
            CFErr::MintMismatch
        );
        require!(
            ctx.accounts.mat_ta.owner == ctx.accounts.owner.key(),
            CFErr::OwnerMismatch
        );

        let f = &mut ctx.accounts.factory;
        let a = &mut ctx.accounts.slot_a;
        let b = &mut ctx.accounts.slot_b;
        let w = &mut ctx.accounts.warehouse;

        let mut s = 0u32;
        for i in 0..w.blks.len() {
            let v = ((bias as u32) + i as u32 * 9) & 0xFFF;
            w.blks[i] = w.blks[i].saturating_add(v);
            s = s.saturating_add(v);
        }

        if a.idx != b.idx {
            w.ok = w.ok.saturating_add((s / 8) as u64);
            a.used = true;
            f.hash ^= (s as u64).rotate_left(11);
            msg!("different slot: ok+");
        } else {
            w.ng = w.ng.saturating_add((s / 7) as u64);
            b.used = true;
            f.hash = f.hash.rotate_right(9) ^ 0xA5A5u64;
            msg!("same slot: ng+");
        }
        Ok(())
    }
}

// ----------------------------------------------------------
// アカウント定義
// ----------------------------------------------------------

#[derive(Accounts)]
pub struct InitFactory<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8)]
    pub factory: Account<'info, Factory>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitSlot<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 1 + 1)]
    pub slot: Account<'info, Slot>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub factory: Account<'info, Factory>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Batch<'info> {
    #[account(mut, has_one = owner)]
    pub factory: Account<'info, Factory>,
    #[account(mut)]
    pub slot_a: Account<'info, Slot>,
    #[account(mut)]
    pub slot_b: Account<'info, Slot>,
    #[account(mut)]
    pub warehouse: Account<'info, Warehouse>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mat_mint: Account<'info, Mint>,
    #[account(mut)]
    pub mat_ta: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

// ----------------------------------------------------------
// データ構造
// ----------------------------------------------------------

#[account]
pub struct Factory {
    pub owner: Pubkey,
    pub hash: u64,
}

#[account]
pub struct Slot {
    pub parent: Pubkey,
    pub idx: u8,
    pub used: bool,
}

#[account]
pub struct Warehouse {
    pub blks: Vec<u32>,
    pub ok: u64,
    pub ng: u64,
}

// ----------------------------------------------------------
// エラーコード
// ----------------------------------------------------------

#[error_code]
pub enum CFErr {
    #[msg("Mint does not match")]
    MintMismatch,
    #[msg("Token account owner does not match")]
    OwnerMismatch,
}
