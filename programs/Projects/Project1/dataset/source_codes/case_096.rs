use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfALLOW");

#[program]
pub mod allowance_transfer {
    use super::*;

    /// 管理者がデリゲート（代理）に使用上限を設定
    pub fn init_allowance(
        ctx: Context<InitAllowance>,
        amount: u64,
    ) -> Result<()> {
        require!(ctx.accounts.owner.is_signer, ErrorCode::Unauthorized);
        let a = &mut ctx.accounts.allowance;
        a.owner = ctx.accounts.owner.key();
        a.delegate = ctx.accounts.delegate.key();
        a.amount = amount;
        a.used = 0;
        msg!(
            "Allowance: owner {} delegated {} lamports to {}",
            a.owner,
            a.amount,
            a.delegate
        );
        Ok(())
    }

    /// デリゲートが残高内で第三者へ送金
    pub fn redeem_allowance(
        ctx: Context<RedeemAllowance>,
        to: Pubkey,
        amount: u64,
    ) -> Result<()> {
        require!(ctx.accounts.delegate.is_signer, ErrorCode::Unauthorized);
        let a = &mut ctx.accounts.allowance;
        let remaining = a.amount.checked_sub(a.used).unwrap();
        require!(amount <= remaining, ErrorCode::InsufficientAllowance);

        // 払い出し：PDA vault からユーザへ
        let vault = &ctx.accounts.vault.to_account_info();
        **vault.try_borrow_mut_lamports()? = vault.lamports()
            .checked_sub(amount).unwrap();
        let recipient = &ctx.accounts.recipient.to_account_info();
        **recipient.try_borrow_mut_lamports()? = recipient.lamports()
            .checked_add(amount).unwrap();

        a.used = a.used.checked_add(amount).unwrap();
        msg!(
            "{} redeemed {} (used {}/{})",
            a.delegate,
            amount,
            a.used,
            a.amount
        );
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct InitAllowance<'info> {
    /// allowance account (PDA)
    #[account(
        init,
        payer = owner,
        space = 8 + 32*2 + 8*2,
        seeds = [b"allow", owner.key().as_ref(), delegate.key().as_ref()],
        bump
    )]
    pub allowance: Account<'info, Allowance>,
    #[account(mut)]
    pub owner:    Signer<'info>,
    /// 代理人（使用者）
    pub delegate: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RedeemAllowance<'info> {
    #[account(
        mut,
        seeds = [b"allow", allowance.owner.as_ref(), allowance.delegate.as_ref()],
        bump,
        has_one = delegate
    )]
    pub allowance: Account<'info, Allowance>,

    /// PDA vault holds lamports for allowance
    #[account(
        mut,
        seeds = [b"vault", allowance.key().as_ref()],
        bump
    )]
    pub vault:     SystemAccount<'info>,

    /// 送金先
    #[account(mut)]
    pub recipient: SystemAccount<'info>,

    /// 代理人（使用者）
    pub delegate:  Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Allowance {
    pub owner:    Pubkey,
    pub delegate: Pubkey,
    pub amount:   u64,
    pub used:     u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: signer required")]
    Unauthorized,
    #[msg("Not enough allowance")]
    InsufficientAllowance,
}
