use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("DiffContent99999999999999999999999999999");

#[program]
pub mod transit_hub {
    use super::*;

    // 1) sensor_map: ループ → 分岐 → 動的CPI（inline Instruction）
    pub fn sensor_map(ctx: Context<SensorMap>, pulse: u64) -> Result<()> {
        for _ in 0..(pulse % 3 + 1) {
            ctx.accounts.grid.heat = ctx.accounts.grid.heat.wrapping_add(5);
        }
        if ctx.accounts.grid.heat > 30 {
            ctx.accounts.grid.flags ^= 1;
        }

        let mut target = ctx.accounts.router.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            target = ctx.remaining_accounts[0].clone(); // ← 差し替え可能
            ctx.accounts.grid.hops = ctx.accounts.grid.hops.wrapping_add(1);
        }

        invoke(
            &Instruction {
                program_id: *target.key,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.panel.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.operator.key(), true),
                ],
                data: pulse.to_le_bytes().to_vec(),
            },
            &[
                target.clone(),
                ctx.accounts.panel.to_account_info(),
                ctx.accounts.operator.to_account_info(),
            ],
        )?;
        Ok(())
    }

    // 2) vault_rotate: 分岐 → ループ → 動的CPI（メタ順も変える）
    pub fn vault_rotate(ctx: Context<VaultRotate>, salt: u64) -> Result<()> {
        if salt & 1 == 1 {
            ctx.accounts.vmeta.odd += 1;
        }
        let mut i = 0;
        while i < (salt % 4) {
            ctx.accounts.vmeta.hash ^= Clock::get()?.slot;
            i += 1;
        }

        let mut target = ctx.accounts.switch.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            target = ctx.remaining_accounts[0].clone();
            ctx.accounts.vmeta.routes += 1;
        }

        invoke(
            &Instruction {
                program_id: *target.key,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.chest.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.guard.key(), false),
                ],
                data: (salt ^ 0xBEEF).to_le_bytes().to_vec(),
            },
            &[
                target.clone(),
                ctx.accounts.chest.to_account_info(),
                ctx.accounts.guard.to_account_info(),
            ],
        )?;
        Ok(())
    }

    // 3) guild_mail: 内部計算 → 分岐 → 動的CPI（別アカウント並び）
    pub fn guild_mail(ctx: Context<GuildMail>, seq: u64) -> Result<()> {
        ctx.accounts.ledger.count = ctx.accounts.ledger.count.wrapping_add(seq);
        if seq >= 5 {
            ctx.accounts.ledger.bump = ctx.accounts.ledger.bump.wrapping_add(2);
        }

        let mut target = ctx.accounts.mpost.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            target = ctx.remaining_accounts[0].clone();
            ctx.accounts.ledger.tracks ^= 1;
        }

        invoke(
            &Instruction {
                program_id: *target.key,
                accounts: vec![
                    AccountMeta::new_readonly(ctx.accounts.member.key(), false),
                    AccountMeta::new(ctx.accounts.mailbox.key(), false),
                ],
                data: seq.to_le_bytes().to_vec(),
            },
            &[
                target.clone(),
                ctx.accounts.member.to_account_info(),
                ctx.accounts.mailbox.to_account_info(),
            ],
        )?;
        Ok(())
    }

    // 4) beacon_ping: 分岐（入れ子 if）→ 動的CPI → ループ
    pub fn beacon_ping(ctx: Context<BeaconPing>, meter: u64) -> Result<()> {
        if meter > 0 {
            if meter % 2 == 0 {
                ctx.accounts.beacon.evens = ctx.accounts.beacon.evens.wrapping_add(1);
            }
        }

        let mut target = ctx.accounts.forward.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            target = ctx.remaining_accounts[0].clone();
            ctx.accounts.beacon.paths = ctx.accounts.beacon.paths.wrapping_add(1);
        }

        invoke(
            &Instruction {
                program_id: *target.key,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.pylon.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.sender.key(), true),
                ],
                data: meter.rotate_left(2).to_le_bytes().to_vec(),
            },
            &[
                target.clone(),
                ctx.accounts.pylon.to_account_info(),
                ctx.accounts.sender.to_account_info(),
            ],
        )?;

        for _ in 0..(meter % 3) {
            ctx.accounts.beacon.noise ^= Clock::get()?.slot;
        }
        Ok(())
    }

    // 5) barter_stamp: ループ → 動的CPI → 分岐（末尾）
    pub fn barter_stamp(ctx: Context<BarterStamp>, lot: u64) -> Result<()> {
        for _ in 0..(lot % 2 + 1) {
            ctx.accounts.bstat.ticks = ctx.accounts.bstat.ticks.wrapping_add(1);
        }

        let mut target = ctx.accounts.broker.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            target = ctx.remaining_accounts[0].clone();
            ctx.accounts.bstat.paths = ctx.accounts.bstat.paths.wrapping_add(1);
        }

        invoke(
            &Instruction {
                program_id: *target.key,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.stall.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.trader.key(), false),
                ],
                data: (lot ^ 7).to_le_bytes().to_vec(),
            },
            &[
                target.clone(),
                ctx.accounts.stall.to_account_info(),
                ctx.accounts.trader.to_account_info(),
            ],
        )?;

        if ctx.accounts.bstat.paths > 2 {
            ctx.accounts.bstat.flags ^= 1;
        }
        Ok(())
    }

    // 6) shrine_attest: 内部計算 → ループ → 動的CPI（メタの new/readonly 組合せ変更）
    pub fn shrine_attest(ctx: Context<ShrineAttest>, code: u64) -> Result<()> {
        ctx.accounts.slog.mix ^= code.rotate_right(3);

        let mut j = 0;
        while j < (code % 3 + 1) {
            ctx.accounts.slog.beads = ctx.accounts.slog.beads.wrapping_add(2);
            j += 1;
        }

        let mut target = ctx.accounts.oracle.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            target = ctx.remaining_accounts[0].clone();
            ctx.accounts.slog.routes = ctx.accounts.slog.routes.wrapping_add(1);
        }

        invoke(
            &Instruction {
                program_id: *target.key,
                accounts: vec![
                    AccountMeta::new_readonly(ctx.accounts.seal.key(), false),
                    AccountMeta::new(ctx.accounts.altar.key(), false),
                ],
                data: code.to_le_bytes().to_vec(),
            },
            &[
                target.clone(),
                ctx.accounts.seal.to_account_info(),
                ctx.accounts.altar.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

/* --------------------------
   Accounts
   -------------------------- */
#[derive(Accounts)]
pub struct SensorMap<'info> {
    #[account(mut)] pub grid: Account<'info, GridState>,
    /// CHECK:
    pub panel: AccountInfo<'info>,
    /// CHECK:
    pub operator: AccountInfo<'info>,
    /// CHECK:
    pub router: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct VaultRotate<'info> {
    #[account(mut)] pub vmeta: Account<'info, VaultMeta>,
    /// CHECK:
    pub chest: AccountInfo<'info>,
    /// CHECK:
    pub guard: AccountInfo<'info>,
    /// CHECK:
    pub switch: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GuildMail<'info> {
    #[account(mut)] pub ledger: Account<'info, MailLedger>,
    /// CHECK:
    pub member: AccountInfo<'info>,
    /// CHECK:
    pub mailbox: AccountInfo<'info>,
    /// CHECK:
    pub mpost: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BeaconPing<'info> {
    #[account(mut)] pub beacon: Account<'info, BeaconState>,
    /// CHECK:
    pub pylon: AccountInfo<'info>,
    /// CHECK:
    pub sender: AccountInfo<'info>,
    /// CHECK:
    pub forward: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BarterStamp<'info> {
    #[account(mut)] pub bstat: Account<'info, BarterState>,
    /// CHECK:
    pub stall: AccountInfo<'info>,
    /// CHECK:
    pub trader: AccountInfo<'info>,
    /// CHECK:
    pub broker: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ShrineAttest<'info> {
    #[account(mut)] pub slog: Account<'info, ShrineLog>,
    /// CHECK:
    pub seal: AccountInfo<'info>,
    /// CHECK:
    pub altar: AccountInfo<'info>,
    /// CHECK:
    pub oracle: AccountInfo<'info>,
}

/* --------------------------
   State
   -------------------------- */
#[account] pub struct GridState   { pub heat: u64, pub flags: u64, pub hops: u64 }
#[account] pub struct VaultMeta   { pub routes: u64, pub odd: u64, pub hash: u64 }
#[account] pub struct MailLedger  { pub count: u64, pub bump: u64, pub tracks: u64 }
#[account] pub struct BeaconState { pub evens: u64, pub paths: u64, pub noise: u64 }
#[account] pub struct BarterState { pub ticks: u64, pub paths: u64, pub flags: u64 }
#[account] pub struct ShrineLog   { pub mix: u64, pub beads: u64, pub routes: u64 }
