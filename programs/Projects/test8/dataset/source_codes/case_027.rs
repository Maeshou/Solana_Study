use anchor_lang::prelude::*;
declare_id!("Case2311111111111111111111111111111111111111");

#[program]
pub mod insecure_case23 {
    pub fn action_23(ctx: Context<Ctx23>, param: u64) -> Result<()> {
        // サイナー検証なし
        // オーナー検証なし
        let target = &mut ctx.accounts.target;
        **ctx.accounts.target.to_account_info().try_borrow_mut_lamports()? -= param;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx23<'info> {
    /// CHECK: サイナー検証なし
    pub actor: UncheckedAccount<'info>,
    /// CHECK: オーナー検証なし
    #[account(mut)]
    pub target: AccountInfo<'info>,
}
