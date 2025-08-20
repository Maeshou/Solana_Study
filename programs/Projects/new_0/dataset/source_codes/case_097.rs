use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfSTKNG");

#[program]
pub mod simple_staking {
    use super::*;

    /// ユーザーがステーキング用アカウントを作成し、指定量を預け入れ
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        // 署名チェック
        require!(ctx.accounts.user.is_signer, ErrorCode::Unauthorized);

        // Anchor の CPIContext を使って型安全に transfer
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to:   ctx.accounts.stake_rec.to_account_info(),
            },
        );
        system_program::transfer(cpi_ctx, amount)?;

        // データとして預け入れ量を記録
        let rec = &mut ctx.accounts.stake_rec;
        rec.user   = ctx.accounts.user.key();
        rec.amount = rec.amount.checked_add(amount).unwrap();

        msg!("Staked {} lamports for {}", amount, rec.user);
        Ok(())
    }

    /// ユーザーがステーキングを解除し、全額を払い戻し
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        // 署名チェック
        require!(ctx.accounts.user.is_signer, ErrorCode::Unauthorized);

        let rec = &mut ctx.accounts.stake_rec;
        let amount = rec.amount;
        rec.amount = 0;

        // CPI で払い戻し
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.stake_rec.to_account_info(),
                to:   ctx.accounts.user.to_account_info(),
            },
        );
        system_program::transfer(cpi_ctx, amount)?;

        msg!("Unstaked {} lamports for {}", amount, rec.user);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Stake<'info> {
    /// ステーク記録用PDA（ユーザーごとに一意）
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"stake_rec", user.key().as_ref()],
        bump
    )]
    pub stake_rec: Account<'info, StakeRecord>,

    /// ステーキングを行うユーザー
    #[account(mut)]
    pub user: Signer<'info>,

    /// システムプログラム（必ずここしか呼ばない）
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    /// ステーク記録用PDA（払い戻し時）
    #[account(
        mut,
        seeds = [b"stake_rec", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub stake_rec: Account<'info, StakeRecord>,

    /// ステーキングを解除するユーザー
    pub user: Signer<'info>,

    /// システムプログラム（必ずここしか呼ばない）
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakeRecord {
    pub user:   Pubkey, // ステーキングしたユーザー
    pub amount: u64,    // 預け入れ量
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: signature missing")]
    Unauthorized,
}
