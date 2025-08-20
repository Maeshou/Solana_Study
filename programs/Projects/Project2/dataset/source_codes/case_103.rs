use anchor_lang::prelude::*;

declare_id!("BankSafe11111111111111111111111111111111");

#[program]
pub mod bank_program_safe {
    use super::*;

    /// 銀行口座(PDA)の初期化
    pub fn initialize(ctx: Context<InitializeAccount>) -> Result<()> {
        let acct = &mut ctx.accounts.bank_account;
        acct.authority = ctx.accounts.authority.key();
        acct.balance   = 0;
        Ok(())
    }

    /// 入金：Lamportsを移動し、状態も増加
    pub fn deposit(ctx: Context<ModifyAccount>, amount: u64) -> Result<()> {
        let acct_info = ctx.accounts.bank_account.to_account_info();
        let auth_info = ctx.accounts.authority.to_account_info();

        // (1) Authorityが十分なLamportsを持っているかチェック
        let auth_lamports = **auth_info.lamports.borrow();
        require!(
            auth_lamports >= amount,
            ErrorCode::InsufficientFunds
        );

        // (2) Lamports移動
        **auth_info.try_borrow_mut_lamports()? -= amount;
        **acct_info.try_borrow_mut_lamports()? -= 0; // Borrow-check workaround
        **acct_info.try_borrow_mut_lamports()? += amount;

        // (3) 状態としての残高を更新
        let acct = &mut ctx.accounts.bank_account;
        acct.balance = acct
            .balance
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }

    /// 出金：状態とLamportsの両方をチェック＆移動
    pub fn withdraw(ctx: Context<ModifyAccount>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.bank_account;
        let acct_info = ctx.accounts.bank_account.to_account_info();
        let auth_info = ctx.accounts.authority.to_account_info();

        // (1) 状態残高チェック
        require!(
            acct.balance >= amount,
            ErrorCode::InsufficientFunds
        );

        // (2) PDAに十分なLamportsがあるかチェック
        let pda_lamports = **acct_info.lamports.borrow();
        require!(
            pda_lamports >= amount,
            ErrorCode::InsufficientFunds
        );

        // (3) Lamports移動
        **acct_info.try_borrow_mut_lamports()? -= amount;
        **auth_info.try_borrow_mut_lamports()? += amount;

        // (4) 状態残高を減算
        acct.balance = acct
            .balance
            .checked_sub(amount)
            .ok_or(ErrorCode::Underflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    /// PDA：["bank", authority], bump付き
    #[account(
        init,
        seeds  = [b"bank", authority.key().as_ref()],
        bump,
        payer  = authority,
        space  = 8 + 32 + 8
    )]
    pub bank_account: Account<'info, BankAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyAccount<'info> {
    /// has_one + seeds でPDAを完全に固定
    #[account(
        mut,
        seeds   = [b"bank", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub bank_account: Account<'info, BankAccount>,

    pub authority: Signer<'info>,
}

#[account]
pub struct BankAccount {
    /// 操作を許可されたユーザー
    pub authority: Pubkey,
    /// 状態上の残高
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("オーバーフローしました")]
    Overflow,
    #[msg("アンダーフローしました")]
    Underflow,
    #[msg("残高が不足しています")]
    InsufficientFunds,
}
