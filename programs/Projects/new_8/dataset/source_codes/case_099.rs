use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

// ─────────────────────────────────────────────────────────────
// Program 2: 検証したPDAの bump を「別ドメインのPDA」に流用（cross-domain BSC）
// 検証: [b"store", merchant, mint]
// 署名: [b"coupon", merchant, coupon_id, store.bump_saved]（別の seed 名前空間）
// ─────────────────────────────────────────────────────────────
declare_id!("CrOsSDomBSC2222222222222222222222222222");

#[program]
pub mod store_bsc_cross_domain {
    use super::*;

    pub fn initialize_store(ctx: Context<InitializeStore>, mint: Pubkey, base: u64) -> Result<()> {
        let s = &mut ctx.accounts.store;
        s.owner = ctx.accounts.merchant.key();
        s.mint = mint;
        s.bump_saved = *ctx.bumps.get("store").ok_or(error!(StoreErr::MissingBump))?;
        s.power = base.rotate_left(3).wrapping_add(63);
        s.rounds = 1;

        // 適当に手を動かす
        for j in 1u8..4u8 {
            let v = (s.power ^ (j as u64 * 19)).rotate_left(1);
            s.power = s.power.wrapping_add(v).wrapping_mul(2).wrapping_add(11 + j as u64);
            s.rounds = s.rounds.saturating_add(((s.power % 23) as u32) + 3);
        }
        if (s.power & 1) > 0 {
            let extra = s.power.rotate_right(2).wrapping_add(29);
            s.power = s.power.wrapping_add(extra).wrapping_mul(2);
            s.rounds = s.rounds.saturating_add(((s.power % 29) as u32) + 4);
        }

        Ok(())
    }

    pub fn issue_coupon_payout(
        ctx: Context<IssueCouponPayout>,
        coupon_id: u64,
        lamports: u64,
    ) -> Result<()> {
        let s = &ctx.accounts.store;

        // ここが BSC：検証した "store" の bump を、別ドメイン "coupon" の seeds で署名に流用
        let seeds = &[
            b"coupon".as_ref(),
            s.owner.as_ref(),
            &coupon_id.to_le_bytes(),
            core::slice::from_ref(&s.bump_saved),
        ];
        let coupon_pda = Pubkey::create_program_address(
            &[b"coupon", s.owner.as_ref(), &coupon_id.to_le_bytes(), &[s.bump_saved]],
            ctx.program_id,
        ).map_err(|_| error!(StoreErr::SeedCompute))?;

        let ix = system_instruction::transfer(&coupon_pda, &ctx.accounts.beneficiary.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.coupon_hint.to_account_info(), // CHECK
                ctx.accounts.beneficiary.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        // おまけ：軽い後処理
        if lamports > 700 {
            let mut k = 1u8;
            let mut acc = lamports.rotate_left(1);
            while k < 3 {
                let u = (acc ^ (k as u64 * 13)).rotate_right(1);
                acc = acc.wrapping_add(u);
                k = k.saturating_add(1);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeStore<'info> {
    #[account(
        init,
        payer = merchant,
        space = 8 + 32 + 32 + 8 + 4 + 1,
        seeds = [b"store", merchant.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub store: Account<'info, StoreState>,
    #[account(mut)]
    pub merchant: Signer<'info>,
    /// CHECK: mint は検証外でもよいサンプルに
    pub mint: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IssueCouponPayout<'info> {
    #[account(
        mut,
        seeds = [b"store", merchant.key().as_ref(), store.mint.key().as_ref()],
        bump = store.bump_saved
    )]
    pub store: Account<'info, StoreState>,
    /// CHECK: 未検証のヒント口座
    pub coupon_hint: AccountInfo<'info>,
    #[account(mut)]
    pub beneficiary: AccountInfo<'info>,
    pub merchant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StoreState {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub power: u64,
    pub rounds: u32,
    pub bump_saved: u8,
}

#[error_code]
pub enum StoreErr {
    #[msg("missing bump")] MissingBump,
    #[msg("seed compute failed")] SeedCompute,
}
