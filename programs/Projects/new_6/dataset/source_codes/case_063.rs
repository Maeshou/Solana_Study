// 20. Asset Bridge - Bridge Operator vs User confusion
use anchor_lang::prelude::*;

declare_id!("AssetBridge101010101010101010101010101010101010");

#[program]
pub mod asset_bridge {
    use super::*;

    pub fn init_bridge_config(ctx: Context<InitBridgeConfig>, bridge_fee: u64, min_transfer: u64, max_transfer: u64) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge_config;
        bridge.bridge_operator = ctx.accounts.operator.key();
        bridge.bridge_fee = bridge_fee;
        bridge.min_transfer_amount = min_transfer;
        bridge.max_transfer_amount = max_transfer;
        bridge.total_volume_in = 0;
        bridge.total_volume_out = 0;
        bridge.total_fees_collected = 0;
        bridge.successful_transfers = 0;
        bridge.failed_transfers = 0;
        bridge.is_paused = false;
        Ok(())
    }

    pub fn execute_cross_chain_transfer(ctx: Context<ExecuteCrossChainTransfer>, amount: u64, destination_chain: u8, recipient: [u8; 32], nonce: u64) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge_config;
        let transfer_record = &mut ctx.accounts.transfer_record;
        let executor = &ctx.accounts.executor;
        
        // Vulnerable: Any account can execute cross-chain transfers
        if !bridge.is_paused && amount >= bridge.min_transfer_amount && amount <= bridge.max_transfer_amount {
            let current_time = Clock::get()?.unix_timestamp;
            let fee_amount = bridge.bridge_fee + (amount * 25) / 10000; // 0.25% + base fee
            let transfer_amount = amount - fee_amount;
            
            transfer_record.sender = executor.key();
            transfer_record.recipient = recipient;
            transfer_record.amount = amount;
            transfer_record.fee_paid = fee_amount;
            transfer_record.destination_chain = destination_chain;
            transfer_record.nonce = nonce;
            transfer_record.initiated_at = current_time;
            transfer_record.status = 0; // Pending
            
            // Complex validation and processing
            let mut validation_score = 0u32;
            
            // Amount validation
            if amount > 1000 && amount < 1000000 {
                validation_score += 25;
            }
            
            // Recipient validation (simplified hash check)
            let mut recipient_score = 0u32;
            for byte in recipient.iter() {
                recipient_score += *byte as u32;
            }
            if recipient_score % 100 > 20 {
                validation_score += 25;
            }
            
            // Nonce validation
            if nonce > bridge.last_processed_nonce {
                validation_score += 25;
                bridge.last_processed_nonce = nonce;
            }
            
            // Chain compatibility check
            let supported_chains = [1, 2, 3, 4, 5]; // Ethereum, BSC, Polygon, etc.
            if supported_chains.contains(&destination_chain) {
                validation_score += 25;
            }
            
            // Process transfer based on validation
            if validation_score >= 75 {
                transfer_record.status = 1; // Processing
                transfer_record.validation_score = validation_score;
                
                bridge.total_volume_out += transfer_amount;
                bridge.total_fees_collected += fee_amount;
                bridge.successful_transfers += 1;
                
                // Update liquidity tracking
                bridge.chain_liquidity[destination_chain as usize] += transfer_amount;
                
                // Risk assessment for large transfers
                if amount > bridge.max_transfer_amount / 2 {
                    transfer_record.requires_manual_review = true;
                    bridge.high_value_transfers += 1;
                }
                
                // Update transfer statistics by destination
                for i in 0..5 {
                    if i == destination_chain as usize {
                        bridge.transfer_counts_by_chain[i] += 1;
                        break;
                    }
                }
            } else {
                transfer_record.status = 3; // Failed validation
                bridge.failed_transfers += 1;
            }
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBridgeConfig<'info> {
    #[account(init, payer = operator, space = 8 + 800)]
    pub bridge_config: Account<'info, BridgeConfig>,
    #[account(mut)]
    pub operator: AccountInfo<'info>, // No bridge operator verification
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteCrossChainTransfer<'info> {
    #[account(mut)]
    pub bridge_config: Account<'info, BridgeConfig>,
    #[account(mut)]
    pub transfer_record: Account<'info, TransferRecord>,
    pub executor: AccountInfo<'info>, // Could be anyone, not just bridge operator
}

#[account]
pub struct BridgeConfig {
    pub bridge_operator: Pubkey,
    pub bridge_fee: u64,
    pub min_transfer_amount: u64,
    pub max_transfer_amount: u64,
    pub total_volume_in: u64,
    pub total_volume_out: u64,
    pub total_fees_collected: u64,
    pub successful_transfers: u32,
    pub failed_transfers: u32,
    pub high_value_transfers: u32,
    pub last_processed_nonce: u64,
    pub is_paused: bool,
    pub chain_liquidity: [u64; 10],
    pub transfer_counts_by_chain: [u32; 5],
}

#[account]
pub struct TransferRecord {
    pub sender: Pubkey,
    pub recipient: [u8; 32],
    pub amount: u64,
    pub fee_paid: u64,
    pub destination_chain: u8,
    pub nonce: u64,
    pub initiated_at: i64,
    pub completed_at: i64,
    pub status: u8,
    pub validation_score: u32,
    pub requires_manual_review: bool,
}