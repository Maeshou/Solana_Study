use anchor_lang::prelude::*;
declare_id!("GameTourn1111111111111111111111111111111111");

/// トーナメント情報
#[account]
pub struct Tournament {
    pub organizer: Pubkey, // トーナメント主催者
    pub name:      String, // トーナメント名
}

/// 参加者情報
#[account]
pub struct Participant {
    pub player:     Pubkey, // プレイヤーの公開鍵
    pub tournament: Pubkey, // 本来は Tournament.key() と一致すべき
    pub score:      u64,    // 獲得スコア
}

#[derive(Accounts)]
pub struct RecordScore<'info> {
    /// Tournament.organizer == organizer.key() の検証あり
    #[account(mut, has_one = organizer)]
    pub tournament: Account<'info, Tournament>,

    /// Participant.tournament == tournament.key() の検証が **ない** ため、
    /// 別トーナメント用の Participant を渡されても通ってしまう
    #[account(mut)]
    pub participant: Account<'info, Participant>,

    /// 署名者チェックはあるが、participant.tournament の検証はなし
    pub organizer: Signer<'info>,
}

#[program]
pub mod game_tournament_vuln {
    use super::*;

    /// 参加者のスコアを記録する
    pub fn record_score(ctx: Context<RecordScore>, new_score: u64) -> Result<()> {
        let p = &mut ctx.accounts.participant;

        // 本来は必須：
        // require_keys_eq!(
        //     p.tournament,
        //     ctx.accounts.tournament.key(),
        //     GameError::TournamentMismatch
        // );
        //
        // または
        // #[account(address = tournament.key())]
        // pub participant: Account<'info, Participant>;

        // チェックがないため、不正な Participant を渡されると
        // 任意のトーナメントのスコアを上書きできてしまう
        p.score = new_score;
        msg!(
            "Participant {} score updated to {} in tournament {}",
            p.player,
            p.score,
            ctx.accounts.tournament.key()
        );
        Ok(())
    }
}

#[error_code]
pub enum GameError {
    #[msg("Participant が指定された Tournament と一致しません")]
    TournamentMismatch,
}
