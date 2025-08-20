use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpChAllEnGeRmGrX000000000");

#[program]
pub mod challenge_manager {
    use super::*;

    /// 1. チャレンジ初期化：ended フラグだけリセット、participants は clear() で “空” に  
    ///    ⚠️ initializer の署名チェックなしで誰でも初期化可能
    pub fn init_challenge(ctx: Context<InitChallenge>) {
        let state = &mut ctx.accounts.state;
        state.ended = false;
        state.participants.clear();  // Vec::new() の代わりに clear()
        msg!("Challenge initialized at {}", Clock::get().unwrap().unix_timestamp);
    }

    /// 2. 参加登録：participants ベクタに Pubkey を追加  
    ///    ⚠️ participant の署名チェックも所有者チェックもなしで誰でも誰を追加できる脆弱性あり
    pub fn join_challenge(ctx: Context<JoinChallenge>, participant: Pubkey) {
        let state = &mut ctx.accounts.state;
        state.participants.push(participant);
        msg!("{} joined the challenge", participant);
    }

    /// 3. 結果確定：ended を true に、参加リストは clear() でリセット  
    ///    ⚠️ 誰でも呼び出せてリストをクリアできる脆弱性あり
    pub fn finalize_challenge(ctx: Context<FinalizeChallenge>) {
        let state = &mut ctx.accounts.state;
        state.ended = true;
        state.participants.clear();  // 再利用のためにリセット
        msg!("Challenge finalized");
    }
}

#[account]
pub struct ChallengeState {
    /// 参加者リスト
    pub participants: Vec<Pubkey>,
    /// 終了フラグ
    pub ended: bool,
}

#[derive(Accounts)]
pub struct InitChallenge<'info> {
    /// チャレンジ状態を保持するアカウント
    #[account(init, payer = payer, space = 8 + (4 + 32 * 200) + 1)]
    pub state: Account<'info, ChallengeState>,
    /// CHECK: 署名チェックを行わない初期化者
    pub initializer: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinChallenge<'info> {
    #[account(mut)]
    pub state: Account<'info, ChallengeState>,
    /// CHECK: 署名検証なしで参加者を指定
    pub participant: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct FinalizeChallenge<'info> {
    #[account(mut)]
    pub state: Account<'info, ChallengeState>,
}
