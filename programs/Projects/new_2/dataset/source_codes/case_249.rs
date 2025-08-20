use anchor_lang::prelude::*;

declare_id!("VulnEx56000000000000000000000000000000000056");

#[program]
pub mod legacy_approve {
    pub fn approve(ctx: Context<Ctx6>, code: u32) -> Result<()> {
        // log_ptr: OWNER CHECK SKIPPED
        let ptr = ctx.accounts.log_ptr.clone();
        let mut data = ptr.data.borrow_mut();
        data[0..4].copy_from_slice(&code.to_le_bytes());

        // approval_acc: has_one = signer
        let ap = &mut ctx.accounts.approval_acc;
        ap.codes.push(code);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx6<'info> {
    /// CHECK: 生ポインタ、所有者検証なし
    #[account(mut)]
    pub log_ptr: AccountInfo<'info>,

    #[account(mut, has_one = signer)]
    pub approval_acc: Account<'info, ApprovalAcc>,
    pub signer: Signer<'info>,
}

#[account]
pub struct ApprovalAcc {
    pub signer: Pubkey,
    pub codes: Vec<u32>,
}
