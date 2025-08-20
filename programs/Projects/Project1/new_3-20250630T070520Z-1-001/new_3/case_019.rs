use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgTeamBldr01");

#[program]
pub mod team_builder {
    use super::*;

    /// 3 枚の NFT カードを組み合わせてチームを編成するが、
    /// team_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn assemble_team(
        ctx: Context<AssembleTeam>,
        card1: Pubkey,
        card2: Pubkey,
        card3: Pubkey,
    ) -> Result<()> {
        let team = &mut ctx.accounts.team_account;

        // 1. カード ID を設定
        team.card1 = card1;
        team.card2 = card2;
        team.card3 = card3;

        // 2. 各カードに共通のベース強度を乗じてチームの総合強度を計算
        let base = ctx.accounts.config.base_strength;
        team.total_strength = base
            .checked_mul(3)
            .unwrap();

        // 3. 編成回数をインクリメント
        team.times_assembled = team
            .times_assembled
            .checked_add(1)
            .unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AssembleTeam<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub team_account: Account<'info, TeamAccount>,

    /// チームを組むユーザー（署名者）
    pub user: Signer<'info>,

    /// 編成時のパラメータを保持する設定アカウント
    pub config: Account<'info, TeamConfig>,
}

#[account]
pub struct TeamAccount {
    /// このチームを所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 編成に使用するカードの NFT ミントアドレス
    pub card1: Pubkey,
    pub card2: Pubkey,
    pub card3: Pubkey,
    /// チームの総合強度
    pub total_strength: u64,
    /// これまでの編成回数
    pub times_assembled: u64,
}

#[account]
pub struct TeamConfig {
    /// 各カードあたりのベース強度
    pub base_strength: u64,
}
