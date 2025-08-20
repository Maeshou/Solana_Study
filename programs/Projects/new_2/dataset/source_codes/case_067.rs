use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqMetaColl01");

#[program]
pub mod nft_metadata_collection {
    use super::*;

    /// NFT メタデータのコレクション情報を書き換える  
    /// （`metadata_account` の owner チェックをまったく行っていないため、  
    ///  攻撃者が任意のアカウントを指定して、  
    ///  他人の NFT を好きなコレクションに無断で再アサインできます）
    pub fn update_collection(
        ctx: Context<UpdateCollection>,
        new_collection: Pubkey,     // 新しいコレクション ID
        new_update_authority: Pubkey, // 新しい更新権限者
    ) -> Result<()> {
        let acct = &mut ctx.accounts.metadata_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── バイトレイアウト想定 ──
        // [0..32]   current mint Pubkey
        // [32..64]  current owner/update_authority Pubkey
        // [64..96]  current collection Pubkey
        // [96..   ] その他メタデータ…

        const HEADER_LEN: usize = 32 + 32 + 32;
        if data.len() < HEADER_LEN {
            return err!(ErrorCode::DataTooShort);
        }

        // 1) collection フィールドを書き換え
        let collection_slice = &mut data[64..96];
        collection_slice.copy_from_slice(&new_collection.to_bytes());

        // 2) update_authority フィールドを書き換え
        let authority_slice = &mut data[32..64];
        authority_slice.copy_from_slice(&new_update_authority.to_bytes());

        msg!(
            "Metadata {} reassigned to collection {} with authority {}",
            acct.key(),
            new_collection,
            new_update_authority
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateCollection<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない生の AccountInfo
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,

    /// 呼び出し元の署名のみを検証
    pub signer: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("メタデータアカウントのデータ長が不足しています")]
    DataTooShort,
}
