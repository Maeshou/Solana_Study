use anchor_lang::prelude::*;
declare_id!("Case2811111111111111111111111111111111111111");

#[program]
pub mod insecure_case28 {
    pub fn action_28(ctx: Context<Ctx28>, param: u64) -> Result<()> {
        // サイナー検証なし
        // オーナー検証なし
        let target = &mut ctx.accounts.target;
        let data = ctx.accounts.target.try_borrow_mut_data()?;
        let mut cnt = u64::from_le_bytes(data[8..16].try_into().unwrap());
        cnt = cnt.wrapping_add(param);
        data[8..16].copy_from_slice(&cnt.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx28<'info> {
    /// CHECK: サイナー検証なし
    pub actor: UncheckedAccount<'info>,
    /// CHECK: オーナー検証なし
    #[account(mut)]
    pub target: AccountInfo<'info>,
}
