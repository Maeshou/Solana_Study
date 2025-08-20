use anchor_lang::prelude::*;
declare_id!("Case4011111111111111111111111111111111111111");

#[program]
pub mod insecure_case40 {
    pub fn action_40(ctx: Context<Ctx40>, param: u64) -> Result<()> {
        // サイナー検証なし
        // オーナー検証なし
        let target = &mut ctx.accounts.target;
        let data = ctx.accounts.target.try_borrow_mut_data()?;
        data[0] ^= param as u8;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx40<'info> {
    /// CHECK: サイナー検証なし
    pub actor: UncheckedAccount<'info>,
    /// CHECK: オーナー検証なし
    #[account(mut)]
    pub target: AccountInfo<'info>,
}
