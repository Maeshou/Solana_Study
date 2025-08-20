use anchor_lang::prelude::*;
declare_id!("Case1911111111111111111111111111111111111111");

#[program]
pub mod insecure_case19 {
    pub fn action_19(ctx: Context<Ctx19>, param: u64) -> Result<()> {
        // サイナー検証なし
        // オーナー検証なし
        let target = &mut ctx.accounts.target;
        let data = ctx.accounts.target.try_borrow_mut_data()?;
        data[0] ^= param as u8;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx19<'info> {
    /// CHECK: サイナー検証なし
    pub actor: UncheckedAccount<'info>,
    /// CHECK: オーナー検証なし
    #[account(mut)]
    pub target: AccountInfo<'info>,
}
