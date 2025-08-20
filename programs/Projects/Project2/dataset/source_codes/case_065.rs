
// 5. Multi-Signature Wallet Implementation
use anchor_lang::prelude::*;

declare_id!("MultiSigWallet11111111111111111111111111111111111");

#[program]
pub mod multi_sig_wallet {
    use super::*;
    
    pub fn create_wallet(ctx: Context<CreateWallet>, threshold: u8, signers: Vec<Pubkey>) -> Result<()> {
        require!(threshold > 0 && threshold as usize <= signers.len(), ErrorCode::InvalidThreshold);
        
        let wallet = &mut ctx.accounts.wallet;
        wallet.threshold = threshold;
        wallet.signers = signers;
        wallet.nonce = 0;
        Ok(())
    }
    
    pub fn propose_transaction(ctx: Context<ProposeTransaction>, to: Pubkey, amount: u64, data: Vec<u8>) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        let transaction = &mut ctx.accounts.transaction;
        
        transaction.to = to;
        transaction.amount = amount;
        transaction.data = data;
        transaction.executed = false;
        transaction.approvals = vec![false; wallet.signers.len()];
        transaction.approval_count = 0;
        
        wallet.nonce += 1;
        Ok(())
    }
    
    pub fn approve_transaction(ctx: Context<ApproveTransaction>) -> Result<()> {
        let wallet = &ctx.accounts.wallet;
        let transaction = &mut ctx.accounts.transaction;
        
        require!(!transaction.executed, ErrorCode::AlreadyExecuted);
        
        // Find signer index
        let signer_index = wallet.signers.iter()
            .position(|&x| x == ctx.accounts.signer.key())
            .ok_or(ErrorCode::NotASigner)?;
        
        require!(!transaction.approvals[signer_index], ErrorCode::AlreadyApproved);
        
        transaction.approvals[signer_index] = true;
        transaction.approval_count += 1;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateWallet<'info> {
    #[account(init, payer = creator, space = 8 + 500)]
    pub wallet: Account<'info, MultiSigWallet>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProposeTransaction<'info> {
    #[account(mut)]
    pub wallet: Account<'info, MultiSigWallet>,
    #[account(init, payer = proposer, space = 8 + 1000, seeds = [b"transaction", wallet.key().as_ref(), &wallet.nonce.to_le_bytes()], bump)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveTransaction<'info> {
    pub wallet: Account<'info, MultiSigWallet>,
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
    pub signer: Signer<'info>,
}

#[account]
pub struct MultiSigWallet {
    pub threshold: u8,
    pub signers: Vec<Pubkey>,
    pub nonce: u64,
}

#[account]
pub struct Transaction {
    pub to: Pubkey,
    pub amount: u64,
    pub data: Vec<u8>,
    pub executed: bool,
    pub approvals: Vec<bool>,
    pub approval_count: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid threshold")]
    InvalidThreshold,
    #[msg("Already executed")]
    AlreadyExecuted,
    #[msg("Not a signer")]
    NotASigner,
    #[msg("Already approved")]
    AlreadyApproved,
}