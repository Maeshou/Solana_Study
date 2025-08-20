use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqUnstakeNew");

#[program]
pub mod nft_unstaking {
    use super::*;

    /// ステーク済み NFT をアンステーク（lamports を返却しデータをクリア）  
    /// （`unstake_nft_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が自分のアカウントを渡すだけで、  
    ///  他ユーザーのステーク済み NFT を強制的にアンステークできます）
    pub fn unstake_nft(ctx: Context<UnstakeNft>) -> Result<()> {
        // 1) lamports の返却
        let stake_acc = &mut ctx.accounts.unstake_nft_account.to_account_info();
        let recipient = &mut ctx.accounts.user_account.to_account_info();
        let bal       = **stake_acc.lamports.borrow();
        **stake_acc.lamports.borrow_mut() = 0;
        **recipient.lamports.borrow_mut() = recipient
            .lamports()
            .checked_add(bal)
            .unwrap_or(recipient.lamports());

        // 2) バイト列から「ステーク情報」フィールド以外を削除
        //
        // データレイアウト想定：
        // [0]   : u8   ステークフラグ (1=ステーク中)
        // [1..9]: u64  ステーク開始時刻
        // [9..41]: Pubkey ステーカー
        // [41..] : 拡張領域
        let data = &mut stake_acc.data.borrow_mut();
        if data.is_empty() {
            return err!(ErrorCode::NoData);
        }
        // フラグだけ残して以降をゼロクリア
        let (flag_slice, rest) = data.split_at_mut(1);
        // ステークフラグを 0 に戻す
        flag_slice[0] = 0;
        // その他のバイトをまとめてクリア
        rest.fill(0);

        msg!(
            "Unstaked NFT: returned {} lamports to {}",
            bal,
            ctx.accounts.user.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UnstakeNft<'info> {
    /// CHECK: owner == program_id の検証を省略している生の AccountInfo
    #[account(mut)]
    pub unstake_nft_account: AccountInfo<'info>,
    /// lamports を受け取るユーザーアカウント
    #[account(mut)]
    pub user_account:        AccountInfo<'info>,
    /// 実行者が署名していることのみを検証
    pub user:                Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントにデータが存在しません")]
    NoData,
}
