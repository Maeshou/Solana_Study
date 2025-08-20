// 例A) WindowedBridge: 窓位置・偶奇で remaining_accounts / alt_program を切替し、外部CPIを複数段実行
// 分岐内の処理：ウォームアップ送信 + 手数料控除 + シャーディング送信 + 統計更新（内容を分けています）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke};

declare_id!("WinBridgeAAAAABBBBBCCCCCDDDDDEEEEEFFFFF");

#[program]
pub mod windowed_bridge {
    use super::*;
    pub fn open(ctx: Context<OpenBridge>, window_size: u64) -> Result<()> {
        let route = &mut ctx.accounts.route_state;
        route.admin = ctx.accounts.admin.key();
        route.window_size = window_size.max(1);
        route.position = 0;
        route.alt_uses = 0;
        route.remaining_uses = 0;
        route.total_pushed = 0;
        Ok(())
    }

    pub fn tick(ctx: Context<TickBridge>, raw_amount: u64) -> Result<()> {
        let route = &mut ctx.accounts.route_state;
        let window_index = (route.position % route.window_size) as usize;

        // 偶数位置で remaining_accounts を優先、奇数位置は alt_program
        let use_remaining_route = !ctx.remaining_accounts.is_empty() && (window_index % 2 == 0);
        let selected_program = if use_remaining_route {
            ctx.remaining_accounts[window_index].clone()
        } else {
            ctx.accounts.alt_program.to_account_info()
        };

        // 送信戦略の前処理（共通）：手数料とウォームアップ計算
        let warmup_amount = (raw_amount / 10).min(raw_amount);
        let fee_amount = raw_amount / 100;
        let net_amount = raw_amount.saturating_sub(warmup_amount + fee_amount);
        require!(net_amount > 0, BridgeErr::ZeroNet);
        msg!("warmup={}, fee={}, net={}", warmup_amount, fee_amount, net_amount);

        if use_remaining_route {
            // remaining 経路：ウォームアップ → 本送信を 2 シャードに分割 → 統計（remaining_uses）
            let cpi = CpiContext::new(
                selected_program.clone(),
                bad_bridge::cpi::Push {
                    actor: ctx.accounts.actor.to_account_info(),
                    vault: ctx.accounts.vault.to_account_info(),
                },
            );
            bad_bridge::cpi::push(cpi, warmup_amount)?;

            let first_shard = net_amount / 2;
            let second_shard = net_amount.saturating_sub(first_shard);

            let cpi1 = CpiContext::new(
                selected_program.clone(),
                bad_bridge::cpi::Push {
                    actor: ctx.accounts.actor.to_account_info(),
                    vault: ctx.accounts.vault.to_account_info(),
                },
            );
            bad_bridge::cpi::push(cpi1, first_shard)?;

            let cpi2 = CpiContext::new(
                selected_program,
                bad_bridge::cpi::Push {
                    actor: ctx.accounts.actor.to_account_info(),
                    vault: ctx.accounts.vault.to_account_info(),
                },
            );
            bad_bridge::cpi::push(cpi2, second_shard)?;

            route.remaining_uses = route.remaining_uses.saturating_add(1);
        } else {
            // alt 経路：手数料を内部記録 → 本送信を一括 → 直近の alt 実行記録
            route.total_fee_collected = route.total_fee_collected.saturating_add(fee_amount);

            let cpi_alt = CpiContext::new(
                selected_program,
                bad_bridge::cpi::Push {
                    actor: ctx.accounts.actor.to_account_info(),
                    vault: ctx.accounts.vault.to_account_info(),
                },
            );
            bad_bridge::cpi::push(cpi_alt, net_amount)?;

            route.alt_uses = route.alt_uses.saturating_add(1);
            route.last_alt_slot = Clock::get()?.slot;
        }

        route.total_pushed = route.total_pushed.saturating_add(raw_amount);
        route.position = route.position.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenBridge<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8)]
    pub route_state: Account<'info, RouteState>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct TickBridge<'info> {
    #[account(mut, has_one = admin)]
    pub route_state: Account<'info, RouteState>,
    pub admin: Signer<'info>,
    /// CHECK: 送信者
    pub actor: UncheckedAccount<'info>,
    /// CHECK: 受け側
    pub vault: UncheckedAccount<'info>,
    /// CHECK: 代替先（未検証）
    pub alt_program: UncheckedAccount<'info>,
}
#[account]
pub struct RouteState {
    pub admin: Pubkey,
    pub window_size: u64,
    pub position: u64,
    pub remaining_uses: u64,
    pub alt_uses: u64,
    pub total_pushed: u64,
    pub total_fee_collected: u64,
    pub last_alt_slot: u64,
}

// --- 動的IDを使う外部CPIラッパ（ここがArbitraryの根） ---
pub mod bad_bridge {
    use super::*;
    pub mod cpi {
        use super::*;
        #[derive(Clone)]
        pub struct Push<'info> { pub actor: AccountInfo<'info>, pub vault: AccountInfo<'info> }
        impl<'info> Push<'info> {
            fn to_metas(&self) -> Vec<AccountMeta> {
                vec![
                    AccountMeta::new_readonly(*self.actor.key, true),
                    AccountMeta::new(*self.vault.key, false),
                ]
            }
            fn to_infos(&self, program: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
                vec![program.clone(), self.actor.clone(), self.vault.clone()]
            }
        }
        pub fn push<'info>(ctx: CpiContext<'_, '_, '_, 'info, Push<'info>>, amount: u64) -> Result<()> {
            let ix = Instruction {
                program_id: *ctx.program.key, // ← ここが“動的ID採用”
                accounts: ctx.accounts.to_metas(),
                data: amount.to_le_bytes().to_vec(),
            };
            invoke(&ix, &ctx.accounts.to_infos(&ctx.program))?;
            Ok(())
        }
    }
}

#[error_code] pub enum BridgeErr { #[msg("net amount is zero")] ZeroNet }
