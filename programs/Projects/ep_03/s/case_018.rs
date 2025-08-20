use anchor_lang::prelude::*;
use anchor_spl::token::{Burn, MintTo, Token, TokenAccount, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgLootBox001");

#[program]
pub mod loot_box_service {
    use super::*;

    /// LootBoxを開封し、報酬NFTをミントするが、
    /// loot_box_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn open_loot_box(ctx: Context<OpenLootBox>) -> Result<()> {
        let box_acc = &mut ctx.accounts.loot_box_account;

        // 1. LootBox NFTをバーン
        let burn_accounts = Burn {
            mint: ctx.accounts.box_mint.to_account_info(),
            from: ctx.accounts.user_box_account.to_account_info(),
            authority: ctx.accounts.service_authority.to_account_info(),
        };
        token::burn(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), burn_accounts),
            1,
        )?;

        // 2. 報酬NFTをミント
        let mint_accounts = MintTo {
            mint: ctx.accounts.reward_mint.to_account_info(),
            to: ctx.accounts.user_reward_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        token::mint_to(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), mint_accounts),
            ctx.accounts.config.reward_amount,
        )?;

        // 3. 開封フラグを更新
        box_acc.opened = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenLootBox<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub loot_box_account: Account<'info, LootBoxAccount>,

    /// LootBox所有者（署名者）
    pub user: Signer<'info>,

    /// LootBox用NFTのMintアカウント
    pub box_mint: Account<'info, Mint>,

    /// ユーザーのLootBoxトークンアカウント
    #[account(mut)]
    pub user_box_account: Account<'info, TokenAccount>,

    /// バーンおよびミント実行用サービス権限
    pub service_authority: Signer<'info>,

    /// 報酬NFTのMintアカウント
    #[account(mut)]
    pub reward_mint: Account<'info, Mint>,

    /// ユーザーの報酬受取用トークンアカウント
    #[account(mut)]
    pub user_reward_account: Account<'info, TokenAccount>,

    /// Mint権限を持つアカウント
    pub mint_authority: Signer<'info>,

    /// SPLトークンプログラム
    pub token_program: Program<'info, Token>,

    /// 報酬数量を保持する設定アカウント
    pub config: Account<'info, LootBoxConfig>,
}

#[account]
pub struct LootBoxAccount {
    /// 本来このLootBoxを所有するユーザーのPubkey
    pub owner: Pubkey,
    /// 開封済みフラグ
    pub opened: bool,
}

#[account]
pub struct LootBoxConfig {
    /// 開封時にミントする報酬NFT数
    pub reward_amount: u64,
}
