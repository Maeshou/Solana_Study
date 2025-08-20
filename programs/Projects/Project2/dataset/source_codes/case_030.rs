
// =====================================
// 9. Multi-Signature Wallet Program
// =====================================
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("99999999999999999999999999999999");

#[program]
pub mod secure_multisig {
    use super::*;

    pub fn create_multisig(
        ctx: Context<CreateMultisig>,
        owners: Vec<Pubkey>,
        threshold: u8,
    ) -> Result<()> {
        // 厳密なowner checkを実装
        let multisig_info = ctx.accounts.multisig.to_account_info();
        require!(
            multisig_info.owner == ctx.program_id,
            ErrorCode::InvalidMultisigOwner
        );

        require!(threshold > 0, ErrorCode::InvalidThreshold);
        require!(
            threshold <= owners.len() as u8,
            ErrorCode::ThresholdTooHigh
        );

        let multisig = &mut ctx.accounts.multisig;
        multisig.owners = owners;
        multisig.threshold = threshold;
        multisig.nonce = 0;

        Ok(())
    }

    pub fn create_transaction(
        ctx: Context<CreateTransaction>,
        destination: Pubkey,
        amount: u64,
    ) -> Result<()> {
        // AccountInfoを使ったowner check
        let multisig_info = ctx.accounts.multisig.to_account_info();
        require!(
            multisig_info.owner == ctx.program_id,
            ErrorCode::InvalidMultisigOwner
        );

        let transaction_info = ctx.accounts.transaction.to_account_info();
        require!(
            transaction_info.owner == ctx.program_id,
            ErrorCode::InvalidTransactionOwner
        );

        let multisig = &ctx.accounts.multisig;
        require!(
            multisig.owners.contains(&ctx.accounts.proposer.key()),
            ErrorCode::NotAnOwner
        );

        let transaction = &mut ctx.accounts.transaction;
        transaction.multisig = ctx.accounts.multisig.key();
        transaction.destination = destination;
        transaction.amount = amount;
        transaction.executed = false;
        transaction.signers = vec![false; multisig.owners.len()];

        // 提案者の署名を記録
        let proposer_index = multisig.owners
            .iter()
            .position(|&owner| owner == ctx.accounts.proposer.key())
            .unwrap();
        transaction.signers[proposer_index] = true;
        transaction.signature_count = 1;

        Ok(())
    }

    pub fn approve_transaction(ctx: Context<ApproveTransaction>) -> Result<()> {
        // 複数のowner checkを実装
        require!(
            ctx.accounts.multisig.to_account_info().owner == ctx.program_id,
            ErrorCode::InvalidMultisigOwner
        );
        require!(
            ctx.accounts.transaction.to_account_info().owner == ctx.program_id,
            ErrorCode::InvalidTransactionOwner
        );

        let multisig = &ctx.accounts.multisig;
        let transaction = &mut ctx.accounts.transaction;

        require!(!transaction.executed, ErrorCode::AlreadyExecuted);
        require!(
            multisig.owners.contains(&ctx.accounts.owner.key()),
            ErrorCode::NotAnOwner
        );

        let owner_index = multisig.owners
            .iter()
            .position(|&owner| owner == ctx.accounts.owner.key())
            .unwrap();

        require!(
            !transaction.signers[owner_index],
            ErrorCode::AlreadySigned
        );

        transaction.signers[owner_index] = true;
        transaction.signature_count += 1;

        Ok(())
    }

    pub fn execute_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
        // 厳格なowner checkを実装
        let multisig_info = ctx.accounts.multisig.to_account_info();
        require!(
            multisig_info.owner == ctx.program_id,
            ErrorCode::InvalidMultisigOwner
        );

        let transaction_info = ctx.accounts.transaction.to_account_info();
        require!(
            transaction_info.owner == ctx.program_id,
            ErrorCode::InvalidTransactionOwner
        );

        require!(
            ctx.accounts.multisig_vault.owner == &token::ID,
            ErrorCode::InvalidVaultOwner
        );

        let multisig = &ctx.accounts.multisig;
        let transaction = &mut ctx.accounts.transaction;

        require!(!transaction.executed, ErrorCode::AlreadyExecuted);
        require!(
            transaction.signature_count >= multisig.threshold,
            ErrorCode::NotEnoughSignatures
        );

        let seeds = &[
            b"multisig",
            multisig.owners[0].as_ref(),
            &[ctx.accounts.multisig.nonce],
        ];
        let signer = &[&seeds[..]];

        let transfer_instruction = Transfer {
            from: ctx.accounts.multisig_vault.to_account_info(),
            to: ctx.accounts.destination_account.to_account_info(),
            authority: ctx.accounts.multisig.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            signer,
        );

        token::transfer(cpi_ctx, transaction.amount)?;

        transaction.executed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMultisig<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 4 + 32 * 10 + 1 + 1, // 最大10人のオーナーを想定
        constraint = multisig.to_account_info().owner == program_id
    )]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTransaction<'info> {
    #[account(constraint = multisig.to_account_info().owner == program_id)]
    pub multisig: Account<'info, Multisig>,
    #[account(
        init,
        payer = proposer,
        space = 8 + 32 + 32 + 8 + 1 + 4 + 10 + 1, // 最大10人のオーナーを想定
        constraint = transaction.to_account_info().owner == program_id
    )]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveTransaction<'info> {
    #[account(constraint = multisig.to_account_info().owner == program_id)]
    pub multisig: Account<'info, Multisig>,
    #[account(
        mut,
        constraint = transaction.to_account_info().owner == program_id
    )]
    pub transaction: Account<'info, Transaction>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(constraint = multisig.to_account_info().owner == program_id)]
    pub multisig: Account<'info, Multisig>,
    #[account(
        mut,
        constraint = transaction.to_account_info().owner == program_id
    )]
    pub transaction: Account<'info, Transaction>,
    #[account(
        mut,
        constraint = multisig_vault.owner == &token::ID
    )]
    pub multisig_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = destination_account.owner == &token::ID
    )]
    pub destination_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Multisig {
    pub owners: Vec<Pubkey>,
    pub threshold: u8,
    pub nonce: u8,
}

#[account]
pub struct Transaction {
    pub multisig: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
    pub executed: bool,
    pub signers: Vec<bool>,
    pub signature_count: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid multisig account owner")]
    InvalidMultisigOwner,
    #[msg("Invalid transaction account owner")]
    InvalidTransactionOwner,
    #[msg("Invalid vault account owner")]
    InvalidVaultOwner,
    #[msg("Invalid threshold")]
    InvalidThreshold,
    #[msg("Threshold too high")]
    ThresholdTooHigh,
    #[msg("Not an owner")]
    NotAnOwner,
    #[msg("Already executed")]
    AlreadyExecuted,
    #[msg("Already signed")]
    AlreadySigned,
    #[msg("Not enough signatures")]
    NotEnoughSignatures,
}