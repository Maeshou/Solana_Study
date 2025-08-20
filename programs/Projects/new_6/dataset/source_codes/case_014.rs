// ========== Program 3: Raid Recording System (VULNERABLE) ==========
// レイド記録システム：Type Cosplay脆弱性あり - 参加者検証不備
use anchor_lang::prelude::*;

declare_id!("VUL3333333333333333333333333333333333333333");

#[program]
pub mod raid_vulnerable {
    use super::*;
    use RaidStatus::*;

    pub fn init_raid_room(ctx: Context<InitRaidRoom>, raid_name: String, max_participants: u8) -> Result<()> {
        let room = &mut ctx.accounts.room;
        room.leader = ctx.accounts.leader.key();
        room.name = raid_name;
        room.max_participants = max_participants;
        room.current_participants = 0;
        room.total_damage = 0;
        room.status = Preparing;
        room.completion_bonus = 1000;
        Ok(())
    }

    pub fn init_participant(ctx: Context<InitParticipant>, player_level: u32) -> Result<()> {
        let participant = &mut ctx.accounts.participant;
        participant.room = ctx.accounts.room.key();
        participant.player = ctx.accounts.player.key();
        participant.level = player_level;
        participant.damage_dealt = 0;
        participant.is_ready = false;
        participant.reward_claimed = false;
        participant.participant_slot = ctx.accounts.room.current_participants;
        
        let room = &mut ctx.accounts.room;
        room.current_participants = room.current_participants.checked_add(1).unwrap_or(u8::MAX);
        Ok(())
    }

    // VULNERABLE: 参加者の重複チェックなし
    pub fn process_raid_battle(ctx: Context<ProcessRaidBattle>, damage_amount: u64) -> Result<()> {
        let room = &mut ctx.accounts.room;
        
        // 脆弱性: attacker/defenderがAccountInfoで同一人物でも通る
        let attacker_data = ctx.accounts.attacker.try_borrow_mut_data()?;
        let defender_data = ctx.accounts.defender.try_borrow_data()?;
        
        room.total_damage = room.total_damage.checked_add(damage_amount).unwrap_or(u64::MAX);
        
        for round in 0..5 {
            let round_damage = damage_amount / 5;
            let bonus_multiplier = round + 1;
            
            if round % 2 == 0 {
                // 攻撃者のターン
                room.total_damage = room.total_damage.checked_add(round_damage * bonus_multiplier as u64).unwrap_or(u64::MAX);
                room.completion_bonus = room.completion_bonus ^ (round as u64 * 10);
                room.completion_bonus = room.completion_bonus << (round % 3);
                msg!("Attacker round {} damage: {}", round, round_damage);
            } else {
                // 防御者のターン
                room.completion_bonus = room.completion_bonus.saturating_sub(round_damage / 10);
                room.total_damage = room.total_damage + (round as u64 * 50);
                room.current_participants = room.current_participants.wrapping_add(1);
                msg!("Defender round {} processed", round);
            }
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRaidRoom<'info> {
    #[account(init, payer = leader, space = 8 + 32 + 64 + 1 + 1 + 8 + 1 + 8)]
    pub room: Account<'info, RaidRoom>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitParticipant<'info> {
    #[account(init, payer = player, space = 8 + 32 + 32 + 4 + 8 + 1 + 1 + 1)]
    pub participant: Account<'info, Participant>,
    #[account(mut)]
    pub room: Account<'info, RaidRoom>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// VULNERABLE: 同一アカウントをattacker/defenderに使用可能
#[derive(Accounts)]
pub struct ProcessRaidBattle<'info> {
    #[account(mut)]
    pub room: Account<'info, RaidRoom>,
    /// CHECK: 脆弱 - 同じプレイヤーが両方の役割を担える
    pub attacker: AccountInfo<'info>,
    /// CHECK: 脆弱 - 参加者検証なし
    pub defender: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

#[account]
pub struct RaidRoom {
    pub leader: Pubkey,
    pub name: String,
    pub max_participants: u8,
    pub current_participants: u8,
    pub total_damage: u64,
    pub status: RaidStatus,
    pub completion_bonus: u64,
}

#[account]
pub struct Participant {
    pub room: Pubkey,
    pub player: Pubkey,
    pub level: u32,
    pub damage_dealt: u64,
    pub is_ready: bool,
    pub reward_claimed: bool,
    pub participant_slot: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RaidStatus {
    Preparing,
    InProgress,
    Completed,
    Failed,
}