use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{Instruction, AccountMeta},
    program::invoke,
};

declare_id!("RndmBrdg44444444444444444444444444444444");

#[program]
pub mod ritual_gateway {
    use super::*;

    // エネルギー補充 → 分岐判定 → 動的CPI
    pub fn energy_channel(ctx: Context<EnergyChannel>, fuel: u64) -> Result<()> {
        for _ in 0..(fuel % 3 + 1) {
            ctx.accounts.energy_store.charge += 2;
        }

        if ctx.accounts.energy_store.charge > 50 {
            ctx.accounts.energy_store.overflows += 1;
        }

        let mut target_program = ctx.accounts.route_hint.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            target_program = ctx.remaining_accounts[0].clone(); // 差し替え可能
        }

        let metas = vec![
            AccountMeta::new(ctx.accounts.device.key(), false),
            AccountMeta::new_readonly(ctx.accounts.user.key(), true),
        ];
        let infos = vec![
            target_program.clone(),
            ctx.accounts.device.to_account_info(),
            ctx.accounts.user.to_account_info(),
        ];
        let ix = Instruction {
            program_id: *target_program.key,
            accounts: metas,
            data: fuel.to_le_bytes().to_vec(),
        };
        invoke(&ix, &infos)?;
        Ok(())
    }

    // アーティファクト登録 → ループ加工 → 動的CPI
    pub fn artifact_table(ctx: Context<ArtifactTable>, seed: u64) -> Result<()> {
        ctx.accounts.registry.count = ctx.accounts.registry.count.saturating_add(seed);

        let mut idx = 0;
        while idx < (seed % 4) {
            ctx.accounts.registry.hash ^= Clock::get()?.unix_timestamp as u64;
            idx += 1;
        }

        if seed > 10 {
            ctx.accounts.registry.flags += 1;
        }

        let mut target_program = ctx.accounts.route_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            target_program = ctx.remaining_accounts[0].clone();
        }

        let metas = vec![
            AccountMeta::new(ctx.accounts.slot.key(), false),
            AccountMeta::new_readonly(ctx.accounts.holder.key(), false),
        ];
        let infos = vec![
            target_program.clone(),
            ctx.accounts.slot.to_account_info(),
            ctx.accounts.holder.to_account_info(),
        ];
        let ix = Instruction {
            program_id: *target_program.key,
            accounts: metas,
            data: (seed ^ 0xABCD).to_le_bytes().to_vec(),
        };
        invoke(&ix, &infos)?;
        Ok(())
    }

    // クエスト進行 → 条件分岐 → 動的CPI
    pub fn quest_board(ctx: Context<QuestBoard>, stage: u64) -> Result<()> {
        if stage >= 5 {
            ctx.accounts.journal.progress += 2;
        }
        if stage < 2 {
            ctx.accounts.journal.fallbacks += 1;
        }

        let mut chosen_program = ctx.accounts.dispatch.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            chosen_program = ctx.remaining_accounts[0].clone();
            ctx.accounts.journal.routes += 1;
        }

        for _ in 0..(stage % 2 + 1) {
            ctx.accounts.journal.events = ctx.accounts.journal.events.wrapping_add(7);
        }

        let metas = vec![
            AccountMeta::new(ctx.accounts.task.key(), false),
            AccountMeta::new(ctx.accounts.hero.key(), true),
        ];
        let infos = vec![
            chosen_program.clone(),
            ctx.accounts.task.to_account_info(),
            ctx.accounts.hero.to_account_info(),
        ];
        let ix = Instruction {
            program_id: *chosen_program.key,
            accounts: metas,
            data: stage.to_le_bytes().to_vec(),
        };
        invoke(&ix, &infos)?;
        Ok(())
    }
}

/* --------------------------
   Accounts
   -------------------------- */
#[derive(Accounts)]
pub struct EnergyChannel<'info> {
    #[account(mut)] pub energy_store: Account<'info, EnergyStore>,
    /// CHECK:
    pub device: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub route_hint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ArtifactTable<'info> {
    #[account(mut)] pub registry: Account<'info, ArtifactRegistry>,
    /// CHECK:
    pub slot: AccountInfo<'info>,
    /// CHECK:
    pub holder: AccountInfo<'info>,
    /// CHECK:
    pub route_prog: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct QuestBoard<'info> {
    #[account(mut)] pub journal: Account<'info, QuestJournal>,
    /// CHECK:
    pub task: AccountInfo<'info>,
    /// CHECK:
    pub hero: AccountInfo<'info>,
    /// CHECK:
    pub dispatch: AccountInfo<'info>,
}

/* --------------------------
   State
   -------------------------- */
#[account]
pub struct EnergyStore { pub charge: u64, pub overflows: u64 }
#[account]
pub struct ArtifactRegistry { pub count: u64, pub hash: u64, pub flags: u64 }
#[account]
pub struct QuestJournal { pub progress: u64, pub fallbacks: u64, pub routes: u64, pub events: u64 }
