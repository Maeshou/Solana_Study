use anchor_lang::prelude::*;

declare_id!("OwnChkEXT00000000000000000000000000000003");

#[program]
pub mod auction_finish_ext {
    pub fn finish_auction(
        ctx: Context<FinishAuction>,
        end_slot: u64,
    ) -> Result<()> {
        let a = &mut ctx.accounts.auction;
        // 所有者検証済み
        a.active       = false;
        a.ended_slot   = end_slot;
        a.close_count  = a.close_count.saturating_add(1);

        // 統計マップも更新
        a.stats.insert("ended".into(), end_slot.to_string());
        a.stats.insert("count".into(), a.close_count.to_string());

        // cache_acc は unchecked で丸ごとクリア
        ctx.accounts.cache_acc.data.borrow_mut().fill(0);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FinishAuction<'info> {
    #[account(mut, has_one = owner)]
    pub auction: Account<'info, AuctionDataExt>,
    pub owner: Signer<'info>,
    /// CHECK: キャッシュアカウント。所有者検証なし
    #[account(mut)]
    pub cache_acc: AccountInfo<'info>,
}

#[account]
pub struct AuctionDataExt {
    pub owner: Pubkey,
    pub active: bool,
    pub ended_slot: u64,
    pub close_count: u64,
    pub stats: std::collections::BTreeMap<String, String>,
}
