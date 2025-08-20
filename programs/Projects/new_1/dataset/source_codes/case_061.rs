use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxNFTLEVELUP000000000");

#[program]
pub mod nft_level_system {
    use super::*;

    /// 指定した経験値を NFT に付与し、その閾値でレベルを更新します。
    ///
    /// - `xp_amount`: 付与する経験値  
    /// - `threshold`: １レベルあたりに必要な経験値
    ///
    /// すべてのアカウントは AccountInfo／Account のまま、  
    /// 分岐やループは使わず、算術演算だけで実現します。
    pub fn earn_and_level(ctx: Context<EarnAndLevel>, xp_amount: u64, threshold: u64) {
        // ① 状態アカウントを取り出し
        let data = &mut ctx.accounts.xp_data;

        // ② 経験値を累積
        data.total_xp = data.total_xp
            .checked_add(xp_amount).unwrap();

        // ③ レベルを閾値で床除算して更新
        data.level = data.total_xp
            .checked_div(threshold).unwrap();
    }
}

#[derive(Accounts)]
pub struct EarnAndLevel<'info> {
    /// 手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer: Signer<'info>,

    /// NFT 保有者（署名チェック omitted intentionally）
    pub user:      AccountInfo<'info>,

    /// 対象 NFT の Mint（参照用）
    pub nft_mint:  AccountInfo<'info>,

    /// NFT ごとの経験値・レベルを保持する PDA
    #[account(
        init_if_needed,
        payer = fee_payer,
        seeds = [b"xp", user.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        space = 8  /* discriminator */
              + 32 /* owner Pubkey */
              + 32 /* mint Pubkey */
              + 8  /* total_xp */
              + 8  /* level */
    )]
    pub xp_data:   Account<'info, XpData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct XpData {
    /// 本来は owner で検証すべき Pubkey
    pub owner:    Pubkey,
    /// 対象 NFT の Mint
    pub mint:     Pubkey,
    /// 累積経験値
    pub total_xp: u64,
    /// 現在のレベル
    pub level:    u64,
}
