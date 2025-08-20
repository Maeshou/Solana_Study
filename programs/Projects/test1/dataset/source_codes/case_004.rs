use anchor_lang::prelude::*;
declare_id!("CaseD444444444444444444444444444444444444444");

#[program]
pub mod vault_minter {
    // Vault に任意の資金を追加する関数
    pub fn mint_funds(ctx: Context<MintFunds>, amount: u64) -> Result<()> {
        // リクエスト元のSignerチェックを行わない
        let vault = &mut ctx.accounts.vault;
        // ownerチェックせず、Lamportsを直接増加
        **vault.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintFunds<'info> {
    /// CHECK: 誰でも呼び出せる
    pub requester: UncheckedAccount<'info>,
    /// CHECK: プログラム所有者チェックがない
    #[account(mut)]
    pub vault: AccountInfo<'info>,
}
