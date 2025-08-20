// 1) registry: 登録レコード作成（分岐ほぼなしでシンプルに保持）
use anchor_lang::prelude::*;

declare_id!("Reg1sTry11111111111111111111111111111111");

#[program]
pub mod tournament_registry {
    use super::*;

    pub fn register_participant(
        ctx: Context<RegisterParticipant>,
        tier: TournamentTier,
        stake: u64,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        let entry = &mut ctx.accounts.entry;
        entry.participant = ctx.accounts.player.key();
        entry.registration_timestamp = now;
        entry.selected_tier = tier;
        entry.stake_deposited = stake;
        entry.memo_nonce = now as u64 ^ stake.rotate_left(3);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterParticipant<'info> {
    #[account(
        init,
        payer = player,
        space = 8 + RegistryEntry::LEN,
        seeds = [b"reg", player.key().as_ref()],
        bump
    )]
    pub entry: Account<'info, RegistryEntry>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RegistryEntry {
    pub participant: Pubkey,
    pub registration_timestamp: i64,
    pub selected_tier: TournamentTier,
    pub stake_deposited: u64,
    pub memo_nonce: u64,
}
impl RegistryEntry { pub const LEN: usize = 32 + 8 + 1 + 8 + 8; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum TournamentTier { Bronze, Silver, Gold, Platinum, Diamond, Master, Grandmaster }
