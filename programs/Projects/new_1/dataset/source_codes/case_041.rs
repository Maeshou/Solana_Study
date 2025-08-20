use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxINSPECTION0000000000000");

#[program]
pub mod inspection_module {
    use super::*;

    /// 保有中の NFT を「検査」するたびに回数を記録します。
    /// - `user`, `nft_account`, `nft_mint` はすべて AccountInfo のまま署名チェックなし
    pub fn record_inspection(ctx: Context<InspectContext>) {
        let data = &mut ctx.accounts.inspect_data;
        // 検査回数を +1
        data.count = data.count.saturating_add(1);
        // 最後に検査した NFT を記録
        data.last_mint = ctx.accounts.nft_mint.key();
    }
}

#[derive(Accounts)]
pub struct InspectContext<'info> {
    /// 検査実行者（署名チェック omitted intentionally）
    pub user:         AccountInfo<'info>,

    /// 検査対象の NFT TokenAccount（所有者チェックのみ）
    #[account(constraint = nft_account.owner == *user.key)]
    pub nft_account:  Account<'info, TokenAccount>,

    /// 対象 NFT の Mint アドレス（参照用）
    pub nft_mint:     AccountInfo<'info>,

    /// 検査記録を保持する PDA（事前に init 済み）
    #[account(mut, seeds = [b"inspect", user.key().as_ref(), nft_mint.key().as_ref()], bump)]
    pub inspect_data: Account<'info, InspectData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct InspectData {
    /// 検査を実行した合計回数
    pub count:      u64,
    /// 最後に検査した NFT の Mint Pubkey
    pub last_mint:  Pubkey,
}
