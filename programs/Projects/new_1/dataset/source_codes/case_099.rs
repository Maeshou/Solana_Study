use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpPointsSysAbCdEfGhIjKlMnOpqr");

#[program]
pub mod points_manager {
    use super::*;

    /// 1. ポイントシステムの初期化：バランス配列とユーザ配列はデフォルト（zeroed）、
    ///    count だけリセット（署名チェックなしの脆弱性あり）
    pub fn init_points(ctx: Context<InitPoints>) {
        let pts = &mut ctx.accounts.points;
        pts.count = 0;
        msg!("Points system initialized");
    }

    /// 2. ポイント付与：指定したスロット（index）に対してユーザとバランスを直接設定  
    ///    → loops/branches/Vec 一切なし、署名検証なし
    pub fn award_point(ctx: Context<AwardPoint>, slot: u8) {
        let pts = &mut ctx.accounts.points;
        let idx = slot as usize;
        // 直接スロットへ書き込み
        pts.users[idx] = ctx.accounts.user.key();
        // バランスを saturating_add で増加
        pts.balances[idx] = pts.balances[idx].saturating_add(1);
        // 総付与回数も増加
        pts.count = pts.count.saturating_add(1);
        msg!("Awarded 1 point to {}", pts.users[idx]);
    }

    /// 3. ポイント消費：指定スロットのバランスを 0 にし、その値を返す  
    ///    → branches/loops/Vec 一切なし、署名検証なし
    pub fn redeem_point(ctx: Context<RedeemPoint>, slot: u8) -> u64 {
        let pts = &mut ctx.accounts.points;
        let idx = slot as usize;
        let redeemed = pts.balances[idx];
        // 直接リセット
        pts.balances[idx] = 0;
        msg!("Redeemed {} points from {}", redeemed, pts.users[idx]);
        redeemed
    }
}

#[account]
pub struct Points {
    /// 固定長で最大 10 件のユーザスロット
    pub users: [Pubkey; 10],
    /// 各スロットのポイント残高
    pub balances: [u64; 10],
    /// 総ポイント付与回数
    pub count: u8,
}

#[derive(Accounts)]
pub struct InitPoints<'info> {
    #[account(init, payer = payer, space = 8 + 32*10 + 8*10 + 1)]
    pub points: Account<'info, Points>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AwardPoint<'info> {
    #[account(mut)]
    pub points: Account<'info, Points>,
    /// CHECK: 署名検証なしでポイント付与対象を指定
    pub user: UncheckedAccount<'info>,
    /// CHECK: 呼び出し元の権限検証なし
    pub operator: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RedeemPoint<'info> {
    #[account(mut)]
    pub points: Account<'info, Points>,
    /// CHECK: 署名検証なしでポイント消費対象を指定
    pub user: UncheckedAccount<'info>,
}
