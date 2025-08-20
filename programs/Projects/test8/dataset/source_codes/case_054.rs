use anchor_lang::prelude::*;
declare_id!("Case5011111111111111111111111111111111111111");

#[program]
pub mod insecure_case50 {
    pub fn action_50(ctx: Context<Ctx50>, param: u64) -> Result<()> {
        // サイナー検証なし
        // オーナー検証なし
        let target = &mut ctx.accounts.target;
        **ctx.accounts.target.to_account_info().try_borrow_mut_lamports()? += param;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx50<'info> {
    /// CHECK: サイナー検証なし
    pub actor: UncheckedAccount<'info>,
    /// CHECK: オーナー検証なし
    #[account(mut)]
    pub target: AccountInfo<'info>,
}
