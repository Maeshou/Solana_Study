use anchor_lang::prelude::*;
declare_id!("CaseA111111111111111111111111111111111111111");

#[program]
pub mod user_role {
    // ユーザーに新しいロールを付与する関数
    pub fn grant_role(ctx: Context<GrantRole>, role: u8) -> Result<()> {
        // 本体ロジックはそのまま
        let account = &mut ctx.accounts.profile_info;
        account.data[0] = role;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GrantRole<'info> {
    /// Signer チェックをアカウント属性で実施
    #[account(signer)]
    pub authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub profile_info: AccountInfo<'info>,
}