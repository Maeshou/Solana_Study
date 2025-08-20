use anchor_lang::prelude::*;

// ── アカウントデータをファイル冒頭に定義 ──
#[account]
#[derive(Default)]
pub struct DonationTracker(pub u8, pub Vec<(Pubkey, u64)>);

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUZ");

#[program]
pub mod donation_tracker {
    use super::*;

    /// トラッカー初期化：内部 Vec はデフォルトで空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let tracker_bump = ctx.bumps.get("tracker").unwrap();
        ctx.accounts.tracker.0 = *tracker_bump;
        Ok(())
    }

    /// 寄付受付：既存の寄付者なら累計加算、なければ新規エントリ追加
    pub fn donate(ctx: Context<Modify>, donor: Pubkey, amount: u64) -> Result<()> {
        let entries = &mut ctx.accounts.tracker.1;
        let mut found = false;

        // 既存エントリの更新
        for entry in entries.iter_mut() {
            if entry.0 == donor {
                entry.1 = entry.1.wrapping_add(amount);
                found = true;
            }
        }

        // 新規寄付者
        if !found {
            entries.push((donor, amount));
        }

        Ok(())
    }

    /// 閾値以下の寄付者を一括削除
    pub fn purge_small(ctx: Context<Modify>, min_amount: u64) -> Result<()> {
        let entries = &mut ctx.accounts.tracker.1;
        entries.retain(|&(_, amt)| amt > min_amount);
        Ok(())
    }

    /// 最高寄付者をログ出力
    pub fn report_top(ctx: Context<Modify>) -> Result<()> {
        let entries    = &ctx.accounts.tracker.1;
        let mut top_amt = 0u64;
        let mut top_dnr = Pubkey::default();

        for &(donor, amt) in entries.iter() {
            if amt > top_amt {
                top_amt = amt;
                top_dnr = donor;
            }
        }

        msg!("Top donor: {}", top_dnr);
        Ok(())
    }
}

// ── Context 定義は末尾に配置 ──
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"donation_tracker", authority.key().as_ref()],
        bump,
        // 8 (discriminator) + 1 (bump) + 4 (Vec len) + 10*(32+8) (max10寄付者)
        space = 8 + 1 + 4 + 10 * (32 + 8)
    )]
    pub tracker:   Account<'info, DonationTracker>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"donation_tracker", authority.key().as_ref()],
        bump = tracker.0,
    )]
    pub tracker:   Account<'info, DonationTracker>,

    #[account(signer)]
    pub authority: AccountInfo<'info>,
}
