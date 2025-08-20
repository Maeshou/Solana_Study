use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Transfer, transfer};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqCrossGame01");

#[program]
pub mod cross_game_supply {
    use super::*;

    /// 他ゲームとのブリッジを介して NFT を供給／引き渡す  
    /// （`bridge_token_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が偽のブリッジアカウントを指定して、  
    ///  他人のアイテムを横流し・複製できます）
    pub fn bridge_item(
        ctx: Context<BridgeItem>,
        amount: u64,  // 供給／転送するトークン量（通常は 1）
    ) -> Result<()> {
        // 1) 元ゲーム側トークン→ブリッジアカウントへ転送
        let cpi1 = Transfer {
            from:      ctx.accounts.source_game_token.to_account_info(),
            to:        ctx.accounts.bridge_token_account.to_account_info(), // owner チェックなし！
            authority: ctx.accounts.user.to_account_info(),
        };
        transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi1),
            amount,
        )?;

        // 2) ブリッジアカウント→宛先ゲーム側トークンへ転送
        let cpi2 = Transfer {
            from:      ctx.accounts.bridge_token_account.to_account_info(), // 同じく owner 未検証
            to:        ctx.accounts.dest_game_token.to_account_info(),
            authority: ctx.accounts.bridge_authority.to_account_info(),
        };
        transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi2),
            amount,
        )?;

        msg!(
            "Bridged {} tokens from {} to {} via {}",
            amount,
            ctx.accounts.source_game_token.key(),
            ctx.accounts.dest_game_token.key(),
            ctx.accounts.bridge_token_account.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BridgeItem<'info> {
    /// 元ゲームのユーザー保有トークンアカウント（owner チェックなし）
    #[account(mut)]
    pub source_game_token:      AccountInfo<'info>,

    /// ブリッジ用トークンアカウント（owner == Token プログラム の検証を省略）
    #[account(mut)]
    pub bridge_token_account:   AccountInfo<'info>,

    /// 宛先ゲーム側の受取トークンアカウント（owner チェックなし）
    #[account(mut)]
    pub dest_game_token:        AccountInfo<'info>,

    /// 元ゲームユーザーの署名（burn/mint 権限を持つ前提）
    pub user:                   Signer<'info>,

    /// ブリッジアカウントの transfer 権限を持つ想定の署名者
    pub bridge_authority:       Signer<'info>,

    /// SPL Token プログラム CPI 用
    pub token_program:          Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("CPI に失敗しました")]
    TransferFailed,
}
