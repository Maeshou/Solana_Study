// =============================================================================
// 6. Multi-signature Wallet with Owner Verification
// =============================================================================
#[program]
pub mod secure_multisig {
    use super::*;

    pub fn create_multisig(ctx: Context<CreateMultisig>, owners: Vec<Pubkey>, threshold: u8) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;
        multisig.owners = owners;
        multisig.threshold = threshold;
        multisig.nonce = 0;
        multisig.bump = *ctx.bumps.get("multisig").unwrap();
        Ok(())
    }

    pub fn create_transaction(ctx: Context<CreateTransaction>, data: Vec<u8>, accounts: Vec<TransactionAccount>) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;
        let transaction = &mut ctx.accounts.transaction;
        
        transaction.multisig = multisig.key();
        transaction.data = data;
        transaction.accounts = accounts;
        transaction.signers = vec![false; multisig.owners.len()];
        transaction.executed = false;
        transaction.bump = *ctx.bumps.get("transaction").unwrap();
        
        multisig.nonce += 1;
        Ok(())
    }

    pub fn approve(ctx: Context<Approve>) -> Result<()> {
        let multisig = &ctx.accounts.multisig;
        let transaction = &mut ctx.accounts.transaction;
        
        let owner_index = multisig.owners.iter().position(|a| *a == ctx.accounts.owner.key())
            .ok_or(MultisigError::InvalidOwner)?;
        
        transaction.signers[owner_index] = true;
        Ok(())
    }
}

#[account]
pub struct Multisig {
    pub owners: Vec<Pubkey>,
    pub threshold: u8,
    pub nonce: u64,
    pub bump: u8,
}

#[account]
pub struct Transaction {
    pub multisig: Pubkey,
    pub data: Vec<u8>,
    pub accounts: Vec<TransactionAccount>,
    pub signers: Vec<bool>,
    pub executed: bool,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TransactionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Accounts)]
#[instruction(owners: Vec<Pubkey>)]
pub struct CreateMultisig<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 4 + (32 * owners.len()) + 1 + 8 + 1,
        seeds = [b"multisig", payer.key().as_ref()],
        bump
    )]
    pub multisig: Account<'info, Multisig>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(data: Vec<u8>, accounts: Vec<TransactionAccount>)]
pub struct CreateTransaction<'info> {
    #[account(
        mut,
        seeds = [b"multisig", multisig.owners[0].as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, Multisig>,
    
    #[account(
        init,
        payer = proposer,
        space = 8 + 32 + 4 + data.len() + 4 + (accounts.len() * 40) + 4 + multisig.owners.len() + 1 + 1,
        seeds = [b"transaction", multisig.key().as_ref(), &multisig.nonce.to_le_bytes()],
        bump
    )]
    pub transaction: Account<'info, Transaction>,
    
    #[account(
        mut,
        constraint = multisig.owners.contains(&proposer.key()) @ MultisigError::InvalidOwner
    )]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(
        seeds = [b"multisig", multisig.owners[0].as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, Multisig>,
    
    #[account(
        mut,
        seeds = [b"transaction", multisig.key().as_ref(), &(multisig.nonce - 1).to_le_bytes()],
        bump = transaction.bump,
        constraint = transaction.multisig == multisig.key()
    )]
    pub transaction: Account<'info, Transaction>,
    
    #[account(
        constraint = multisig.owners.contains(&owner.key()) @ MultisigError::InvalidOwner
    )]
    pub owner: Signer<'info>,
}

#[error_code]
pub enum MultisigError {
    #[msg("Invalid owner")]
    InvalidOwner,
}