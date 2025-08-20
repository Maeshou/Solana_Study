use anchor_lang::prelude::*;
declare_id!("Case4611111111111111111111111111111111111111");

#[program]
pub mod insecure_case46 {
    pub fn action_46(ctx: Context<Ctx46>, param: u64) -> Result<()> {
        // サイナー検証なし
        // オーナー検証なし
        let target = &mut ctx.accounts.target;
        let data = ctx.accounts.target.try_borrow_mut_data()?;
        data[2] = if data[2] == 0 { 1 } else { 0 };
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx46<'info> {
    /// CHECK: サイナー検証なし
    pub actor: UncheckedAccount<'info>,
    /// CHECK: オーナー検証なし
    #[account(mut)]
    pub target: AccountInfo<'info>,
}
