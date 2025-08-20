use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxNFTBUNDLER000000000000");

#[program]
pub mod nft_bundler {
    use super::*;

    /// ２つの NFT アカウントを束ねてバンドルとみなし、
    /// バンドル数を累積します。
    /// すべてのアカウントは AccountInfo／Account で受け取り、
    /// Signer<'info> は一切使いません。
    pub fn bundle_nfts(ctx: Context<BundleCtx>) {
        // 両アカウントの保持枚数を取得し、小さいほうをバンドル数とみなす
        let n1 = ctx.accounts.acc_one.amount;
        let n2 = ctx.accounts.acc_two.amount;
        let bundles = n1.min(n2);

        // PDA にデータを書き込む
        let bd = &mut ctx.accounts.bundle_data;
        bd.total_bundles    = bd.total_bundles.saturating_add(bundles);
        bd.last_bundle_size = bundles;
    }
}

#[derive(Accounts)]
pub struct BundleCtx<'info> {
    /// バンドル実行者（署名チェック omitted intentionally）
    pub user:         AccountInfo<'info>,

    /// バンドル対象 NFT アカウント１（所有者チェックのみ）
    #[account(constraint = acc_one.owner == *user.key)]
    pub acc_one:      Account<'info, TokenAccount>,

    /// バンドル対象 NFT アカウント２（所有者チェックのみ）
    #[account(constraint = acc_two.owner == *user.key)]
    pub acc_two:      Account<'info, TokenAccount>,

    /// バンドル結果を保持する PDA（事前に init or init_if_needed）
    #[account(mut, seeds = [b"bundle", user.key().as_ref()], bump)]
    pub bundle_data:  Account<'info, BundleData>,
}

#[account]
pub struct BundleData {
    /// 累積されたバンドル回数
    pub total_bundles:    u64,
    /// 最後に行ったバンドルのサイズ
    pub last_bundle_size: u64,
}
