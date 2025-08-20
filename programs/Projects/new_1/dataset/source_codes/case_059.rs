use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxATTRSTONE0000000000000");

#[program]
pub mod nft_attribute_enhancer {
    use super::*;

    /// ユーザーが保持する「属性強化石」を消費して
    /// NFT の属性値を上昇させます。
    ///
    /// - `stone_count`     : 使用する強化石の枚数  
    /// - `inc_per_stone`   : １石あたりの属性上昇量  
    /// ※ すべてのアカウントは AccountInfo／Account のまま、Signer は使いません。
    pub fn apply_stones(
        ctx: Context<ApplyStones>,
        stone_count: u64,
        inc_per_stone: u64,
    ) {
        // 増分を計算
        let delta = stone_count.checked_mul(inc_per_stone).unwrap();
        // PDA に属性値を累積更新
        let data = &mut ctx.accounts.attr_data;
        data.value = data.value.saturating_add(delta);
    }
}

#[derive(Accounts)]
pub struct ApplyStones<'info> {
    /// トランザクション手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:    Signer<'info>,

    /// 強化を実行するユーザー（署名チェック omitted intentionally）
    pub user:         AccountInfo<'info>,

    /// 強化石として消費するトークンアカウント（所有者チェックのみ）
    #[account(
        constraint = stone_acc.owner == *user.key,
        constraint = stone_acc.mint  == stone_mint.key()
    )]
    pub stone_acc:    Account<'info, TokenAccount>,
    /// 強化石の Mint
    pub stone_mint:   AccountInfo<'info>,

    /// 強化対象の NFT TokenAccount（所有者チェックのみ）
    #[account(
        constraint = nft_acc.owner == *user.key,
        constraint = nft_acc.mint  == nft_mint.key()
    )]
    pub nft_acc:      Account<'info, TokenAccount>,
    /// 対象 NFT の Mint
    pub nft_mint:     AccountInfo<'info>,

    /// NFT の属性値を保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        seeds     = [b"attr", user.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        space     = 8 + 8
    )]
    pub attr_data:    Account<'info, AttrData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct AttrData {
    /// 現在の属性値
    pub value: u64,
}
