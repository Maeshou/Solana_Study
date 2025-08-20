use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpTop5LeadErboardMgrXYZ");

#[program]
pub mod leaderboard_manager {
    use super::*;

    /// 初期化：トップ5スロットにデフォルトのプレイヤーとスコアを設定  
    /// ⚠️ initializer の署名チェックなしの脆弱性あり
    pub fn init_leaderboard(
        ctx: Context<InitLeaderboard>,
        default_scores: [u64; 5],
    ) -> ProgramResult {
        let lb = &mut ctx.accounts.leaderboard;
        lb.players = [Pubkey::default(); 5];
        lb.scores = default_scores;
        lb.bonus_claimed = [false; 5];
        Ok(())
    }

    /// スロット更新：指定スロットのプレイヤーとスコアを直接書き換え  
    /// ⚠️ operator（AccountInfo）に対する署名チェックも所有者検証もなし
    pub fn update_slot(
        ctx: Context<UpdateSlot>,
        slot: u8,
        player: Pubkey,
        score: u64,
    ) -> ProgramResult {
        let lb = &mut ctx.accounts.leaderboard;
        let idx = slot as usize;
        lb.players[idx] = player;
        lb.scores[idx] = score;
        Ok(())
    }

    /// ボーナス請求：指定スロットのスコアを返し、請求フラグを立てる  
    /// ⚠️ claimer（UncheckedAccount）に対する署名チェックなし
    pub fn claim_bonus(
        ctx: Context<ClaimBonus>,
        slot: u8,
    ) -> u64 {
        let lb = &mut ctx.accounts.leaderboard;
        let idx = slot as usize;
        // ボーナスはスコアそのまま返す想定
        let bonus = lb.scores[idx];
        lb.bonus_claimed[idx] = true;
        bonus
    }
}

#[account]
pub struct Leaderboard {
    /// 上位5プレイヤーのPubkeyスロット
    pub players: [Pubkey; 5],
    /// 各プレイヤーのスコア
    pub scores: [u64; 5],
    /// ボーナス請求済みフラグ
    pub bonus_claimed: [bool; 5],
}

#[derive(Accounts)]
pub struct InitLeaderboard<'info> {
    #[account(init, payer = payer, space = 8 + (32*5) + (8*5) + (1*5))]
    pub leaderboard: Account<'info, Leaderboard>,
    /// CHECK: 初期化者の署名検証なし
    pub initializer: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateSlot<'info> {
    #[account(mut)]
    pub leaderboard: Account<'info, Leaderboard>,
    /// CHECK: operator の署名検証なし
    pub operator: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ClaimBonus<'info> {
    #[account(mut)]
    pub leaderboard: Account<'info, Leaderboard>,
    /// CHECK: claimer の署名検証なし
    pub claimer: UncheckedAccount<'info>,
}
