use anchor_lang::prelude::*;

declare_id!("EscrowSv1414141414141414141414141414141414");

#[program]
pub mod escrow_service {
    use super::*;

    /// エスクロー初期化
    pub fn init_escrow(ctx: Context<InitEscrow>, amount: u64) -> Result<()> {
        let esc = &mut ctx.accounts.escrow;
        esc.depositor = ctx.accounts.depositor.key();
        esc.amount = amount;
        esc.is_ready = false;
        emit!(EscrowCreated { depositor: esc.depositor, amount });
        Ok(())
    }

    /// デポジット：指定量を転送し準備完了
    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        let esc = &mut ctx.accounts.escrow;
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.depositor.key(),
            &ctx.accounts.escrow_acc.key(),
            esc.amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.depositor.to_account_info(),
                ctx.accounts.escrow_acc.to_account_info(),
            ],
        )?;
        esc.is_ready = true;
        emit!(Deposited { by: esc.depositor, amount: esc.amount });
        Ok(())
    }

    /// 解放：受取人への払戻し
    pub fn release(ctx: Context<Release>) -> Result<()> {
        let esc = &ctx.accounts.escrow;
        require!(esc.is_ready, ErrorCode::NotReady);
        **ctx.accounts.recipient.lamports.borrow_mut() += esc.amount;
        **ctx.accounts.escrow_acc.lamports.borrow_mut() -= esc.amount;
        emit!(Released { to: ctx.accounts.recipient.key(), amount: esc.amount });
        Ok(())
    }

    /// 返還：デポジットなしなら戻せない
    pub fn refund(ctx: Context<Release>) -> Result<()> {
        let esc = &ctx.accounts.escrow;
        require!(!esc.is_ready, ErrorCode::AlreadyDeposited);
        **ctx.accounts.depositor.lamports.borrow_mut() += esc.amount;
        **ctx.accounts.escrow_acc.lamports.borrow_mut() -= esc.amount;
        emit!(Refunded { to: esc.depositor, amount: esc.amount });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(init, payer = depositor, space = 8 + 32 + 8 + 1)]
    pub escrow: Account<'info, EscrowData>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(mut)]
    pub escrow_acc: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub escrow: Account<'info, EscrowData>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(mut)]
    pub escrow_acc: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Release<'info> {
    pub escrow: Account<'info, EscrowData>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    #[account(mut)]
    pub depositor: Signer<'info>,
}

#[account]
pub struct EscrowData {
    pub depositor: Pubkey,
    pub amount: u64,
    pub is_ready: bool,
}

#[event]
pub struct EscrowCreated {
    pub depositor: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Deposited {
    pub by: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Released {
    pub to: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Refunded {
    pub to: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("まだ転送されていません")]
    NotReady,
    #[msg("既に転送済みです")]
    AlreadyDeposited,
}
