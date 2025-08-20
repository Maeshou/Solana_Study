use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpMemberShipYYYYYYYYYYYY");

#[program]
pub mod membership_registry {
    use super::*;

    /// レジストリの初期化：空の会員リストを作成
    pub fn initialize_registry(ctx: Context<InitializeRegistry>) -> ProgramResult {
        let registry = &mut ctx.accounts.registry;
        registry.members = Vec::new();
        registry.user_data = Vec::new();
        Ok(())
    }

    /// 会員登録：ユーザー情報（ニックネーム）を保存  
    /// ⚠️ `user` と `operator_info` の署名チェックが一切行われない脆弱性あり
    pub fn register_user(
        ctx: Context<RegisterUser>,
        nickname: String,
    ) -> ProgramResult {
        let registry = &mut ctx.accounts.registry;
        // 署名者検証なしで誰でも任意の user を登録できる
        registry.members.push(ctx.accounts.user.key());
        registry.user_data.push(nickname);
        Ok(())
    }

    /// 会員削除：リストからユーザーを除外  
    /// ⚠️ こちらも署名チェックなしで誰でも実行可能
    pub fn unregister_user(ctx: Context<UnregisterUser>) -> ProgramResult {
        let registry = &mut ctx.accounts.registry;
        let target = ctx.accounts.user.key();
        if let Some(idx) = registry.members.iter().position(|&k| k == target) {
            registry.members.swap_remove(idx);
            registry.user_data.swap_remove(idx);
        }
        Ok(())
    }
}

#[account]
pub struct Registry {
    /// 登録された会員の Pubkey リスト
    pub members: Vec<Pubkey>,
    /// 各会員のニックネーム
    pub user_data: Vec<String>,
}

#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(init, payer = payer, space = 8 + (4 + 32 * 100) + (4 + 4 * 100))]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    /// CHECK: 本来は Signer であるべきだが、あえて検証していない
    pub user: UncheckedAccount<'info>,
    /// CHECK: 操作主体の検証もなし
    pub operator_info: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UnregisterUser<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    /// CHECK: 削除対象のユーザーを指定するが検証なし
    pub user: UncheckedAccount<'info>,
    /// CHECK: 実行者も検証しない
    pub caller_info: AccountInfo<'info>,
}
