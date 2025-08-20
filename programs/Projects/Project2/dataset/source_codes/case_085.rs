use anchor_lang::prelude::*;

declare_id!("VaultMgr1616161616161616161616161616161616");

#[program]
pub mod vault_manager_service {
    use super::*;

    /// 預入：プログラム由来アドレスに送金
    pub fn deposit(ctx: Context<DepositLamports>, amount: u64) -> Result<()> {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.vault_acc.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.vault_acc.to_account_info(),
            ],
        )?;
        emit!(Deposited { by: ctx.accounts.user.key(), amount });
        Ok(())
    }

    /// 引出：元送金者のみ戻せる
    pub fn withdraw(ctx: Context<WithdrawLamports>) -> Result<()> {
        let vault_ai = ctx.accounts.vault_acc.to_account_info();
        let sender = ctx.accounts.user.key();
        require_keys_eq!(ctx.accounts.record.depositor, sender, ErrorCode::NoAuth);
        let lamports = **vault_ai.lamports.borrow();
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += lamports;
        **vault_ai.lamports.borrow_mut() = 0;
        emit!(Withdrawn { to: sender, amount: lamports });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositLamports<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub record: Account<'info, LamportRecord>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub vault_acc: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawLamports<'info> {
    #[account(mut)]
    pub record: Account<'info, LamportRecord>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub vault_acc: AccountInfo<'info>,
}

#[account]
pub struct LamportRecord {
    pub depositor: Pubkey,
}

#[event]
pub struct Deposited {
    pub by: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Withdrawn {
    pub to: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("許可がありません")]
    NoAuth,
}
