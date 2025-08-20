use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct CouponLedger(pub u8, pub Vec<(Pubkey, bool)>); // (bump, Vec<(user, used_flag)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzV9");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of coupons issued")]
    MaxCouponsIssued,
    #[msg("Coupon already used or not found")]
    InvalidCoupon,
}

#[program]
pub mod coupon_ledger {
    use super::*;

    const MAX_COUPONS: usize = 100;

    /// 台帳初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bump = *ctx.bumps.get("ledger").unwrap();
        ctx.accounts.ledger.0 = bump;
        Ok(())
    }

    /// 新規クーポン発行：件数制限チェック＋未使用フラグで追加
    pub fn issue_coupon(ctx: Context<Modify>, user: Pubkey) -> Result<()> {
        let entries = &mut ctx.accounts.ledger.1;
        if entries.len() >= MAX_COUPONS {
            return err!(ErrorCode::MaxCouponsIssued);
        }
        entries.push((user, false));
        Ok(())
    }

    /// クーポン使用：該当ユーザーの未使用エントリを「使用済み」に更新
    pub fn redeem_coupon(ctx: Context<Modify>, user: Pubkey) -> Result<()> {
        let entries = &mut ctx.accounts.ledger.1;
        let mut found = false;
        for entry in entries.iter_mut() {
            if entry.0 == user {
                if entry.1 == false {
                    entry.1 = true;
                    found = true;
                }
            }
        }
        if found == false {
            return err!(ErrorCode::InvalidCoupon);
        }
        Ok(())
    }

    /// 既使用クーポンの一括削除
    pub fn purge_used(ctx: Context<Modify>) -> Result<()> {
        let entries = &mut ctx.accounts.ledger.1;
        entries.retain(|&(_, used)| {
            if used == false {
                true
            } else {
                false
            }
        });
        Ok(())
    }

    /// 未使用クーポン数をログ出力
    pub fn count_available(ctx: Context<Modify>) -> Result<()> {
        let entries = &ctx.accounts.ledger.1;
        let mut cnt = 0u64;
        for &(_, used) in entries.iter() {
            if used == false {
                cnt = cnt.wrapping_add(1);
            }
        }
        msg!("Available coupons: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"ledger", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec len(4) + max100*(32+1)
        space = 8 + 1 + 4 + 100 * (32 + 1)
    )]
    pub ledger:    Account<'info, CouponLedger>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"ledger", authority.key().as_ref()],
        bump = ledger.0
    )]
    pub ledger:    Account<'info, CouponLedger>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
