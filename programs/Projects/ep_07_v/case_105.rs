// 例1) ShardBucketRouter: 累積量とストライドでバケット分岐し、残り口座 or alt_program に対して
//   - ウォームアップ送信
//   - 手数料積立
//   - シャーディング（分割）送信
//   - メトリクス更新
// を行う。分岐ごとに処理内容・統計の更新先が異なる。
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke};

declare_id!("ShardBucketAAAAABBBBCCCCDDDDEEEEFFFF0001");

#[program]
pub mod shard_bucket_router {
    use super::*;
    pub fn configure(ctx: Context<ConfigureRouter>, shard_base: u64, stride: u64) -> Result<()> {
        let stats = &mut ctx.accounts.router_stats;
        stats.operator = ctx.accounts.operator.key();
        stats.shard_base = shard_base.max(2);
        stats.stride = stride.max(3);
        stats.running_sum = 0;
        stats.fallback_hits = 0;
        stats.remaining_hits = 0;
        stats.total_collected_fee = 0;
        stats.last_switch_slot = 0;
        Ok(())
    }

    pub fn route(ctx: Context<RouteShard>, total_amount: u64) -> Result<()> {
        let stats = &mut ctx.accounts.router_stats;

        // 前処理：ウォームアップ + 手数料 + 正味額
        let warmup_amount = (total_amount % stats.stride).max(1);
        let fee_amount = total_amount / 200; // 0.5%
        let net_amount = total_amount.saturating_sub(warmup_amount + fee_amount);
        require!(net_amount > 0, RouteErr::NonPositiveNet);

        // 送信先プログラムの選択（累積/ストライドで2バケット）
        let bucket_index = (stats.running_sum / stats.stride) % 2;
        let selected_program_account = if bucket_index == 0 {
            // remaining_accounts から選択（無ければ alt 側へフォールバック）
            if let Some(a) = ctx.remaining_accounts.first() {
                stats.remaining_hits = stats.remaining_hits.saturating_add(1);
                a.clone()
            } else {
                stats.fallback_hits = stats.fallback_hits.saturating_add(1);
                ctx.accounts.alt_program.to_account_info()
            }
        } else {
            // 代替先を使用し、最後にスロットを記録
            stats.fallback_hits = stats.fallback_hits.saturating_add(1);
            stats.last_switch_slot = Clock::get()?.slot;
            ctx.accounts.alt_program.to_account_info()
        };

        // ウォームアップ送信（小額）
        let cpi_warmup = CpiContext::new(
            selected_program_account.clone(),
            bad_bridge::cpi::Push {
                actor: ctx.accounts.sender.to_account_info(),
                vault: ctx.accounts.receiver_vault.to_account_info(),
            },
        );
        bad_bridge::cpi::push(cpi_warmup, warmup_amount)?;

        // 正味額をシャーディングして2回送信
        let shard_size = (net_amount / stats.shard_base).max(1);
        let first_chunk = shard_size.min(net_amount);
        let second_chunk = net_amount.saturating_sub(first_chunk);

        let cpi_first = CpiContext::new(
            selected_program_account.clone(),
            bad_bridge::cpi::Push {
                actor: ctx.accounts.sender.to_account_info(),
                vault: ctx.accounts.receiver_vault.to_account_info(),
            },
        );
        bad_bridge::cpi::push(cpi_first, first_chunk)?;

        if second_chunk > 0 {
            let cpi_second = CpiContext::new(
                selected_program_account,
                bad_bridge::cpi::Push {
                    actor: ctx.accounts.sender.to_account_info(),
                    vault: ctx.accounts.receiver_vault.to_account_info(),
                },
            );
            bad_bridge::cpi::push(cpi_second, second_chunk)?;
        }

        // メトリクス更新
        stats.total_collected_fee = stats.total_collected_fee.saturating_add(fee_amount);
        stats.running_sum = stats.running_sum.saturating_add(total_amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigureRouter<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8)]
    pub router_stats: Account<'info, RouterStats>,
    #[account(mut)] pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RouteShard<'info> {
    #[account(mut, has_one = operator)]
    pub router_stats: Account<'info, RouterStats>,
    pub operator: Signer<'info>,
    /// CHECK: 送信者
    pub sender: UncheckedAccount<'info>,
    /// CHECK: 受け口
    pub receiver_vault: UncheckedAccount<'info>,
    /// CHECK: フォールバック先（未検証）
    pub alt_program: UncheckedAccount<'info>,
}

#[account]
pub struct RouterStats {
    pub operator: Pubkey,
    pub shard_base: u64,
    pub stride: u64,
    pub running_sum: u64,
    pub remaining_hits: u64,
    pub fallback_hits: u64,
    pub total_collected_fee: u64,
    pub last_switch_slot: u64,
}

#[error_code]
pub enum RouteErr { #[msg("net amount must be positive")] NonPositiveNet }

// --- 動的IDを使う外部CPIラッパ（Arbitrary CPI の根） ---
pub mod bad_bridge {
    use super::*;
    pub mod cpi {
        use super::*;
        #[derive(Clone)]
        pub struct Push<'info> { pub actor: AccountInfo<'info>, pub vault: AccountInfo<'info> }
        impl<'info> Push<'info> {
            fn metas(&self) -> Vec<AccountMeta> {
                vec![
                    AccountMeta::new_readonly(*self.actor.key, true),
                    AccountMeta::new(*self.vault.key, false),
                ]
            }
            fn infos(&self, program: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
                vec![program.clone(), self.actor.clone(), self.vault.clone()]
            }
        }
        pub fn push<'info>(ctx: CpiContext<'_, '_, '_, 'info, Push<'info>>, amt: u64) -> Result<()> {
            let ix = Instruction {
                program_id: *ctx.program.key, // ← 動的ID採用
                accounts: ctx.accounts.metas(),
                data: amt.to_le_bytes().to_vec(),
            };
            invoke(&ix, &ctx.accounts.infos(&ctx.program))?;
            Ok(())
        }
    }
}
