use anchor_lang::prelude::*;
declare_id!("Case0611111111111111111111111111111111111111");

#[program]
pub mod insecure_case06 {
    pub fn action_06(ctx: Context<Ctx06>, param: u64) -> Result<()> {
        // サイナー検証なし
        // オーナー検証なし
        let target = &mut ctx.accounts.target;
        let data = ctx.accounts.target.try_borrow_mut_data()?;
        data[4..12].copy_from_slice(&param.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx06<'info> {
    /// CHECK: サイナー検証なし
    pub actor: UncheckedAccount<'info>,
    /// CHECK: オーナー検証なし
    #[account(mut)]
    pub target: AccountInfo<'info>,
}
