use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct ShipmentTracker(pub u8, pub Vec<(u64, u8)>); // (bump, Vec<(shipment_id, status)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzV6");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of shipments reached")]
    MaxShipmentsReached,
    #[msg("Shipment not found")]
    ShipmentNotFound,
}

#[program]
pub mod shipment_tracker {
    use super::*;

    const MAX_SHIPMENTS: usize = 50;
    // status: 0 = pending, 1 = in_transit, 2 = delivered

    /// アカウント初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bump = *ctx.bumps.get("tracker").unwrap();
        ctx.accounts.tracker.0 = bump;
        Ok(())
    }

    /// 出荷登録：件数制限チェック＋pendingステータスで追加
    pub fn add_shipment(ctx: Context<Modify>, shipment_id: u64) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        if list.len() >= MAX_SHIPMENTS {
            return err!(ErrorCode::MaxShipmentsReached);
        }
        list.push((shipment_id, 0));
        Ok(())
    }

    /// 在庫輸送中へ更新：existingチェック＋ステータス更新
    pub fn mark_in_transit(ctx: Context<Modify>, shipment_id: u64) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == shipment_id {
                entry.1 = 1;
                found = true;
            }
        }
        if !found {
            return err!(ErrorCode::ShipmentNotFound);
        }
        Ok(())
    }

    /// 配送完了へ更新：existingチェック＋ステータス更新
    pub fn mark_delivered(ctx: Context<Modify>, shipment_id: u64) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == shipment_id {
                entry.1 = 2;
                found = true;
            }
        }
        if !found {
            return err!(ErrorCode::ShipmentNotFound);
        }
        Ok(())
    }

    /// deliveredステータスのものだけ一括除去
    pub fn purge_delivered(ctx: Context<Modify>) -> Result<()> {
        ctx.accounts.tracker.1.retain(|&(_, status)| status != 2);
        Ok(())
    }

    /// pending状態の個数をログ出力
    pub fn count_pending(ctx: Context<Modify>) -> Result<()> {
        let list = &ctx.accounts.tracker.1;
        let mut cnt = 0u64;
        for &(_, status) in list.iter() {
            if status == 0 {
                cnt = cnt.wrapping_add(1);
            }
        }
        msg!("Pending shipments: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"tracker", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec len(4) + max50*(8+1)
        space = 8 + 1 + 4 + 50 * (8 + 1)
    )]
    pub tracker:   Account<'info, ShipmentTracker>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"tracker", authority.key().as_ref()],
        bump = tracker.0
    )]
    pub tracker:   Account<'info, ShipmentTracker>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
