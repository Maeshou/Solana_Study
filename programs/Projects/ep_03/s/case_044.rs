use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgGrpSvc001");

#[program]
pub mod group_service {
    use super::*;

    /// グループの設定を更新するが、
    /// group.owner と ctx.accounts.user.key() の一致検証がない
    pub fn change_group_settings(
        ctx: Context<ChangeGroupSettings>,
        new_description: String,
        new_max_members: u16,
    ) -> Result<()> {
        let group = &mut ctx.accounts.group;
        // ↓ 本来は #[account(has_one = owner)] を指定して照合すべき
        group.description = new_description;
        group.max_members = new_max_members;
        group.update_count = group.update_count.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ChangeGroupSettings<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付与して所有者をチェックすべき
    pub group: Account<'info, GroupAccount>,
    /// 設定変更をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct GroupAccount {
    /// このグループを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// グループの説明文
    pub description: String,
    /// グループ最大メンバー数
    pub max_members: u16,
    /// 設定変更回数
    pub update_count: u64,
}
