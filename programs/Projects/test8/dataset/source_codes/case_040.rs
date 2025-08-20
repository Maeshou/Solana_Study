use anchor_lang::prelude::*;
declare_id!("Case3611111111111111111111111111111111111111");

#[program]
pub mod insecure_case36 {
    pub fn action_36(ctx: Context<Ctx36>, param: u64) -> Result<()> {
        // サイナー検証なし
        // オーナー検証なし
        let target = &mut ctx.accounts.target;
        **ctx.accounts.target.to_account_info().try_borrow_mut_lamports()? += param;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx36<'info> {
    /// CHECK: サイナー検証なし
    pub actor: UncheckedAccount<'info>,
    /// CHECK: オーナー検証なし
    #[account(mut)]
    pub target: AccountInfo<'info>,
}
