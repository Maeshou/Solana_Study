use anchor_lang::prelude::*;
declare_id!("Case0861111111111111111111111111111111111111");

#[program]
pub mod case086 {
    use super::*;
    pub fn execute_p2_p_transfer(ctx: Context<P2PTransferContext>) -> Result<()> {
        // Use Case 86: P2P 送金サービス（P2PTransfer）
        // Vulnerable: using UncheckedAccount where P2PTransferAccount is expected
        msg!("Executing execute_p2_p_transfer for P2P 送金サービス（P2PTransfer）");
        // Example logic (dummy operation)
        let mut acct_data = P2PTransferAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct P2PTransferContext<'info> {
    /// CHECK: expecting P2PTransferAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting P2PTransferAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct P2PTransferAccount {
    pub dummy: u64,
}