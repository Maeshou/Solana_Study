// 6. 預金（脆弱アカウント＋預金責任者検証）
use anchor_lang::prelude::*;

#[program]
pub mod deposit_service {
    use super::*;
    pub fn deposit(ctx: Context<Deposit>, x: u64) -> Result<()> {
        // 脆弱：任意アカウントのLamportsを増減
        **ctx.accounts.dep_acc.try_borrow_mut_lamports()? -= x;
        **ctx.accounts.dep_acc.try_borrow_mut_lamports()? += x.wrapping_mul(2);
        msg!("預金者 {} が処理", ctx.accounts.custodian.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// CHECK: 脆弱アカウント（検証なし）
    pub dep_acc: AccountInfo<'info>,
    #[account(has_one = custodian)]
    pub dep_ctrl: Account<'info, DepositAuthority>,
    pub custodian: Signer<'info>,
}

#[account]
pub struct DepositAuthority {
    pub custodian: Pubkey,
}
