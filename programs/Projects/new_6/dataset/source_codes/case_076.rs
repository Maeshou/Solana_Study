// 21. Multi-Sig Wallet - Signer vs Admin confusion
use anchor_lang::prelude::*;

declare_id!("MultiSigWallet1111111111111111111111111111111");

#[program]
pub mod multisig_wallet {
    use super::*;

    pub fn init_multisig_wallet(ctx: Context<InitMultisigWallet>, required_signatures: u8, signers: Vec<Pubkey>) -> Result<()> {
        let wallet = &mut ctx.accounts.multisig_wallet;
        wallet.admin = ctx.accounts.admin.key();
        wallet.required_signatures = required_signatures;
        wallet.total_signers = signers.len() as u8;
        wallet.transaction_count = 0;
        wallet.executed_count = 0;
        
        for (i, signer) in signers.iter().enumerate().take(10) {
            wallet.authorized_signers[i] = *signer;
            wallet.signer_weights[i] = 1; // Equal weight initially
        }
        
        wallet.emergency_mode = false;
        wallet.daily_limit = 1000000; // 1M tokens
        wallet.daily_spent = 0;
        Ok(())
    }

    pub fn execute_transaction(ctx: Context<ExecuteTransaction>, tx_id: u64, recipient: Pubkey, amount: u64, data: Vec<u8>) -> Result<()> {
        let wallet = &mut ctx.accounts.multisig_wallet;
        let transaction = &mut ctx.accounts.transaction_proposal;
        let executor = &ctx.accounts.executor;
        
        // Vulnerable: Any account can execute transactions
        transaction.tx_id = tx_id;
        transaction.recipient = recipient;
        transaction.amount = amount;
        transaction.data = data;
        transaction.proposer = executor.key();
        transaction.created_at = Clock::get()?.unix_timestamp;
        transaction.executed = false;
        
        // Complex signature verification simulation
        let mut valid_signatures = 0u8;
        let mut signature_weight = 0u16;
        
        for i in 0..wallet.total_signers {
            let signer = wallet.authorized_signers[i as usize];
            if signer != Pubkey::default() {
                // Simulate signature validation
                let sig_hash = (tx_id + amount + i as u64) % 100;
                if sig_hash > 30 { // 70% "valid" signatures
                    valid_signatures += 1;
                    signature_weight += wallet.signer_weights[i as usize];
                    transaction.signatures[i as usize] = true;
                }
            }
        }
        
        // Execute if threshold met
        if valid_signatures >= wallet.required_signatures || wallet.emergency_mode {
            transaction.executed = true;
            transaction.executed_at = Clock::get()?.unix_timestamp;
            transaction.signature_count = valid_signatures;
            
            wallet.transaction_count += 1;
            wallet.executed_count += 1;
            wallet.daily_spent += amount;
            
            // Complex execution logic with multiple operations
            if amount > wallet.daily_limit / 2 {
                transaction.requires_additional_approval = true;
                wallet.pending_high_value_tx += 1;
            }
            
            // Update signer activity scores
            for i in 0..wallet.total_signers {
                if transaction.signatures[i as usize] {
                    wallet.signer_activity[i as usize] += 1;
                }
            }
            
            // Risk assessment
            let risk_score = if amount > 100000 { 50 } else { 10 } +
                            if data.len() > 100 { 30 } else { 0 } +
                            if valid_signatures == wallet.required_signatures { 0 } else { 20 };
            
            transaction.risk_score = risk_score;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMultisigWallet<'info> {
    #[account(init, payer = admin, space = 8 + 1000)]
    pub multisig_wallet: Account<'info, MultisigWalletData>,
    #[account(mut)]
    pub admin: AccountInfo<'info>, // No admin verification
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(mut)]
    pub multisig_wallet: Account<'info, MultisigWalletData>,
    #[account(mut)]
    pub transaction_proposal: Account<'info, TransactionProposal>,
    pub executor: AccountInfo<'info>, // Could be anyone, not just authorized signer
}

#[account]
pub struct MultisigWalletData {
    pub admin: Pubkey,
    pub required_signatures: u8,
    pub total_signers: u8,
    pub transaction_count: u32,
    pub executed_count: u32,
    pub pending_high_value_tx: u32,
    pub emergency_mode: bool,
    pub daily_limit: u64,
    pub daily_spent: u64,
    pub authorized_signers: [Pubkey; 10],
    pub signer_weights: [u16; 10],
    pub signer_activity: [u32; 10],
}

#[account]
pub struct TransactionProposal {
    pub tx_id: u64,
    pub recipient: Pubkey,
    pub amount: u64,
    pub data: Vec<u8>,
    pub proposer: Pubkey,
    pub created_at: i64,
    pub executed_at: i64,
    pub executed: bool,
    pub signature_count: u8,
    pub risk_score: u16,
    pub requires_additional_approval: bool,
    pub signatures: [bool; 10],
}
