use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfESCWQ");

#[program]
pub mod simple_escrow {
    use super::*;

    /// エスクロー契約を初期化：管理者（イニシエータ）と受益者を登録
    pub fn initialize_escrow(ctx: Context<InitializeEscrow>) -> Result<()> {
        let esc = &mut ctx.accounts.escrow;
        esc.initializer = ctx.accounts.initializer.key();
        esc.beneficiary = ctx.accounts.beneficiary.key();
        msg!(
            "Escrow initialized: initializer={}, beneficiary={}",
            esc.initializer,
            esc.beneficiary
        );
        Ok(())
    }

    /// エスクロー口座に入金：イニシエータ署名で自身からPDAへ移動
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(
            ctx.accounts.initializer.is_signer,
            ErrorCode::Unauthorized
        );
        **ctx.accounts.initializer.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()?       += amount;
        msg!("Deposited {} lamports into escrow vault", amount);
        Ok(())
    }

    /// エスクローから受益者へ引出し：イニシエータ署名でPDAから受益者へ移動
    pub fn release(ctx: Context<Release>) -> Result<()> {
        require!(
            ctx.accounts.initializer.is_signer,
            ErrorCode::Unauthorized
        );
        let vault_balance = **ctx.accounts.vault.to_account_info().lamports.borrow();
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()?       -= vault_balance;
        **ctx.accounts.beneficiary.to_account_info().try_borrow_mut_lamports()? += vault_balance;
        msg!(
            "Released {} lamports to beneficiary {}",
            vault_balance,
            ctx.accounts.beneficiary.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    /// 初期化用エスクロー状態保持アカウント
    #[account(
        init,
        payer = initializer,
        space = 8 + 32 + 32,
        seeds = [b"escrow", initializer.key().as_ref()],
        bump
    )]
    pub escrow:      Account<'info, EscrowState>,

    /// 管理者（イニシエータ）署名
    #[account(mut)]
    pub initializer: Signer<'info>,

    /// 受益者（資金受け取り先）
    pub beneficiary: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// 既存のエスクロー状態
    #[account(
        mut,
        seeds = [b"escrow", initializer.key().as_ref()],
        bump,
        has_one = initializer
    )]
    pub escrow:      Account<'info, EscrowState>,

    /// イニシエータ署名
    pub initializer: Signer<'info>,

    /// 資金保管用PDA
    #[account(
        mut,
        seeds = [b"vault", escrow.key().as_ref()],
        bump,
        // init_if_needed により最初のDeposit時に自動初期化
        init_if_needed,
        payer = initializer,
        space = 8
    )]
    pub vault:       SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Release<'info> {
    /// 既存のエスクロー状態
    #[account(
        mut,
        seeds = [b"escrow", initializer.key().as_ref()],
        bump,
        has_one = initializer
    )]
    pub escrow:      Account<'info, EscrowState>,

    /// イニシエータ署名
    pub initializer: Signer<'info>,

    /// 資金保管用PDA
    #[account(
        mut,
        seeds = [b"vault", escrow.key().as_ref()],
        bump
    )]
    pub vault:       SystemAccount<'info>,

    /// 資金受取人
    #[account(mut, address = escrow.beneficiary)]
    pub beneficiary: SystemAccount<'info>,
}

#[account]
pub struct EscrowState {
    /// イニシエータPubkey
    pub initializer: Pubkey,
    /// 受益者Pubkey
    pub beneficiary: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: initializer signature required")]
    Unauthorized,
}
