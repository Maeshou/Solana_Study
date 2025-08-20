// 例C) DailySelectorWithSigner: 日替わりで切替。新しい日→ヘッダ記録＋本記録の2回、同日→増分のみ
// 分岐内の処理：PDA署名付きCPIを複数回、メトリクス更新も分岐ごとに別
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};

declare_id!("DailySelSignAAAAABBBBCCCCDDDDEEEEFFFFHHHH");

#[program]
pub mod daily_selector_with_signer {
    use super::*;
    pub fn setup(ctx: Context<SetupMeta>, bump: u8) -> Result<()> {
        let meta = &mut ctx.accounts.meta_state;
        meta.owner = ctx.accounts.owner.key();
        meta.bump = bump;
        meta.last_day = 0;
        meta.daily_counter = 0;
        meta.rolling_total = 0;
        Ok(())
    }

    pub fn record(ctx: Context<RecordWithSigner>, raw_value: u64) -> Result<()> {
        let meta = &mut ctx.accounts.meta_state;
        let today = (Clock::get()?.unix_timestamp as u64) / 86_400;

        let pda_seeds: &[&[u8]] = &[b"meta", meta.owner.as_ref(), &[meta.bump]];
        let signer = &[pda_seeds];

        if meta.last_day != today {
            // 新しい日：ヘッダー的な値を先に記録 → 本値を記録 → カウンタ・最終日更新
            let header_value = (raw_value / 10).max(1);
            let cpi_header = CpiContext::new_with_signer(
                ctx.accounts.alt_program.to_account_info(),
                bad_registry::cpi::Record {
                    authority_pda: meta.to_account_info(),
                    subject: ctx.accounts.subject.to_account_info(),
                },
                signer,
            );
            bad_registry::cpi::record(cpi_header, header_value)?;

            let cpi_full = CpiContext::new_with_signer(
                ctx.accounts.alt_program.to_account_info(),
                bad_registry::cpi::Record {
                    authority_pda: meta.to_account_info(),
                    subject: ctx.accounts.subject.to_account_info(),
                },
                signer,
            );
            bad_registry::cpi::record(cpi_full, raw_value)?;

            meta.daily_counter = 1;
            meta.last_day = today;
            meta.rolling_total = meta.rolling_total.saturating_add(raw_value);
            msg!("new day: header={}, value={}", header_value, raw_value);
        } else {
            // 同じ日：増分だけ記録（半分）→ カウンタ・合計更新
            let incremental = (raw_value / 2).max(1);
            let cpi_inc = CpiContext::new_with_signer(
                ctx.accounts.primary_program.to_account_info(),
                bad_registry::cpi::Record {
                    authority_pda: meta.to_account_info(),
                    subject: ctx.accounts.subject.to_account_info(),
                },
                signer,
            );
            bad_registry::cpi::record(cpi_inc, incremental)?;

            meta.daily_counter = meta.daily_counter.saturating_add(1);
            meta.rolling_total = meta.rolling_total.saturating_add(incremental);
            msg!("same day: incremental={}", incremental);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupMeta<'info> {
    #[account(init, payer = owner, seeds=[b"meta", owner.key().as_ref()], bump, space = 8 + 32 + 8 + 1 + 8 + 8)]
    pub meta_state: Account<'info, MetaState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RecordWithSigner<'info> {
    #[account(mut, has_one = owner)]
    pub meta_state: Account<'info, MetaState>,
    pub owner: Signer<'info>,
    /// CHECK:
    pub subject: UncheckedAccount<'info>,
    /// CHECK:
    pub primary_program: UncheckedAccount<'info>,
    /// CHECK:
    pub alt_program: UncheckedAccount<'info>,
}
#[account]
pub struct MetaState {
    pub owner: Pubkey,
    pub last_day: u64,
    pub bump: u8,
    pub daily_counter: u64,
    pub rolling_total: u64,
}

// --- 動的IDを使う外部CPIラッパ（ここがArbitraryの根） ---
pub mod bad_registry {
    use super::*;
    pub mod cpi {
        use super::*;
        #[derive(Clone)]
        pub struct Record<'info> { pub authority_pda: AccountInfo<'info>, pub subject: AccountInfo<'info> }
        impl<'info> Record<'info> {
            fn to_metas(&self) -> Vec<AccountMeta> {
                vec![
                    AccountMeta::new_readonly(*self.authority_pda.key, true),
                    AccountMeta::new(*self.subject.key, false),
                ]
            }
            fn to_infos(&self, program: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
                vec![program.clone(), self.authority_pda.clone(), self.subject.clone()]
            }
        }
        pub fn record<'info>(ctx: CpiContext<'_, '_, '_, 'info, Record<'info>>, val: u64) -> Result<()> {
            let ix = Instruction {
                program_id: *ctx.program.key, // ← ここが“動的ID採用”
                accounts: ctx.accounts.to_metas(),
                data: val.to_le_bytes().to_vec(),
            };
            invoke_signed(&ix, &ctx.accounts.to_infos(&ctx.program), ctx.signer_seeds)?;
            Ok(())
        }
    }
}
