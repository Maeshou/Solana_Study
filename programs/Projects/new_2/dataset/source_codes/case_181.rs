use anchor_lang::prelude::*;

declare_id!("OwnChkC3000000000000000000000000000000003");

#[program]
pub mod player_status {
    pub fn update_stats(
        ctx: Context<UpdateStats>,
        xp_gain: u64,
        hp_loss: u64,
    ) -> Result<()> {
        let p = &mut ctx.accounts.player;
        // has_one で player.owner を検証
        p.xp = p.xp.saturating_add(xp_gain);
        p.hp = p.hp.saturating_sub(hp_loss);
        p.update_count = p.update_count.saturating_add(1);

        // backup_acc は unchecked でデータ丸ごとコピー
        let src = ctx.accounts.player.to_account_info().data.borrow();
        let mut dst = ctx.accounts.backup_acc.data.borrow_mut();
        dst.clone_from_slice(&src);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateStats<'info> {
    #[account(mut, has_one = owner)]
    pub player: Account<'info, PlayerData>,
    pub owner: Signer<'info>,
    /// CHECK: バックアップアカウント、所有者検証なし
    #[account(mut)]
    pub backup_acc: AccountInfo<'info>,
}

#[account]
pub struct PlayerData {
    pub owner: Pubkey,
    pub xp: u64,
    pub hp: u64,
    pub update_count: u64,
}
