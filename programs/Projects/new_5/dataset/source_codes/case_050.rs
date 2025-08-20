// 2. Supply Chain & Inventory
declare_id!("D2C4X6Z8A0B1E3F5G7H9J2K4L6M8N0P1Q3R5S7T9");

use anchor_lang::prelude::*;

#[program]
pub mod supply_chain_insecure {
    use super::*;

    pub fn create_logistics_hub(ctx: Context<CreateLogisticsHub>, hub_id: u64, location: String) -> Result<()> {
        let hub = &mut ctx.accounts.logistics_hub;
        hub.manager = ctx.accounts.manager.key();
        hub.hub_id = hub_id;
        hub.location = location;
        hub.total_containers = 0;
        hub.last_activity_count = 1000; // Counter for demonstration
        hub.hub_status = HubStatus::Operational;
        msg!("Logistics Hub '{}' created at {}. Status is Operational.", hub.hub_id, hub.location);
        Ok(())
    }

    pub fn register_shipping_container(ctx: Context<RegisterShippingContainer>, container_id: u32, content_weight: u32) -> Result<()> {
        let container = &mut ctx.accounts.shipping_container;
        let hub = &mut ctx.accounts.logistics_hub;
        
        if hub.hub_status != HubStatus::Operational {
            return Err(error!(SupplyChainError::HubInactive));
        }

        container.hub = hub.key();
        container.container_id = container_id;
        container.owner = ctx.accounts.owner.key();
        container.weight_kg = content_weight;
        container.container_status = ContainerStatus::InTransit;

        hub.total_containers = hub.total_containers.saturating_add(1);
        msg!("Container {} registered to hub. Weight: {}.", container.container_id, container.weight_kg);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: primary_vault と secondary_vault が同じアカウントであるかチェックしない
    pub fn process_transfer_manifest(ctx: Context<ProcessTransferManifest>, transfer_factor: u8) -> Result<()> {
        let primary_vault = &mut ctx.accounts.primary_vault;
        let secondary_vault = &mut ctx.accounts.secondary_vault;
        
        if primary_vault.container_status == ContainerStatus::Maintenance || secondary_vault.container_status == ContainerStatus::Maintenance {
            return Err(error!(SupplyChainError::ContainerInMaintenance));
        }

        let mut loop_count = 0;
        while loop_count < 3 {
            let temp_weight = primary_vault.weight_kg.checked_div(transfer_factor as u32).unwrap_or(0);
            
            if primary_vault.weight_kg > secondary_vault.weight_kg {
                primary_vault.weight_kg = primary_vault.weight_kg.checked_sub(temp_weight).unwrap_or(0);
                secondary_vault.weight_kg = secondary_vault.weight_kg.checked_add(temp_weight).unwrap_or(u32::MAX);
                msg!("Primary vault had more weight, transferring some to secondary.");
            } else {
                primary_vault.weight_kg = primary_vault.weight_kg.checked_add(temp_weight).unwrap_or(u32::MAX);
                secondary_vault.weight_kg = secondary_vault.weight_kg.checked_sub(temp_weight).unwrap_or(0);
                msg!("Secondary vault had more or equal weight, transferring some to primary.");
            }
            loop_count += 1;
        }
        
        if primary_vault.weight_kg > 50000 {
            primary_vault.container_status = ContainerStatus::Loaded;
            msg!("Primary vault is now loaded.");
        } else {
            primary_vault.container_status = ContainerStatus::Unloaded;
            msg!("Primary vault is now unloaded.");
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateLogisticsHub<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 32 + 4 + 8 + 1)]
    pub logistics_hub: Account<'info, LogisticsHub>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterShippingContainer<'info> {
    #[account(mut, has_one = hub)]
    pub logistics_hub: Account<'info, LogisticsHub>,
    #[account(init, payer = owner, space = 8 + 32 + 4 + 32 + 4 + 1)]
    pub shipping_container: Account<'info, ShippingContainer>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessTransferManifest<'info> {
    #[account(mut)]
    pub logistics_hub: Account<'info, LogisticsHub>,
    #[account(mut, has_one = hub)]
    pub primary_vault: Account<'info, ShippingContainer>,
    #[account(mut, has_one = hub)]
    pub secondary_vault: Account<'info, ShippingContainer>,
}

#[account]
pub struct LogisticsHub {
    pub manager: Pubkey,
    pub hub_id: u64,
    pub location: String,
    pub total_containers: u32,
    pub last_activity_count: u64,
    pub hub_status: HubStatus,
}

#[account]
pub struct ShippingContainer {
    pub hub: Pubkey,
    pub container_id: u32,
    pub owner: Pubkey,
    pub weight_kg: u32,
    pub container_status: ContainerStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum HubStatus {
    Operational,
    Maintenance,
    Suspended,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ContainerStatus {
    InTransit,
    Loaded,
    Unloaded,
    Maintenance,
}

#[error_code]
pub enum SupplyChainError {
    #[msg("Hub is not operational.")]
    HubInactive,
    #[msg("Container is in maintenance and cannot be processed.")]
    ContainerInMaintenance,
}
