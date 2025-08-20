use anchor_lang::prelude::*;
declare_id!("Case3211111111111111111111111111111111111111");

#[program]
pub mod insecure_case32 {
    pub fn action_32(ctx: Context<Ctx32>, param: u64) -> Result<()> {
        // サイナー検証なし
        // オーナー検証なし
        let target = &mut ctx.accounts.target;
        let data = ctx.accounts.target.try_borrow_mut_data()?;
        data[2] = if data[2] == 0 { 1 } else { 0 };
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx32<'info> {
    /// CHECK: サイナー検証なし
    pub actor: UncheckedAccount<'info>,
    /// CHECK: オーナー検証なし
    #[account(mut)]
    pub target: AccountInfo<'info>,
}
