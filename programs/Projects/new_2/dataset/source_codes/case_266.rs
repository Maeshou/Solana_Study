use anchor_lang::prelude::*;

declare_id!("VulnEx80000000000000000000000000000000000080");

#[program]
pub mod example80 {
    pub fn toggle_flag(ctx: Context<Ctx80>) -> Result<()> {
        // flag_buffer: OWNER CHECK SKIPPED
        let mut buf = ctx.accounts.flag_buffer.data.borrow_mut();
        buf[0] = 1 - buf[0];

        // flag_acc: has_one = setter
        let fa = &mut ctx.accounts.flag_acc;
        fa.flag = !fa.flag;
        fa.toggle_count = fa.toggle_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx80<'info> {
    #[account(mut)]
    pub flag_acc: Account<'info, FlagAcc>,  // validated
    pub setter: Signer<'info>,
    #[account(mut)]
    pub flag_buffer: AccountInfo<'info>,    // unchecked
}

#[account]
pub struct FlagAcc {
    pub setter: Pubkey,
    pub flag: bool,
    pub toggle_count: u64,
}
