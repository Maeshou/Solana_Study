use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭に定義 ──
#[account]
#[derive(Default)]
pub struct LeaderboardEntry {
    pub round:       u64,             // ラウンド番号
    pub players:     Vec<Pubkey>,     // プレイヤー一覧
    pub scores:      Vec<u64>,        // 各プレイヤーのスコア
    pub created_at:  i64,             // 作成時刻
    pub updated_at:  i64,             // 更新時刻
}

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUO");

#[program]
pub mod leaderboard_manager {
    use super::*;

    /// リーダーボード初期化：round をセットし、空のリストと時刻を登録
    pub fn initialize_leaderboard(
        ctx: Context<InitializeLeaderboard>,
        round: u64,
    ) -> Result<()> {
        let entry = &mut ctx.accounts.leaderboard;
        let now   = ctx.accounts.clock.unix_timestamp;

        *entry = LeaderboardEntry {
            round,
            created_at: now,
            updated_at: now,
            // players, scores は Default::default() による空ベクタ
            ..Default::default()
        };
        Ok(())
    }

    /// スコア投稿：既存データに対して挿入とソートをおこなう
    pub fn submit_score(
        ctx: Context<ModifyLeaderboard>,
        player: Pubkey,
        score: u64,
    ) -> Result<()> {
        let entry = &mut ctx.accounts.leaderboard;
        let now   = ctx.accounts.clock.unix_timestamp;

        // 同一プレイヤーがいれば上書き、いなければ追加
        if let Some(idx) = entry.players.iter().position(|&p| p == player) {
            entry.scores[idx] = score;
        } else {
            entry.players.push(player);
            entry.scores.push(score);
        }

        // スコア降順にソート
        let mut combined: Vec<(Pubkey,u64)> =
            entry.players.iter().cloned()
                .zip(entry.scores.iter().cloned())
                .collect();
        combined.sort_by(|a,b| b.1.cmp(&a.1));
        entry.players = combined.iter().map(|(p,_)| *p).collect();
        entry.scores  = combined.iter().map(|(_,s)| *s).collect();

        entry.updated_at = now;
        Ok(())
    }

    /// トップ N の取得（ローカル処理例）  
    /// on‐chain ではあまり使わない想定ですが、Edge case の処理例として。
    pub fn trim_leaderboard(
        ctx: Context<ModifyLeaderboard>,
        top_n: u8,
    ) -> Result<()> {
        let entry = &mut ctx.accounts.leaderboard;
        if entry.players.len() > top_n as usize {
            entry.players.truncate(top_n as usize);
            entry.scores.truncate(top_n as usize);
            entry.updated_at = ctx.accounts.clock.unix_timestamp;
        }
        Ok(())
    }
}

// ── コンテキスト定義は末尾に ──
#[derive(Accounts)]
#[instruction(round: u64)]
pub struct InitializeLeaderboard<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"leaderboard", authority.key().as_ref(), &round.to_le_bytes()],
        bump,
        // space の計算例：8+8 + (4+10*32) + (4+10*8) + 8 + 8
        space = 8 + 8 + 4 + 10*32 + 4 + 10*8 + 8 + 8
    )]
    pub leaderboard: Account<'info, LeaderboardEntry>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyLeaderboard<'info> {
    #[account(
        mut,
        seeds = [b"leaderboard", authority.key().as_ref(), &leaderboard.round.to_le_bytes()],
        bump = leaderboard.bump,
        has_one = authority
    )]
    pub leaderboard: Account<'info, LeaderboardEntry>,

    #[account(signer)]
    pub authority: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
