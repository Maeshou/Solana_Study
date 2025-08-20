// 例3) WeekdayRegistryWithSigner: 曜日で経路分岐し、PDA署名で外部CPIへ複数回記録
//  - alt側（平日奇数日想定）：概要→本文の2段記録 + カウンタ更新
//  - primary側（平日偶数日想定）：増分記録 + タグ付与 + 最終更新スロット記録
//  いずれも bad_registry が動的 program_id を使用。
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};

declare_id!("WeekdayRegAAAAABBBBCCCCDDDDEEEEFFFF0003");

#[program]
pub mod weekday_registry_with_signer {
    use super::*;
    pub fn setup(ctx: Context<SetupRegistry>, bump: u8) -> Result<()> {
        let meta = &mut ctx.accounts.registry_meta;
        meta.owner = ctx.accounts.owner.key();
        meta.bump = bump;
        meta.weekday_even_calls = 0;
        meta.weekday_odd_calls = 0;
        meta.cumulative_value = 0;
        meta.last_update_slot = 0;
        Ok(())
    }

    pub fn record(ctx: Context<RecordRegistry>, raw_value: u64) -> Result<()> {
        let meta = &mut ctx.accounts.registry_meta;
        let day_index = (Clock::get()?.unix_timestamp as u64 / 86_400 + 4) % 7; // 0=日…6=土（簡易）
        let is_odd_weekday = matches!(day_index, 1 | 3 | 5);

        let seeds: &[&[u8]] = &[b"registry", meta.owner.as_ref(), &[meta.bump]];
        let signer = &[seeds];

        if is_odd_weekday {
            // alt 経路：概要→本文の2回記録
            let summary_value = (raw_value / 8).max(1);
            let cpi_summary = CpiContext::new_with_signer(
                ctx.accounts.alt_program.to_account_info(),
                bad_registry::cpi::Record {
                    authority_pda: meta.to_account_info(),
                    subject: ctx.accounts.subject.to_account_info(),
                },
                signer,
            );
            bad_registry::cpi::record(cpi_summary, summary_value)?;

            let cpi_body = CpiContext::new_with_signer(
                ctx.accounts.alt_program.to_account_info(),
                bad_registry::cpi::Record {
                    authority_pda: meta.to_account_info(),
                    subject: ctx.accounts.subject.to_account_info(),
                },
                signer,
            );
            bad_registry::cpi::record(cpi_body, raw_value)?;

            meta.weekday_odd_calls = meta.weekday_odd_calls.saturating_add(2);
            meta.cumulative_value = meta.cumulative_value.saturating_add(raw_value);
        } else {
            // primary 経路：増分記録 + タグ付与 + スロット記録
            let incremental = (raw_value * 3) / 5;
            let cpi_inc = CpiContext::new_with_signer(
                ctx.accounts.primary_program.to_account_info(),
                bad_registry::cpi::Record {
                    authority_pda: meta.to_account_info(),
                    subject: ctx.accounts.subject.to_account_info(),
                },
                signer,
            );
            bad_registry::cpi::record(cpi_inc, incremental)?;

            let label = (day_index as u16) << 8 | ((raw_value % 255) as u16);
            let cpi_tag = CpiContext::new_with_signer(
                ctx.accounts.primary_program.to_account_info(),
                bad_registry::cpi::Tag {
                    authority_pda: meta.to_account_info(),
                    subject: ctx.accounts.subject.to_account_info(),
                },
                signer,
            );
            bad_registry::cpi::tag(cpi_tag, label)?;

            meta.weekday_even_calls = meta.weekday_even_calls.saturating_add(2);
            meta.last_update_slot = Clock::get()?.slot;
            meta.cumulative_value = meta.cumulative_value.saturating_add(incremental as u64);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupRegistry<'info> {
    #[account(init, payer = owner, seeds=[b"registry", owner.key().as_ref()], bump, space = 8 + 32 + 1 + 8 + 8 + 8)]
    pub registry_meta: Account<'info, RegistryMeta>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RecordRegistry<'info> {
    #[account(mut, has_one = owner)]
    pub registry_meta: Account<'info, RegistryMeta>,
    pub owner: Signer<'info>,
    /// CHECK:
    pub subject: UncheckedAccount<'info>,
    /// CHECK: 未検証
    pub primary_program: UncheckedAccount<'info>,
    /// CHECK: 未検証
    pub alt_program: UncheckedAccount<'info>,
}

#[account]
pub struct RegistryMeta {
    pub owner: Pubkey,
    pub bump: u8,
    pub weekday_even_calls: u64,
    pub weekday_odd_calls: u64,
    pub cumulative_value: u64,
    pub last_update_slot: u64,
}

// --- 動的IDを使う外部CPIラッパ（Arbitrary CPI の根） ---
pub mod bad_registry {
    use super::*;
    pub mod cpi {
        use super::*;
        #[derive(Clone)]
        pub struct Record<'info> { pub authority_pda: AccountInfo<'info>, pub subject: AccountInfo<'info> }
        #[derive(Clone)]
        pub struct Tag<'info> { pub authority_pda: AccountInfo<'info>, pub subject: AccountInfo<'info> }

        impl<'info> Record<'info> {
            fn metas(&self) -> Vec<AccountMeta> {
                vec![
                    AccountMeta::new_readonly(*self.authority_pda.key, true),
                    AccountMeta::new(*self.subject.key, false),
                ]
            }
            fn infos(&self, program: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
                vec![program.clone(), self.authority_pda.clone(), self.subject.clone()]
            }
        }
        impl<'info> Tag<'info> {
            fn metas(&self) -> Vec<AccountMeta> {
                vec![
                    AccountMeta::new_readonly(*self.authority_pda.key, true),
                    AccountMeta::new(*self.subject.key, false),
                ]
            }
            fn infos(&self, program: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
                vec![program.clone(), self.authority_pda.clone(), self.subject.clone()]
            }
        }

        pub fn record<'info>(ctx: CpiContext<'_, '_, '_, 'info, Record<'info>>, val: u64) -> Result<()> {
            let ix = Instruction {
                program_id: *ctx.program.key, // ← 動的ID採用
                accounts: ctx.accounts.metas(),
                data: val.to_le_bytes().to_vec(),
            };
            invoke_signed(&ix, &ctx.accounts.infos(&ctx.program), ctx.signer_seeds)?;
            Ok(())
        }

        pub fn tag<'info>(ctx: CpiContext<'_, '_, '_, 'info, Tag<'info>>, label: u16) -> Result<()> {
            let ix = Instruction {
                program_id: *ctx.program.key, // ← 動的ID採用
                accounts: ctx.accounts.metas(),
                data: label.to_le_bytes().to_vec(),
            };
            invoke_signed(&ix, &ctx.accounts.infos(&ctx.program), ctx.signer_seeds)?;
            Ok(())
        }
    }
}
