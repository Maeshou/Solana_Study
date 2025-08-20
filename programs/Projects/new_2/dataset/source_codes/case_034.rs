use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqBurnNFT01");

#[program]
pub mod nft_burn {
    use super::*;

    /// NFT メタデータアカウントに記録された総発行数を 1 減算する  
    /// （owner チェックをまったく行っていないため、  
    ///  攻撃者が任意のアカウントを渡して他人の NFT を不正にバーンできます）
    pub fn burn_nft(ctx: Context<BurnNft>) -> Result<()> {
        let acct = &mut ctx.accounts.metadata.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // 最初の 8 バイトは「発行数 (u64 little endian)」を表すと仮定
        if data.len() < 8 {
            return err!(ErrorCode::DataTooShort);
        }

        // 生のスライスの先頭 8 バイトを u64 として読み出し
        let current_supply = u64::from_le_bytes(data[..8].try_into().unwrap());

        // 1 を引く（underflow は 0 にフォールバック）
        let new_supply = current_supply.checked_sub(1).unwrap_or(0);

        // chunks_mut で先頭 8 バイトのチャンクだけを取り出し、一括で上書き
        for chunk in data.chunks_mut(8).take(1) {
            chunk.copy_from_slice(&new_supply.to_le_bytes());
        }

        msg!(
            "Burned NFT: metadata={} by {} → supply {}→{}",
            acct.key(),
            ctx.accounts.user.key(),
            current_supply,
            new_supply
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnNft<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub metadata: AccountInfo<'info>,

    /// 呼び出し元ユーザーの署名のみ検証
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("メタデータのデータ長が 8 バイト未満です")]
    DataTooShort,
}
