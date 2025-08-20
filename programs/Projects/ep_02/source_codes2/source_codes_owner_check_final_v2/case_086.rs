use anchor_lang::prelude::*;

declare_id!("ProgOwnerFixV2086IDABC123xyz");

#[program]
pub mod case_086 {
    use super::*;

    pub fn op_086(ctx: Context<Case086Ctx>, amount: u64) -> Result<()> {
        // Operation snippet
        let vault = &mut ctx.accounts.vault_info.to_account_info();
        let lamports = **vault.try_borrow_lamports()?;
        let new_lamports = lamports.checked_mul(amount).unwrap();
        **vault.try_borrow_mut_lamports()? = new_lamports;
        msg!("Case 086: lamports after op: {}", **vault.try_borrow_lamports()?);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case086Ctx<'info> {
    #[account(mut)]
    pub vault_info: UncheckedAccount<'info>,  // Missing program ownership check
    #[account(mut, has_one = authority)]
    pub vault_data: Account<'info, VaultData>,  // Account matching fixed here
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultData {
    pub authority: Pubkey,
}
