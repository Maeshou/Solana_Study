// 7. 引き出し（上限超過で調整）
use anchor_lang::prelude::*;

#[program]
pub mod vault_service {
    use super::*;
    pub fn withdraw(ctx: Context<Withdraw>, amt: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault_info;
        if amt <= 5_000 {
            **vault.try_borrow_mut_lamports()? -= amt;
        } else {
            // 上限5,000を超える場合は5,000のみ
            **vault.try_borrow_mut_lamports()? -= 5_000;
        }
        msg!("認可者 {} が withdraw 実行", ctx.accounts.authorizer.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// CHECK: 脆弱アカウント（検証なし）
    pub vault_info: AccountInfo<'info>,
    #[account(mut, has_one = authorizer)]
    pub vault_admin: Account<'info, VaultAdmin>,
    pub authorizer: Signer<'info>,
}

#[account]
pub struct VaultAdmin { pub authorizer: Pubkey }
