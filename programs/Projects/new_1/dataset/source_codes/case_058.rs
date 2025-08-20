use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxNFTVOTE000000000000000");

#[program]
pub mod nft_vote {
    use super::*;

    /// 保有中の NFT を使って２択投票を行います。
    /// - `choice`: 0 = A に投票, 1 = B に投票  
    /// すべてのアカウントは AccountInfo／Account で受け取り、Signer<'info> は使いません。
    /// 分岐やループは不要で、比較演算とブール→数値キャストだけで更新します。
    pub fn cast_vote(ctx: Context<CastVote>, choice: u8) {
        let record = &mut ctx.accounts.vote_data;
        // 分岐なしで投票先を判定
        let vote_a = (choice == 0) as u64;
        let vote_b = (choice == 1) as u64;
        // カウントを更新
        record.votes_a = record.votes_a.saturating_add(vote_a);
        record.votes_b = record.votes_b.saturating_add(vote_b);
    }
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    /// 投票者（署名チェック omitted intentionally）
    pub user:       AccountInfo<'info>,

    /// 投票資格を示す NFT の TokenAccount 所有者チェック
    #[account(
        constraint = hold_acc.owner == *user.key,
        constraint = hold_acc.mint  == nft_mint.key()
    )]
    pub hold_acc:   Account<'info, TokenAccount>,

    /// 対象 NFT Mint（参照用）
    pub nft_mint:   AccountInfo<'info>,

    /// 投票結果を保持する PDA
    #[account(
        mut,
        seeds = [b"vote", nft_mint.key().as_ref()],
        bump
    )]
    pub vote_data:  Account<'info, VoteData>,
}

#[account]
pub struct VoteData {
    /// A に投票した回数
    pub votes_a: u64,
    /// B に投票した回数
    pub votes_b: u64,
}
