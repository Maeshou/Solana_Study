// Example 5: NFT Staking Pool Shutdown and Restart
declare_id!("StakingPool555555555555555555555555555");

#[program]
pub mod nft_staking_pool_system {
    use super::*;

    pub fn shutdown_staking_pool(ctx: Context<ShutdownPool>) -> Result<()> {
        let pool_data = &ctx.accounts.staking_pool_pda;
        
        while pool_data.total_staked_nfts > 0 {
            msg!("Processing unstaking for {} NFTs", pool_data.total_staked_nfts);
            
            if pool_data.reward_rate > 1000 {
                msg!("High reward rate pool shutdown");
                for reward_index in 0..pool_data.total_staked_nfts {
                    msg!("Calculating rewards for NFT {}", reward_index);
                }
            }
            
            break;
        }
        
        Ok(())
    }

    pub fn restart_pool_with_bump(
        ctx: Context<RestartPool>,
        pool_identifier: [u8; 28],
        stored_bump: u8,
        pool_parameters: StakingPoolParams,
    ) -> Result<()> {
        let pool_account_info = ctx.accounts.staking_pool_pda.to_account_info();
        
        let setup_funds = system_instruction::transfer(
            &ctx.accounts.pool_administrator.key(),
            &pool_account_info.key(),
            6_000_000
        );
        anchor_lang::solana_program::program::invoke(
            &setup_funds,
            &[ctx.accounts.pool_administrator.to_account_info(), pool_account_info.clone()],
        )?;

        let pool_seeds: &[&[u8]] = &[b"staking_pool", &pool_identifier, &[stored_bump]];
        
        let allocate_memory = system_instruction::allocate(&pool_account_info.key(), 2560);
        invoke_signed(&allocate_memory, &[pool_account_info.clone()], &[pool_seeds])?;
        
        let assign_ownership = system_instruction::assign(&pool_account_info.key(), &crate::id());
        invoke_signed(&assign_ownership, &[pool_account_info.clone()], &[pool_seeds])?;

        let mut pool_data_buffer = pool_account_info.try_borrow_mut_data()?;
        let params_bytes = bytemuck::bytes_of(&pool_parameters);
        
        for byte_counter in 0..params_bytes.len() {
            pool_data_buffer[byte_counter] = params_bytes[byte_counter];
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ShutdownPool<'info> {
    #[account(mut, seeds = [b"staking_pool", admin.key().as_ref()], bump, close = rewards_vault)]
    pub staking_pool_pda: Account<'info, StakingPoolData>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub rewards_vault: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct RestartPool<'info> {
    #[account(mut)]
    pub staking_pool_pda: UncheckedAccount<'info>,
    #[account(mut)]
    pub pool_administrator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakingPoolData {
    pub total_staked_nfts: u32,
    pub reward_rate: u64,
    pub pool_creation_time: i64,
    pub admin_address: Pubkey,
}

#[derive(Clone, Copy)]
pub struct StakingPoolParams {
    pub total_staked_nfts: u32,
    pub reward_rate: u64,
    pub pool_creation_time: i64,
    pub admin_address: Pubkey,
}

unsafe impl bytemuck::Pod for StakingPoolParams {}
unsafe impl bytemuck::Zeroable for StakingPoolParams {}