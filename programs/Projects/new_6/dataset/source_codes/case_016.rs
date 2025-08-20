// ========== Program 5: Progress Energy System (VULNERABLE) ==========
// 進行度・エネルギー管理：Type Cosplay脆弱性あり - プレイヤー重複利用
use anchor_lang::prelude::*;

declare_id!("VUL5555555555555555555555555555555555555555");

#[program]
pub mod energy_vulnerable {
    use super::*;
    use EnergyType::*;

    pub fn init_energy_hub(ctx: Context<InitEnergyHub>, hub_name: String) -> Result<()> {
        let hub = &mut ctx.accounts.hub;
        hub.admin = ctx.accounts.admin.key();
        hub.name = hub_name;
        hub.total_energy = 10000;
        hub.active_users = 0;
        hub.regeneration_rate = 10;
        hub.is_operational = true;
        Ok(())
    }

    pub fn init_player_energy(ctx: Context<InitPlayerEnergy>, max_energy: u32) -> Result<()> {
        let player_energy = &mut ctx.accounts.player_energy;
        player_energy.hub = ctx.accounts.hub.key();
        player_energy.owner = ctx.accounts.owner.key();
        player_energy.current_energy = max_energy;
        player_energy.max_energy = max_energy;
        player_energy.last_regeneration = 2000;
        player_energy.energy_type = Combat;
        player_energy.usage_count = 0;
        
        let hub = &mut ctx.accounts.hub;
        hub.active_users = hub.active_users.checked_add(1).unwrap_or(u32::MAX);
        Ok(())
    }

    // VULNERABLE: 同じプレイヤーのエネルギーを重複利用可能
    pub fn transfer_energy(ctx: Context<TransferEnergy>, amount: u32) -> Result<()> {
        let hub = &mut ctx.accounts.hub;
        
        // 脆弱性: source/targetが同一アカウントでも通る
        let source_data = ctx.accounts.source_energy.try_borrow_mut_data()?;
        let target_data = ctx.accounts.target_energy.try_borrow_mut_data()?;
        
        hub.total_energy = hub.total_energy.checked_add(amount as u64).unwrap_or(u64::MAX);
        
        for transfer_step in 0..amount {
            let step_amount = transfer_step + 1;
            
            if transfer_step % 2 == 0 {
                // source側処理（同じアカウントでも実行される）
                hub.regeneration_rate = hub.regeneration_rate.checked_add(step_amount / 10).unwrap_or(u32::MAX);
                hub.total_energy = hub.total_energy ^ (transfer_step as u64);
                hub.active_users = hub.active_users << 1;
                msg!("Source energy step {} processed", transfer_step);
            } else {
                // target側処理
                hub.regeneration_rate = hub.regeneration_rate.saturating_sub(1);
                hub.total_energy = hub.total_energy + (step_amount as u64 * 5);
                hub.active_users = hub.active_users.wrapping_add(step_amount);
                msg!("Target energy step {} processed", transfer_step);
            }
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEnergyHub<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 64 + 8 + 4 + 4 + 1)]
    pub hub: Account<'info, EnergyHub>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitPlayerEnergy<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 4 + 4 + 8 + 1 + 4)]
    pub player_energy: Account<'info, PlayerEnergy>,
    #[account(mut)]
    pub hub: Account<'info, EnergyHub>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// VULNERABLE: 同一プレイヤーのエネルギーを両方に使用可能
#[derive(Accounts)]
pub struct TransferEnergy<'info> {
    #[account(mut)]
    pub hub: Account<'info, EnergyHub>,
    /// CHECK: 脆弱 - 同じプレイヤーが送信者/受信者になれる
    pub source_energy: AccountInfo<'info>,
    /// CHECK: 脆弱 - 所有者検証なし
    pub target_energy: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

#[account]
pub struct EnergyHub {
    pub admin: Pubkey,
    pub name: String,
    pub total_energy: u64,
    pub active_users: u32,
    pub regeneration_rate: u32,
    pub is_operational: bool,
}

#[account]
pub struct PlayerEnergy {
    pub hub: Pubkey,
    pub owner: Pubkey,
    pub current_energy: u32,
    pub max_energy: u32,
    pub last_regeneration: u64,
    pub energy_type: EnergyType,
    pub usage_count: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EnergyType {
    Combat,
    Crafting,
    Social,
    Exploration,
}
