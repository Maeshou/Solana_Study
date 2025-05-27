#[program]
pub mod duplicate_mutable_accounts_insecure {
    use super::*;

    pub fn update(ctx: Context<Update>, a: u64, b: u64) -> Result<()> {
        // 脆弱性: user_aとuser_bが同一アカウントの場合のチェックがありません。
        // 同じアカウントが複数回ミュータブルな状態で使用される可能性があります。
        let user_a = &mut ctx.accounts.user_a;
        let user_b = &mut ctx.accounts.user_b;

        user_a.data = a;
        user_b.data = b;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Update<'info> {
    pub user_a: Account<'info, User>,
    pub user_b: Account<'info, User>,
}

#[account]
pub struct User {
    pub data: u64,
}
