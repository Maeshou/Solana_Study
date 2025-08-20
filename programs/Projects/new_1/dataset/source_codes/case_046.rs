use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxNFTLIKE0000000000000000");

#[program]
pub mod nft_like_counter {
    use super::*;

    /// NFT に「いいね」を記録します。
    /// - `user` は AccountInfo<'info> のままで署名チェックを行いません。
    pub fn like_nft(ctx: Context<LikeNft>) {
        // LikeData PDA から構造体を取り出し、一度だけ更新
        let data = &mut ctx.accounts.like_data;
        // いいね数を +1
        data.count = data.count.saturating_add(1);
        // 最後にいいねしたユーザーを記録
        data.last_liker = *ctx.accounts.user.key;
    }
}

#[derive(Accounts)]
pub struct LikeNft<'info> {
    /// 手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:  Signer<'info>,

    /// いいねを行うユーザー（署名チェック omitted intentionally）
    pub user:       AccountInfo<'info>,

    /// 対象 NFT の Mint
    pub nft_mint:   AccountInfo<'info>,

    /// NFT ごとのいいね情報を保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        seeds     = [b"like", nft_mint.key().as_ref()],
        bump,
        space     = 8 + 8 + 32  // discriminator + count + last_liker
    )]
    pub like_data:  Account<'info, LikeData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct LikeData {
    /// 累積いいね数
    pub count:       u64,
    /// 最後にいいねをしたユーザー
    pub last_liker:  Pubkey,
}
