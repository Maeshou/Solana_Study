use anchor_lang::prelude::*;
use anchor_spl::token::{Burn, Token, burn};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqBurnUnsafe02");

#[program]
pub mod nft_burn_unsafe {
    use super::*;

    /// 指定のアカウントから NFT をバーンする  
    /// （`burn_nft_account` の owner チェックをまったく行っていないため、  
    ///  攻撃者が別プログラム所有のトークンアカウントを指定し、  
    ///  他ユーザーの NFT を無断で消滅させられる脆弱性があります）
    pub fn burn_nft(
        ctx: Context<BurnNft>,
        amount: u64,  // バーンするトークン量（通常は 1）
    ) -> Result<()> {
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            to:   ctx.accounts.burn_nft_account.to_account_info(), // 所有者チェックなし！
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        burn(cpi_ctx, amount)?;

        msg!(
            "Burned {} token(s) from {} by {}",
            amount,
            ctx.accounts.burn_nft_account.key(),
            ctx.accounts.user.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnNft<'info> {
    /// CHECK: `burn_nft_account.owner` == Token プログラム かどうかの検証を行っていない生の AccountInfo
    #[account(mut)]
    pub burn_nft_account: AccountInfo<'info>,

    /// CHECK: `mint.owner` == Token プログラム の検証を行っていない生の AccountInfo
    pub mint: AccountInfo<'info>,

    /// バーン権限を持つ署名者（通常はトークンアカウント所有者）が呼び出す想定
    pub user: Signer<'info>,

    /// SPL Token プログラム CPI 用
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("バーン操作に失敗しました")]
    BurnFailed,
}
