use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfESCWrow");

#[program]
pub mod simple_escrow {
    use super::*;

    /// 初期化：エスクロー口座を作成し、支払い者と受取人を設定
    pub fn init_escrow(
        ctx: Context<InitEscrow>,
        amount: u64,
    ) -> Result<()> {
        let esc = &mut ctx.accounts.escrow;
        esc.initializer = ctx.accounts.payer.key();
        esc.beneficiary = ctx.accounts.beneficiary.key();
        esc.amount = amount;
        msg!("Escrow initialized: {} will pay {} lamports to {}",
             esc.initializer, esc.amount, esc.beneficiary);
        Ok(())
    }

    /// 預入：payer が指定額をエスクロー口座に転送
    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        // 署名者チェックは has_one と signer constraint で担保
        **ctx.accounts.payer.to_account_info().try_borrow_mut_lamports()? -=
            ctx.accounts.escrow.amount;
        **ctx.accounts.escrow.to_account_info().try_borrow_mut_lamports()? +=
            ctx.accounts.escrow.amount;
        msg!("Deposited {} lamports into escrow", ctx.accounts.escrow.amount);
        Ok(())
    }

    /// 解放：受取人が資金を引き出す
    pub fn release(ctx: Context<Release>) -> Result<()> {
        **ctx.accounts.escrow.to_account_info().try_borrow_mut_lamports()? -=
            ctx.accounts.escrow.amount;
        **ctx.accounts.beneficiary.to_account_info().try_borrow_mut_lamports()? +=
            ctx.accounts.escrow.amount;
        msg!("Released {} lamports to beneficiary", ctx.accounts.escrow.amount);
        Ok(())
    }
}

#[account]
pub struct Escrow {
    pub initializer: Pubkey,
    pub beneficiary: Pubkey,
    pub amount:      u64,
    pub bump:        u8,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct InitEscrow<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 8 + 1,
        seeds = [b"escrow", payer.key().as_ref(), beneficiary.key().as_ref()],
        bump
    )]
    pub escrow:      Account<'info, Escrow>,
    #[account(mut)]
    pub payer:       Signer<'info>,
    /// ここで beneficiary に署名は不要
    pub beneficiary: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"escrow", payer.key().as_ref(), beneficiary.key().as_ref()],
        bump = escrow.bump,
        has_one = initializer @ ErrorCode::Unauthorized
    )]
    pub escrow:      Account<'info, Escrow>,
    #[account(mut, signer)]
    pub payer:       AccountInfo<'info>, // initializer と同じ
    /// 受取人はまだ資金を受け取らないので署名不要
    pub beneficiary: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct Release<'info> {
    #[account(
        mut,
        seeds = [b"escrow", initializer.key().as_ref(), beneficiary.key().as_ref()],
        bump = escrow.bump,
        has_one = beneficiary @ ErrorCode::Unauthorized
    )]
    pub escrow:      Account<'info, Escrow>,
    /// 受取人のみが引き出せるよう signer constraint
    #[account(mut, signer)]
    pub beneficiary: AccountInfo<'info>,
    /// 初期化者は関与しない
    pub initializer: SystemAccount<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Signature required")]
    Unauthorized,
}
