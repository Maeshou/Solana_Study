use anchor_lang::prelude::*;
declare_id!("CaseD444444444444444444444444444444444444444");

#[program]
pub mod vault_minter {
    // Vault に任意の資金を追加する関数
    pub fn mint_funds(ctx: Context<MintFunds>, amount: u64) -> Result<()> {
        // 本体ロジックはそのまま
        let vault = &mut ctx.accounts.vault;
        **vault.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintFunds<'info> {
    /// Signer チェックをアカウント属性で実施
    #[account(signer)]
    pub requester: UncheckedAccount<'info>,

    #[account(mut)]
    pub vault: AccountInfo<'info>,
}