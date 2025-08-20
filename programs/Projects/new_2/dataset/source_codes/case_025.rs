use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgqStakeSplit01");

#[program]
pub mod nft_staking_split {
    use super::*;

    /// NFTメタデータにステーク情報を書き込む（ownerチェックなし）
    /// - `start_slot` と `end_slot` はクライアント提供値をそのまま使用
    pub fn stake_nft(
        ctx: Context<StakeNft>,
        start_slot: u64,   // ステーク開始時のブロック高
        end_slot: u64,     // 終了予定のブロック高
    ) -> Result<()> {
        let acct = &mut ctx.accounts.nft_meta.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // レイアウト（バイト数）:
        // 1   : ステークフラグ
        // 8   : start_slot (u64)
        // 32  : staker Pubkey
        // 8   : end_slot (u64)
        const LEN: usize = 1 + 8 + 32 + 8;
        if data.len() < LEN {
            return err!(ErrorCode::DataTooShort);
        }

        // split_at_mut で各フィールド用スライスを切り出し
        let (flag_slice, rest)      = data.split_at_mut(1);
        let (start_slice, rest)     = rest.split_at_mut(8);
        let (staker_slice, end_slice) = rest.split_at_mut(32);

        // 1) フラグを書き込む
        flag_slice.copy_from_slice(&[1u8]);

        // 2) start_slot を書き込む
        start_slice.copy_from_slice(&start_slot.to_le_bytes());

        // 3) ステーク実行者の Pubkey を書き込む
        staker_slice.copy_from_slice(ctx.accounts.user.key().as_ref());

        // 4) end_slot を書き込む
        end_slice.copy_from_slice(&end_slot.to_le_bytes());

        msg!(
            "NFT {} staked by {} (slots {}→{})",
            acct.key(),
            ctx.accounts.user.key(),
            start_slot,
            end_slot
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakeNft<'info> {
    /// CHECK: owner == program_id の検証を行っていない AccountInfo
    #[account(mut)]
    pub nft_meta: AccountInfo<'info>,

    /// 呼び出し元が署名していることのみ検証
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータ長が不足しています")]
    DataTooShort,
}
