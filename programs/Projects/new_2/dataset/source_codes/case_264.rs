use anchor_lang::prelude::*;

declare_id!("VulnEx78000000000000000000000000000000000078");

#[program]
pub mod example78 {
    pub fn filter_above(ctx: Context<Ctx78>, threshold: u8) -> Result<()> {
        // src_acc: OWNER CHECK SKIPPED
        let data = ctx.accounts.src_acc.data.borrow();
        let filtered: Vec<u8> = data.iter().cloned().filter(|&x| x > threshold).collect();

        // dest_acc: has_one = owner
        let mut dst = ctx.accounts.dest_acc.data.borrow_mut();
        dst.clear();
        dst.extend_from_slice(&filtered);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx78<'info> {
    #[account(mut)]
    pub src_acc: AccountInfo<'info>,  // unchecked
    #[account(mut, has_one = owner)]
    pub dest_acc: Account<'info, DataStore>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataStore {
    pub owner: Pubkey,
    pub data: Vec<u8>,
}
