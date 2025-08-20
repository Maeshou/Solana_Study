use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgGuildBk001");

#[program]
pub mod guild_bank_service {
    use super::*;

    /// ギルドバンクに資金を入金するが、
    /// bank_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn deposit_funds(ctx: Context<DepositFunds>, amount: u64) -> Result<()> {
        let bank = &mut ctx.accounts.bank_account;

        // 1. ギルドバンク残高に加算
        bank.balance = bank.balance.checked_add(amount).unwrap();

        // 2. ユーザーからバンクの財務口座へLamportsを移動
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() -= amount;
        **ctx.accounts.bank_treasury.to_account_info().lamports.borrow_mut() += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositFunds<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub bank_account: Account<'info, GuildBank>,

    /// ギルドの財務を管理する口座（Lamports保管先）
    #[account(mut)]
    pub bank_treasury: AccountInfo<'info>,

    /// 入金を実行するユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct GuildBank {
    /// 本来このバンクを操作できるギルドリーダーの Pubkey
    pub owner: Pubkey,
    /// 現在の総資金残高
    pub balance: u64,
}
