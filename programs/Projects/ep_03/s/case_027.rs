use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgClanSvc01");

#[program]
pub mod clan_service {
    use super::*;

    /// クランの名称とエンブレムを更新するが、
    /// clan_account.owner と ctx.accounts.leader.key() の照合チェックがない
    pub fn update_clan_profile(
        ctx: Context<UpdateClanProfile>,
        new_name: String,
        new_emblem: Pubkey,
    ) -> Result<()> {
        let clan = &mut ctx.accounts.clan_account;

        // ↓ 本来は #[account(has_one = owner)] を付けて所有者照合を行うべき
        clan.name = new_name;
        clan.emblem = new_emblem;

        // 更新履歴カウンタをインクリメント
        clan.update_count = clan
            .update_count
            .checked_add(1)
            .unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateClanProfile<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合チェックを入れるべき
    pub clan_account: Account<'info, ClanAccount>,

    /// クランリーダー（署名者）
    pub leader: Signer<'info>,
}

#[account]
pub struct ClanAccount {
    /// このクランを管理するべきリーダーの Pubkey
    pub owner: Pubkey,
    /// クラン名
    pub name: String,
    /// クランエンブレムとして使う NFT ミントアドレス
    pub emblem: Pubkey,
    /// プロフィール更新回数
    pub update_count: u64,
}
