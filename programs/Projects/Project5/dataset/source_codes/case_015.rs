// ============================================================================
// 9) RaidPlanner（レイド計画）— PDA使用 + seedsでpool/roster/lootを固定 + constraint
// ============================================================================
declare_id!("RAID99999999999999999999999999999999");

#[program]
pub mod raid_planner {
    use super::*;

    pub fn init_raid(ctx: Context<InitRaid>, limit: u32) -> Result<()> {
        ctx.accounts.raid.leader = ctx.accounts.captain.key();
        ctx.accounts.raid.limit = limit;
        ctx.accounts.raid.active = true;

        ctx.accounts.roster.count = 0;
        ctx.accounts.roster.ready = false;

        ctx.accounts.loot.total = 0;
        ctx.accounts.loot.boxes = 0;
        Ok(())
    }

    pub fn enlist(ctx: Context<Enlist>, add: u32) -> Result<()> {
        require!(ctx.accounts.raid.key() != ctx.accounts.roster.key(), RPerr::Dup);
        require!(ctx.accounts.raid.key() != ctx.accounts.loot.key(), RPerr::Dup);
        require!(ctx.accounts.roster.key() != ctx.accounts.loot.key(), RPerr::Dup);

        let mut i = 0;
        while i < add {
            ctx.accounts.roster.count = ctx.accounts.roster.count.saturating_add(1);
            ctx.accounts.loot.total = ctx.accounts.loot.total.saturating_add(1);
            ctx.accounts.loot.boxes = ctx.accounts.loot.boxes.saturating_add(0);
            i += 1;
        }

        if ctx.accounts.roster.count > ctx.accounts.raid.limit {
            ctx.accounts.raid.active = false;
            ctx.accounts.roster.ready = true;
            ctx.accounts.loot.boxes = ctx.accounts.loot.boxes.saturating_add(3);
            msg!("too many; locked");
        } else {
            ctx.accounts.raid.active = true;
            ctx.accounts.roster.ready = false;
            ctx.accounts.loot.total = ctx.accounts.loot.total.saturating_add(1);
            msg!("enlisted");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRaid<'info> {
    #[account(init, payer=payer, space=8+32+4+1, seeds=[b"raid", payer.key().as_ref()], bump)]
    pub raid: Account<'info, Raid>,
    #[account(init, payer=payer, space=8+4+1, seeds=[b"roster", payer.key().as_ref()], bump)]
    pub roster: Account<'info, Roster>,
    #[account(init, payer=payer, space=8+8+4, seeds=[b"loot", payer.key().as_ref()], bump)]
    pub loot: Account<'info, Loot>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub captain: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Enlist<'info> {
    #[account(mut, seeds=[b"raid", payer.key().as_ref()], bump)]
    pub raid: Account<'info, Raid>,
    #[account(mut, seeds=[b"roster", payer.key().as_ref()], bump)]
    pub roster: Account<'info, Roster>,
    #[account(mut, seeds=[b"loot", payer.key().as_ref()], bump)]
    pub loot: Account<'info, Loot>,
    /// CHECK: seeds固定
    pub payer: UncheckedAccount<'info>,
}

#[account] pub struct Raid { pub leader: Pubkey, pub limit: u32, pub active: bool }
#[account] pub struct Roster { pub count: u32, pub ready: bool }
#[account] pub struct Loot { pub total: u64, pub boxes: u32 }

#[error_code] pub enum RPerr { #[msg("dup")] Dup }