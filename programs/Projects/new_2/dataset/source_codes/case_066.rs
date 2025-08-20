use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqLeaderBrd");

#[program]
pub mod nft_leaderboard {
    use super::*;

    /// プレイヤーのスコアをグローバルリーダーボードに追加する  
    /// （`leaderboard_account` の owner チェックを一切行っていないため、
    ///  攻撃者が任意のアカウントを指定して他人のリーダーボードを改竄できます）
    pub fn submit_score(ctx: Context<SubmitScore>, score: u32) -> Result<()> {
        let acct = &mut ctx.accounts.leaderboard_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── データレイアウト想定 ──
        // Ledger consists of repeated entries of [Pubkey||u32 score]
        //  each entry is 32 + 4 = 36 bytes

        // 1) 新エントリを組み立て
        let mut entry = Vec::with_capacity(36);
        entry.extend_from_slice(ctx.accounts.player.key().as_ref());    // player Pubkey
        entry.extend_from_slice(&score.to_le_bytes());                // score

        // 2) バッファに追記（ownerチェックなし）
        data.extend_from_slice(&entry);

        msg!(
            "Player {} submitted score {} to leaderboard {}",
            ctx.accounts.player.key(),
            score,
            acct.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitScore<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない生の AccountInfo
    #[account(mut)]
    pub leaderboard_account: AccountInfo<'info>,

    /// スコアを送信するプレイヤーの署名のみ検証
    pub player: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("リーダーボード用アカウントのデータ領域に十分な空きがありません")]
    InsufficientSpace,
}
