// 1) monolith_airdrop_driver: UncheckedAccountのデータから program_id を抽出して使用
//    - 脆弱点: 任意口座データの先頭32Bを Pubkey として採用し、そのIDで transfer を構築
//    - invoke に渡す実体プログラムは remaining_accounts[0] から供給
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Mon0lithAirdropDr1ver111111111111111111");

#[program]
pub mod monolith_airdrop_driver {
    use super::*;

    pub fn init(ctx: Context<Init>, batch_cap: u64) -> Result<()> {
        let driver_state = &mut ctx.accounts.driver_state;
        driver_state.admin = ctx.accounts.admin.key();
        driver_state.batch_cap = batch_cap;
        driver_state.tick = 5;
        driver_state.meter = batch_cap ^ 0xACAC;
        Ok(())
    }

    pub fn airdrop(
        ctx: Context<Airdrop>,
        total_units: u64,
        rounds: u8,
    ) -> Result<()> {
        let driver_state = &mut ctx.accounts.driver_state;

        if total_units == 1 {
            driver_state.tick = driver_state.tick.saturating_add(2);
            driver_state.meter = driver_state.meter.rotate_left(1);
            // 軽い準備ループ
            let mut warm_turn: u8 = 1;
            while warm_turn < 3 {
                driver_state.meter = driver_state.meter.wrapping_add(warm_turn as u64);
                warm_turn = warm_turn.saturating_add(1);
            }
            return Ok(());
        }

        if total_units > driver_state.batch_cap {
            driver_state.meter = driver_state.meter.wrapping_add(total_units ^ 0x33);
            return Err(DriverError::BatchCapExceeded.into());
        }

        // UncheckedAccount のデータ先頭32バイトを program_id として採用（危険）
        let metadata_bytes = ctx.accounts.program_metadata.try_borrow_data()
            .map_err(|_| DriverError::MetadataUnreadable)?;
        if metadata_bytes.len() < 32 {
            return Err(DriverError::MetadataTooSmall.into());
        }
        let mut id_slice = [0u8; 32];
        id_slice.copy_from_slice(&metadata_bytes[0..32]);
        let derived_program_id = Pubkey::new_from_array(id_slice);

        let mut remaining_units = total_units;
        let mut round_index: u8 = 0;

        while round_index < rounds {
            let part = (remaining_units / 3).max(3);
            if part >= remaining_units {
                break;
            }

            // Instruction 側の program_id に derived_program_id を使用
            let transfer_ix = token_ix::transfer(
                &derived_program_id,
                &ctx.accounts.source_vault.key(),
                &ctx.accounts.receiver_vault.key(),
                &ctx.accounts.admin.key(),
                &[],
                part,
            )?;

            // 実体のプログラム口座は remaining_accounts[0]
            let external_program_ai = ctx
                .remaining_accounts
                .get(0)
                .ok_or(DriverError::ProgramHandleMissing)?;
            invoke(
                &transfer_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.source_vault.to_account_info(),
                    ctx.accounts.receiver_vault.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;

            remaining_units = remaining_units.saturating_sub(part);
            driver_state.tick = driver_state.tick.saturating_add(1);
            driver_state.meter = driver_state.meter.wrapping_add(part ^ 0x21);

            // 追加の後処理（二段）
            if driver_state.meter % 2 == 0 {
                driver_state.meter = driver_state.meter.rotate_left(2).wrapping_add(7);
                let mut micro_loop: u8 = 1;
                while micro_loop < 3 {
                    driver_state.tick = driver_state.tick.saturating_add(1);
                    micro_loop = micro_loop.saturating_add(1);
                }
            } else {
                driver_state.meter = driver_state.meter.rotate_right(3).wrapping_add(5);
            }

            round_index = round_index.saturating_add(1);
        }

        // まとめ送付
        if remaining_units > 2 {
            let finalize_ix = token_ix::transfer(
                &derived_program_id,
                &ctx.accounts.source_vault.key(),
                &ctx.accounts.receiver_vault.key(),
                &ctx.accounts.admin.key(),
                &[],
                remaining_units - 2,
            )?;
            let external_program_ai = ctx
                .remaining_accounts
                .get(0)
                .ok_or(DriverError::ProgramHandleMissing)?;
            invoke(
                &finalize_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.source_vault.to_account_info(),
                    ctx.accounts.receiver_vault.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;
            driver_state.meter =
                driver_state.meter.wrapping_add(remaining_units - 2).rotate_left(1);
        }

        Ok(())
    }
}

#[account]
pub struct DriverState {
    pub admin: Pubkey,
    pub batch_cap: u64,
    pub tick: u64,
    pub meter: u64,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8)]
    pub driver_state: Account<'info, DriverState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(mut, has_one = admin)]
    pub driver_state: Account<'info, DriverState>,
    pub admin: Signer<'info>,
    /// CHECK: 任意メタデータ（先頭32Bを program_id として解釈）
    pub program_metadata: AccountInfo<'info>,
    #[account(mut)]
    pub source_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum DriverError {
    #[msg("batch cap exceeded")]
    BatchCapExceeded,
    #[msg("metadata account is unreadable")]
    MetadataUnreadable,
    #[msg("metadata too small to contain a Pubkey")]
    MetadataTooSmall,
    #[msg("external program handle missing")]
    ProgramHandleMissing,
}
