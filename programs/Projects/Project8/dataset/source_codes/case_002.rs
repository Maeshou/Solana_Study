use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("CrOsSDomBSC2222222222222222222222222222");

#[program]
pub mod store_bsc_cross_domain_safe {
    use super::*;

    pub fn initialize_store(ctx: Context<InitializeStore>, mint: Pubkey, base: u64) -> Result<()> {
        let s = &mut ctx.accounts.store;
        s.owner = ctx.accounts.merchant.key();
        s.mint = mint;
        s.bump_saved = *ctx.bumps.get("store").ok_or(error!(StoreErr::MissingBump))?;
        s.power = base.rotate_left(3).wrapping_add(63);
        s.rounds = 1;

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

    // Option A: store PDA から直接支払う（最もシンプルで安全）
    pub fn issue_coupon_payout(ctx: Context<IssueCouponPayout>, coupon_id: u64, lamports: u64) -> Result<()> {
        let s = &ctx.accounts.store;

        let from_key = ctx.accounts.store.key();
        let ix = system_instruction::transfer(&from_key, &ctx.accounts.beneficiary.key(), lamports);

        // 検証と同一 seeds/bump（store のもの）で署名
        let seeds: &[&[u8]] = &[
            b"store",
            ctx.accounts.merchant.key.as_ref(),
            ctx.accounts.store.mint.as_ref(),
            &[s.bump_saved],
        ];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.store.to_account_info(),         // from
                ctx.accounts.beneficiary.to_account_info(),   // to
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        // coupon_id はイベントやログに使うなど（署名・シードに絡めない）
        emit!(CouponPaid { coupon_id, to: ctx.accounts.beneficiary.key(), amount: lamports });

        Ok(())
    }

    // ─────────────────────────────────────────────────────────
    // Option B（必要な場合のみ使用）:
    // coupon PDA から払いたい要件があるなら、coupon も「検証済みアカウント」にする。
    // 署名は ctx.bumps["coupon"] による正規 bump のみを使用する。
    // ─────────────────────────────────────────────────────────
    /*
    pub fn issue_coupon_payout_from_coupon(ctx: Context<IssueCouponPayoutFromCoupon>, lamports: u64) -> Result<()> {
        let from_key = ctx.accounts.coupon.key();
        let ix = system_instruction::transfer(&from_key, &ctx.accounts.beneficiary.key(), lamports);

        let seeds: &[&[u8]] = &[
            b"coupon",
            ctx.accounts.merchant.key.as_ref(),
            &ctx.accounts.coupon_id.to_le_bytes(),
            &[ctx.bumps.get("coupon").copied().unwrap()],
        ];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.coupon.to_account_info(),
                ctx.accounts.beneficiary.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
    */
}

#[event]
pub struct CouponPaid {
    pub coupon_id: u64,
    pub to: Pubkey,
    pub amount: u64,
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
    /// CHECK: 実プロダクションでは Mint 検証を入れる
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
    #[account(mut)]
    pub beneficiary: SystemAccount<'info>,
    pub merchant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/* 必要な場合のみ使う Option B 用の Accounts
#[derive(Accounts)]
pub struct IssueCouponPayoutFromCoupon<'info> {
    #[account(
        mut,
        seeds = [b"coupon", merchant.key().as_ref(), coupon_id.to_le_bytes().as_ref()],
        bump
    )]
    pub coupon: SystemAccount<'info>,
    pub coupon_id: u64,
    #[account(mut)]
    pub beneficiary: SystemAccount<'info>,
    pub merchant: Signer<'info>,
    pub system_program: Program<'info, System>,
}
*/

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
}
