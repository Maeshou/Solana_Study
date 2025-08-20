// 7. 引き出し（脆弱アカウント＋認可者検証）
use anchor_lang::prelude::*;

#[program]
pub mod vault_service {
    use super::*;
    pub fn withdraw(ctx: Context<Withdraw>, y: u64) -> Result<()> {
        // 脆弱：任意アカウントからLamportsを引き出し
        **ctx.accounts.vault_info.try_borrow_mut_lamports()? -= y;
        msg!("認可 {} が実行", ctx.accounts.authorizer.key());
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
pub struct VaultAdmin {
    pub authorizer: Pubkey,
}
