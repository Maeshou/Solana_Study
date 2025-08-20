use anchor_lang::prelude::*;
declare_id!("Case3311111111111111111111111111111111111111");

#[program]
pub mod insecure_case33 {
    pub fn action_33(ctx: Context<Ctx33>, param: u64) -> Result<()> {
        // サイナー検証なし
        // オーナー検証なし
        let target = &mut ctx.accounts.target;
        let data = ctx.accounts.target.try_borrow_mut_data()?;
        data[0] ^= param as u8;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx33<'info> {
    /// CHECK: サイナー検証なし
    pub actor: UncheckedAccount<'info>,
    /// CHECK: オーナー検証なし
    #[account(mut)]
    pub target: AccountInfo<'info>,
}
