// 6. 預金（入金額でボーナス or 手数料）
use anchor_lang::prelude::*;

#[program]
pub mod deposit_service {
    use super::*;
    pub fn deposit(ctx: Context<Deposit>, amt: u64) -> Result<()> {
        if amt >= 10_000 {
            // 大口なら2倍
            **ctx.accounts.dep_acc.try_borrow_mut_lamports()? += amt * 2;
        } else {
            // 小口なら手数料として1%差し引き
            let fee = amt / 100;
            **ctx.accounts.dep_acc.try_borrow_mut_lamports()? += amt - fee;
        }
        msg!("預金責任者 {} が deposit 実行", ctx.accounts.custodian.key());
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
pub struct DepositAuthority { pub custodian: Pubkey }
