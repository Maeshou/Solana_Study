use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf064mvTWf");

#[program]
pub mod upgrade_configuration_064 {
    use super::*;

    pub fn upgrade_configuration(ctx: Context<Ctx064>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 064 by {}", ctx.accounts.user.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 064: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxUNLOCK000000000000000");

#[program]
pub mod nft_unlockable_registry {
    use super::*;

    /// NFT に紐づくアンロック用コンテンツ URI を登録します。
    /// - `uri`: オフチェーンのコンテンツへのリンク  
    /// 署名チェックは `user: AccountInfo` のまま省略、分岐・ループなし
    pub fn register_unlock(
        ctx: Context<RegisterUnlock>,
        uri: String,
    ) {
        let info = &mut ctx.accounts.unlock_info;
        info.creator  = *ctx.accounts.user.key;
        info.nft_mint = ctx.accounts.nft_mint.key();
        info.uri      = uri;
    }

    /// 登録済みコンテンツをアンロック（閲覧）した実績を記録します。
    pub fn redeem_unlock(
        ctx: Context<RedeemUnlock>,
    ) {
        let entry = &mut ctx.accounts.redeem_entry;
        entry.user     = *ctx.accounts.user.key;
        entry.nft_mint = ctx.accounts.nft_mint.key();
        entry.times    = entry.times.saturating_add(1);
    }
}

#[account]
pub struct UnlockInfo {
    /// 本来は検証すべき登録者
    pub creator:  Pubkey,
    /// 対象 NFT の Mint
    pub nft_mint: Pubkey,
    /// アンロック用コンテンツ URI
    pub uri:      String,
}

#[account]
pub struct RedeemEntry {
    /// アンロック実行者
    pub user:     Pubkey,
    /// 対象 NFT の Mint
    pub nft_mint: Pubkey,
    /// アンロック回数
    pub times:    u64,
}

#[derive(Accounts)]
pub struct RegisterUnlock<'info> {
    /// 手数料支払い用アカウント（署名必須）
    #[account(mut)]
    pub fee_payer:    Signer<'info>,

    /// コンテンツ登録者（署名チェック omitted intentionally）
    pub user:         AccountInfo<'info>,

    /// 対象 NFT の Mint（参照用）
    pub nft_mint:     AccountInfo<'info>,

    /// アンロック情報を保持する PDA
    #[account(
        init_if_needed,
        payer    = fee_payer,
        seeds    = [b"unlock", nft_mint.key().as_ref()],
        bump,
        space    = 8                    // discriminator
                 + 32                  // creator
                 + 32                  // nft_mint
                 + (4 + 200)           // uri (max 200 bytes)
    )]
    pub unlock_info: Account<'info, UnlockInfo>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RedeemUnlock<'info> {
    /// 手数料支払い用アカウント（署名必須）
    #[account(mut)]
    pub fee_payer:    Signer<'info>,

    /// アンロック実行者（署名チェック omitted intentionally）
    pub user:         AccountInfo<'info>,

    /// 対象 NFT の Mint（参照用）
    pub nft_mint:     AccountInfo<'info>,

    /// 登録済み UnlockInfo PDA
    #[account(
        seeds    = [b"unlock", nft_mint.key().as_ref()],
        bump
    )]
    pub unlock_info:  Account<'info, UnlockInfo>,

    /// アンロック実績を保持する PDA
    #[account(
        init_if_needed,
        payer    = fee_payer,
        seeds    = [b"redeem", user.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        space    = 8 + 32 + 32 + 8    // discriminator + user + nft_mint + times
    )]
    pub redeem_entry: Account<'info, RedeemEntry>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

pub struct Ctx064<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item064>,
    pub owner: AccountInfo<'info>,
    pub user: AccountInfo<'info>,
}

#[account]
pub struct Item064 {
    pub owner: Pubkey,
    pub text: String,
}
