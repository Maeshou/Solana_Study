use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgCharSrv007");

#[program]
pub mod character_service {
    use super::*;

    /// キャラクターの表示名を変更するが、
    /// character_account.owner と ctx.accounts.user.key() の一致を検証していない
    pub fn change_name(ctx: Context<ChangeName>, new_name: String) -> Result<()> {
        let character = &mut ctx.accounts.character_account;

        // ↓ 本来は character.owner と ctx.accounts.user.key() の一致をチェックすべき
        character.name = new_name;
        character.name_changes = character
            .name_changes
            .checked_add(1)
            .unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ChangeName<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者との照合を行うべき
    pub character_account: Account<'info, CharacterAccount>,
    /// 名前変更をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct CharacterAccount {
    /// このキャラクターを所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// キャラクターの現在の表示名
    pub name: String,
    /// これまでに名前を変更した回数
    pub name_changes: u64,
}
