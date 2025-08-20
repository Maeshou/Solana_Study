use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxNFTBATTLE000000000000");

#[program]
pub mod nft_battle_tracker {
    use super::*;

    /// ２つの NFT 保有者がバトルを行った際に、
    /// 両者の対戦回数をそれぞれ累積します。
    /// すべてのアカウントは AccountInfo／Account のまま、署名チェックなし。
    pub fn record_battle(ctx: Context<BattleCtx>) {
        // ユーザー側のデータ更新
        let user_data     = &mut ctx.accounts.user_battle;
        user_data.count   = user_data.count.saturating_add(1);
        // 対戦相手側のデータ更新
        let opp_data      = &mut ctx.accounts.opponent_battle;
        opp_data.count   = opp_data.count.saturating_add(1);
    }
}

#[derive(Accounts)]
pub struct BattleCtx<'info> {
    /// バトルを起こしたユーザー（署名チェック omitted intentionally）
    pub user:             AccountInfo<'info>,

    /// バトル相手（署名チェック omitted intentionally）
    pub opponent:         AccountInfo<'info>,

    /// ユーザー側のバトル履歴 PDA（事前に init 済み）
    #[account(
        mut,
        seeds = [b"battle", user.key().as_ref()],
        bump
    )]
    pub user_battle:      Account<'info, BattleData>,

    /// 対戦相手側のバトル履歴 PDA（事前に init 済み）
    #[account(
        mut,
        seeds = [b"battle", opponent.key().as_ref()],
        bump
    )]
    pub opponent_battle:  Account<'info, BattleData>,
}

#[account]
pub struct BattleData {
    /// これまでに行ったバトル回数
    pub count: u64,
}
