use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_game {
    use super::*;

    // レイドボスに挑戦する
    pub fn start_raid_battle(ctx: Context<StartRaidBattle>) -> Result<()> {
        let raid_boss = &ctx.accounts.raid_boss_info;
        let raid_ticket = &mut ctx.accounts.raid_ticket;
        let player = &ctx.accounts.player;
        let raid_instance = &mut ctx.accounts.raid_instance;

        // チケットが有効か確認
        require!(raid_ticket.is_used == false, GameError::RaidTicketAlreadyUsed);
        require!(raid_ticket.boss_id == raid_boss.boss_id, GameError::TicketNotForThisBoss);
        
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        // レイド開催期間内か確認
        require!(current_timestamp >= raid_boss.start_timestamp, GameError::RaidNotStarted);
        require!(current_timestamp <= raid_boss.end_timestamp, GameError::RaidHasEnded);
        
        // チケットを使用済みにする
        raid_ticket.is_used = true;

        // レイドインスタンスを初期化
        raid_instance.boss_id = raid_boss.boss_id;
        raid_instance.challenger = player.key();
        raid_instance.remaining_hp = raid_boss.total_hp;
        raid_instance.battle_start_timestamp = current_timestamp;
        raid_instance.is_active = true;
        
        // プレイヤーの貢献度リストを初期化
        // 挑戦者自身を最初の貢献者として追加
        let mut initial_contributors = Vec::new();
        for _ in 0..5 { // 固定サイズの例
            initial_contributors.push(Contribution {
                player: Pubkey::default(),
                damage: 0
            });
        }
        initial_contributors[0] = Contribution {
            player: player.key(),
            damage: 0,
        };
        raid_instance.contributors = initial_contributors;

        msg!("Raid battle against boss {} has started!", raid_boss.boss_id);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartRaidBattle<'info> {
    #[account(seeds = [b"raid_boss", raid_boss_info.boss_id.to_le_bytes().as_ref()], bump = raid_boss_info.bump)]
    pub raid_boss_info: Account<'info, RaidBossInfo>,
    #[account(mut, seeds = [b"raid_ticket", player.key().as_ref(), raid_ticket.mint.as_ref()], bump = raid_ticket.bump)]
    pub raid_ticket: Account<'info, RaidTicket>, // 挑戦権NFT
    #[account(
        init,
        payer = player,
        space = 8 + 4 + 32 + 8 + 8 + 1 + (32 + 8) * 5, // spaceの計算は要調整
        seeds = [b"raid_instance", player.key().as_ref(), raid_boss_info.key().as_ref()],
        bump
    )]
    pub raid_instance: Account<'info, RaidInstance>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RaidBossInfo {
    pub boss_id: u32,
    pub total_hp: u64,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub bump: u8,
}

#[account]
pub struct RaidTicket {
    pub mint: Pubkey,
    pub boss_id: u32,
    pub is_used: bool,
    pub bump: u8,
}

#[account]
pub struct RaidInstance {
    pub boss_id: u32,
    pub challenger: Pubkey,
    pub remaining_hp: u64,
    pub battle_start_timestamp: i64,
    pub is_active: bool,
    pub contributors: Vec<Contribution>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Contribution {
    pub player: Pubkey,
    pub damage: u64,
}

#[error_code]
pub enum GameError {
    #[msg("This raid ticket has already been used.")]
    RaidTicketAlreadyUsed,
    #[msg("This ticket is not for the current raid boss.")]
    TicketNotForThisBoss,
    #[msg("This raid has not started yet.")]
    RaidNotStarted,
    #[msg("This raid has already ended.")]
    RaidHasEnded,
}