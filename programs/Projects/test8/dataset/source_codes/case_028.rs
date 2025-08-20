use anchor_lang::prelude::*;
declare_id!("Case2411111111111111111111111111111111111111");

#[program]
pub mod insecure_case24 {
    pub fn action_24(ctx: Context<Ctx24>, param: u64) -> Result<()> {
        // サイナー検証なし
        // オーナー検証なし
        let target = &mut ctx.accounts.target;
        let data = ctx.accounts.target.try_borrow_mut_data()?;
        for byte in data.iter_mut() { *byte = 0; }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx24<'info> {
    /// CHECK: サイナー検証なし
    pub actor: UncheckedAccount<'info>,
    /// CHECK: オーナー検証なし
    #[account(mut)]
    pub target: AccountInfo<'info>,
}
