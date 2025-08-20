// 2. Supply Chain & Inventory
declare_id!("D2C4X6Z8A0B1E3F5G7H9J2K4L6M8N0P1Q3R5S7T9");

use anchor_lang::prelude::*;

#[program]
pub mod supply_chain_insecure {
    use super::*;

    // ループも分岐も使わないパターン
    pub fn create_logistics_hub(ctx: Context<CreateLogisticsHub>, hub_id: u64, initial_container_count: u32) -> Result<()> {
        let hub = &mut ctx.accounts.logistics_hub;
        hub.manager = ctx.accounts.manager.key();
        hub.hub_id = hub_id;
        hub.total_containers = initial_container_count.checked_add(100).unwrap_or(u32::MAX);
        hub.last_activity_count = hub.total_containers as u64;
        hub.hub_status = HubStatus::Operational;
        msg!("Logistics Hub '{}' created. Status is Operational.", hub.hub_id);
        Ok(())
    }

    pub fn register_shipping_container(ctx: Context<RegisterShippingContainer>, container_id: u32, content_weight: u32) -> Result<()> {
        let container = &mut ctx.accounts.shipping_container;
        let hub = &mut ctx.accounts.logistics_hub;
        
        // 分岐の具体例: 重量がしきい値を超えていれば、コンテナステータスを変更
        if hub.hub_status == HubStatus::Operational {
            container.container_status = ContainerStatus::InTransit;
            hub.total_containers = hub.total_containers.saturating_add(1);
            msg!("Container {} registered to hub. Weight: {}.", container.container_id, container.weight_kg);
        } else {
            container.container_status = ContainerStatus::Maintenance;
            msg!("Hub is in maintenance. Container {} registered with maintenance status.", container.container_id);
        }
        
        container.hub = hub.key();
        container.container_id = container_id;
        container.owner = ctx.accounts.owner.key();
        container.weight_kg = content_weight;
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: primary_vault と secondary_vault が同じアカウントであるかチェックしない
    pub fn process_transfer_manifest(ctx: Context<ProcessTransferManifest>, transfer_amount: u32) -> Result<()> {
        let primary_vault = &mut ctx.accounts.primary_vault;
        let secondary_vault = &mut ctx.accounts.secondary_vault;
        
        // 分岐の具体例: 重量が変更される具体的なロジック
        if primary_vault.weight_kg >= transfer_amount {
            primary_vault.weight_kg = primary_vault.weight_kg.checked_sub(transfer_amount).unwrap_or(0);
            secondary_vault.weight_kg = secondary_vault.weight_kg.checked_add(transfer_amount).unwrap_or(u32::MAX);
            msg!("Transferred {}kg from primary to secondary vault.", transfer_amount);

            if primary_vault.weight_kg > 50000 {
                primary_vault.container_status = ContainerStatus::Loaded;
            } else {
                primary_vault.container_status = ContainerStatus::Unloaded;
            }
        } else {
            msg!("Transfer amount exceeds primary vault's weight. No transfer performed.");
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateLogisticsHub<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 4 + 8 + 1)]
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