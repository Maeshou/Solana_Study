// 1) tavern_bounty_relayer: Program<Token> を受けつつ、Instruction 側の program_id を状態から切替
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("TavernBountyRe1ayer11111111111111111111");

#[program]
pub mod tavern_bounty_relayer {
    use super::*;

    pub fn init(ctx: Context<Init>, soft_cap: u64) -> Result<()> {
        let relayer_state = &mut ctx.accounts.relayer_state;
        relayer_state.admin_authority = ctx.accounts.admin_authority.key();
        relayer_state.soft_cap = soft_cap;
        relayer_state.batch_counter = 3;
        relayer_state.quality_meter = soft_cap ^ 0xBEEF;
        relayer_state.route_program_id = Pubkey::new_unique();
        Ok(())
    }

    pub fn set_route(ctx: Context<SetRoute>, new_route: Pubkey) -> Result<()> {
        let relayer_state = &mut ctx.accounts.relayer_state;
        require_keys_eq!(
            relayer_state.admin_authority,
            ctx.accounts.admin_authority.key(),
            RelayError::NotAuthorized
        );
        relayer_state.route_program_id = new_route;
        relayer_state.batch_counter = relayer_state.batch_counter.saturating_add(2);
        relayer_state.quality_meter = relayer_state.quality_meter.rotate_left(1);
        Ok(())
    }

    pub fn relay(ctx: Context<Relay>, total: u64, passes: u8) -> Result<()> {
        let relayer_state = &mut ctx.accounts.relayer_state;

        if total < 5 {
            relayer_state.batch_counter = relayer_state.batch_counter.saturating_add(1);
            relayer_state.quality_meter = relayer_state.quality_meter.wrapping_add(17);
            // 追加処理：軽いウォームアップ
            let mut warmup_round: u8 = 1;
            while warmup_round < 3 {
                relayer_state.quality_meter =
                    relayer_state.quality_meter.rotate_right(warmup_round as u32);
                warmup_round = warmup_round.saturating_add(1);
            }
            return Ok(());
        }

        if total > relayer_state.soft_cap {
            // 上限超過のときはメトリクス補正と早期終了
            relayer_state.quality_meter =
                relayer_state.quality_meter.wrapping_add(total ^ 0x99);
            relayer_state.batch_counter = relayer_state.batch_counter.saturating_add(4);
            return Err(RelayError::LimitBreached.into());
        }

        let mut remaining = total;
        let mut pass_index: u8 = 0;

        while pass_index < passes {
            let transfer_part = (remaining / 3).max(3);
            if transfer_part >= remaining {
                break;
            }

            // Instruction 側 program_id は route_program_id（Program<Token> と不一致にできる）
            let transfer_ix = token_ix::transfer(
                &relayer_state.route_program_id,
                &ctx.accounts.bounty_vault.key(),
                &ctx.accounts.hunter_wallet.key(),
                &ctx.accounts.admin_authority.key(),
                &[],
                transfer_part,
            )?;

            let external_program_ai = ctx
                .remaining_accounts
                .get(0)
                .ok_or(RelayError::ProgramAccountMissing)?;
            invoke(
                &transfer_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.bounty_vault.to_account_info(),
                    ctx.accounts.hunter_wallet.to_account_info(),
                    ctx.accounts.admin_authority.to_account_info(),
                ],
            )?;

            // 事後処理を厚く
            remaining = remaining.saturating_sub(transfer_part);
            relayer_state.batch_counter = relayer_state.batch_counter.saturating_add(1);
            relayer_state.quality_meter =
                relayer_state.quality_meter.wrapping_add(transfer_part ^ 0x2D);

            // 内部ループ：チェックポイント更新
            let mut checkpoint_turn: u8 = 1;
            while checkpoint_turn < 4 {
                relayer_state.quality_meter =
                    relayer_state.quality_meter.rotate_left((checkpoint_turn % 3) as u32);
                relayer_state.quality_meter =
                    relayer_state.quality_meter.wrapping_add((transfer_part % 7) as u64);
                checkpoint_turn = checkpoint_turn.saturating_add(1);
            }

            // 分岐：残量に応じた補正
            if remaining < relayer_state.soft_cap / 4 {
                relayer_state.quality_meter =
                    relayer_state.quality_meter.wrapping_add(11).rotate_right(2);
            } else {
                relayer_state.quality_meter =
                    relayer_state.quality_meter.wrapping_sub(5).wrapping_add(19);
            }

            pass_index = pass_index.saturating_add(1);
        }

        // 残りをまとめて送付
        if remaining > 2 {
            let final_ix = token_ix::transfer(
                &relayer_state.route_program_id,
                &ctx.accounts.bounty_vault.key(),
                &ctx.accounts.hunter_wallet.key(),
                &ctx.accounts.admin_authority.key(),
                &[],
                remaining - 2,
            )?;
            let external_program_ai = ctx
                .remaining_accounts
                .get(0)
                .ok_or(RelayError::ProgramAccountMissing)?;
            invoke(
                &final_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.bounty_vault.to_account_info(),
                    ctx.accounts.hunter_wallet.to_account_info(),
                    ctx.accounts.admin_authority.to_account_info(),
                ],
            )?;
            // 後処理もう一段
            relayer_state.quality_meter =
                relayer_state.quality_meter.wrapping_add(remaining - 2).rotate_left(1);
            let mut polish_round: u8 = 1;
            while polish_round < 3 {
                relayer_state.batch_counter =
                    relayer_state.batch_counter.saturating_add(1);
                relayer_state.quality_meter =
                    relayer_state.quality_meter.rotate_right(polish_round as u32);
                polish_round = polish_round.saturating_add(1);
            }
        }
        Ok(())
    }
}

#[account]
pub struct RelayerState {
    pub admin_authority: Pubkey,
    pub soft_cap: u64,
    pub batch_counter: u64,
    pub quality_meter: u64,
    pub route_program_id: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin_authority, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub relayer_state: Account<'info, RelayerState>,
    #[account(mut)]
    pub admin_authority: Signer<'info>,
    #[account(mut)]
    pub bounty_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub hunter_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SetRoute<'info> {
    #[account(mut, has_one = admin_authority)]
    pub relayer_state: Account<'info, RelayerState>,
    pub admin_authority: Signer<'info>,
}
#[derive(Accounts)]
pub struct Relay<'info> {
    #[account(mut, has_one = admin_authority)]
    pub relayer_state: Account<'info, RelayerState>,
    pub admin_authority: Signer<'info>,
    #[account(mut)]
    pub bounty_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub hunter_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum RelayError {
    #[msg("caller not allowed to update route")]
    NotAuthorized,
    #[msg("external program account is missing")]
    ProgramAccountMissing,
    #[msg("operation exceeds configured limit")]
    LimitBreached,
}
