use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRoyalty001");

#[program]
pub mod resale_service {
    use super::*;

    /// NFT の二次販売時にロイヤリティを分配するが、
    /// royalty_info.owner と ctx.accounts.seller.key() の照合チェックがない
    pub fn distribute_royalty(ctx: Context<DistributeRoyalty>, sale_price: u64) -> Result<()> {
        let info = &mut ctx.accounts.royalty_info;
        // 設定アカウントからパーセンテージを取得（例：5 = 5%）
        let pct = ctx.accounts.config.royalty_percentage as u64;
        // ロイヤリティ額を計算
        let amount = sale_price
            .checked_mul(pct)
            .unwrap()
            .checked_div(100)
            .unwrap();

        // 累計分配額に加算
        info.total_disbursed = info.total_disbursed.checked_add(amount).unwrap();

        // Lamports の直接転送（所有者チェックなし）
        **ctx.accounts.royalty_wallet.to_account_info().lamports.borrow_mut() += amount;
        **ctx.accounts.seller.to_account_info().lamports.borrow_mut() -= amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DistributeRoyalty<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付与して売り手照合を行うべき
    pub royalty_info: Account<'info, RoyaltyInfo>,

    /// ロイヤリティ受取用ウォレット
    #[account(mut)]
    pub royalty_wallet: AccountInfo<'info>,

    /// NFT を売却した売り手（署名者）
    #[account(mut)]
    pub seller: Signer<'info>,

    /// ロイヤリティ率を保持する設定アカウント
    pub config: Account<'info, RoyaltyConfig>,
}

#[account]
pub struct RoyaltyInfo {
    /// 本来このロイヤリティ情報を所有するべきアカウント
    pub owner: Pubkey,
    /// これまでに分配された合計ロイヤリティ（Lamports）
    pub total_disbursed: u64,
}

#[account]
pub struct RoyaltyConfig {
    /// 二次販売時のロイヤリティ率（パーセンテージ、0〜100）
    pub royalty_percentage: u8,
}
