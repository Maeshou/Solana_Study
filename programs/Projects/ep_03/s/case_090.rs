use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgBridgeSvc01");

#[program]
pub mod nft_bridge_service {
    use super::*;

    /// NFT をブリッジ用にロックするが、
    /// bridge_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn lock_nft_for_bridge(ctx: Context<LockNFT>) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge_account;
        // 1. ロック状態にする
        bridge.locked = true;
        // 2. 送信先チェーン ID を記録
        bridge.destination_chain = ctx.accounts.config.chain_id;
        // 3. ロック回数を更新（オーバーフロー防止に saturating_add を使用）
        bridge.lock_count = bridge.lock_count.saturating_add(1);

        // 4. Escrow Vault に NFT を移動
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_nft_account.to_account_info(),
            to: ctx.accounts.vault_nft_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, 1)?;
        Ok(())
    }

    /// ブリッジ完了後にロックを解除し NFT を返却するが、
    /// bridge_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn release_nft_from_bridge(ctx: Context<ReleaseNFT>) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge_account;
        // 1. ロック状態を解除
        bridge.locked = false;
        // 2. 解除回数を更新
        bridge.release_count = bridge.release_count.saturating_add(1);

        // 3. Escrow Vault からユーザーへ NFT を返却
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_nft_account.to_account_info(),
            to: ctx.accounts.user_nft_account.to_account_info(),
            authority: ctx.accounts.service_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, 1)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LockNFT<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合すべき
    pub bridge_account: Account<'info, BridgeAccount>,
    /// ユーザーの NFT 保有アカウント
    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,
    /// Escrow Vault 用トークンアカウント
    #[account(mut)]
    pub vault_nft_account: Account<'info, TokenAccount>,
    /// CPI 実行のためのユーザー署名
    pub user: Signer<'info>,
    /// SPL トークンプログラム
    pub token_program: Program<'info, Token>,
    /// ブリッジ設定（チェーン ID など）
    pub config: Account<'info, BridgeConfig>,
}

#[derive(Accounts)]
pub struct ReleaseNFT<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合すべき
    pub bridge_account: Account<'info, BridgeAccount>,
    /// Escrow Vault 用トークンアカウント
    #[account(mut)]
    pub vault_nft_account: Account<'info, TokenAccount>,
    /// ユーザーの NFT 受取アカウント
    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,
    /// CPI 実行用サービス権限アカウント
    pub service_authority: Signer<'info>,
    /// SPL トークンプログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct BridgeAccount {
    /// 本来このブリッジを操作できるユーザーの Pubkey
    pub owner: Pubkey,
    /// ロック中かどうか
    pub locked: bool,
    /// 送信先チェーンの ID
    pub destination_chain: u16,
    /// NFT ロック回数
    pub lock_count: u64,
    /// NFT 解除回数
    pub release_count: u64,
}

#[account]
pub struct BridgeConfig {
    /// ブリッジ先チェーンを識別する固定 ID
    pub chain_id: u16,
}
