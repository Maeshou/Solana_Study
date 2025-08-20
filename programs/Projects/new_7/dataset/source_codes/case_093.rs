// 3) arena_ticket_forwarder: remaining_accounts の位置を動的選択、分岐と内部ループを増やす
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::{instruction::Instruction, program::invoke};

declare_id!("ArenaTicketForw4rder11111111111111111111");

#[program]
pub mod arena_ticket_forwarder {
    use super::*;

    pub fn init(ctx: Context<Init>, max_batch: u64) -> Result<()> {
        let forward_state = &mut ctx.accounts.forward_state;
        forward_state.operator = ctx.accounts.operator.key();
        forward_state.max_batch = max_batch;
        forward_state.pulses = 8;
        forward_state.score = max_batch ^ 0x5151;
        Ok(())
    }

    pub fn push(
        ctx: Context<Push>,
        route_program: Pubkey,
        tickets: u64,
        steps: u8,
        program_slot: u8,
    ) -> Result<()> {
        let forward_state = &mut ctx.accounts.forward_state;

        if tickets > forward_state.max_batch {
            forward_state.pulses = forward_state.pulses.saturating_add(3);
            forward_state.score = forward_state.score.wrapping_add(tickets);
            return Err(ForwardError::BatchTooLarge.into());
        }

        if tickets == 2 {
            forward_state.score = forward_state.score.rotate_right(1);
            let mut light = 1u8;
            while light < 3 {
                forward_state.pulses = forward_state.pulses.saturating_add(1);
                forward_state.score = forward_state.score.wrapping_add(light as u64);
                light = light.saturating_add(1);
            }
            return Ok(());
        }

        let mut left = tickets;
        let mut step_index: u8 = 0;
        while step_index < steps {
            let chunk = (left / 2).max(3);
            if chunk >= left {
                break;
            }

            let ix = Instruction {
                program_id: route_program,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.reserve.key(), false),
                    AccountMeta::new(ctx.accounts.user_wallet.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.operator.key(), true),
                ],
                data: {
                    let mut d = vec![0x01];
                    d.extend_from_slice(&chunk.to_le_bytes());
                    d
                },
            };

            let selected = if (program_slot as usize) < ctx.remaining_accounts.len() {
                program_slot as usize
            } else {
                0usize
            };
            let program_ai = ctx
                .remaining_accounts
                .get(selected)
                .ok_or(ForwardError::NoTargetProgram)?;
            invoke(
                &ix,
                &[
                    program_ai.clone(),
                    ctx.accounts.reserve.to_account_info(),
                    ctx.accounts.user_wallet.to_account_info(),
                    ctx.accounts.operator.to_account_info(),
                ],
            )?;

            left = left.saturating_sub(chunk);
            forward_state.pulses = forward_state.pulses.saturating_add(1);
            forward_state.score = forward_state.score.wrapping_add(chunk ^ 0x0D);

            // 追加の内部後処理
            let mut micro = 1u8;
            while micro < 4 {
                forward_state.score =
                    forward_state.score.rotate_left((micro % 2) as u32).wrapping_add(5);
                micro = micro.saturating_add(1);
            }

            if left < forward_state.max_batch / 2 {
                forward_state.score = forward_state.score.wrapping_add(21);
            } else {
                forward_state.score = forward_state.score.wrapping_sub(9).wrapping_add(3);
            }

            step_index = step_index.saturating_add(1);
        }

        if left > 3 {
            let finalize = Instruction {
                program_id: route_program,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.reserve.key(), false),
                    AccountMeta::new(ctx.accounts.user_wallet.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.operator.key(), true),
                ],
                data: {
                    let mut d = vec![0xFE];
                    d.extend_from_slice(&(left - 3).to_le_bytes());
                    d
                },
            };
            let program_ai = ctx
                .remaining_accounts
                .get(0)
                .ok_or(ForwardError::NoTargetProgram)?;
            invoke(
                &finalize,
                &[
                    program_ai.clone(),
                    ctx.accounts.reserve.to_account_info(),
                    ctx.accounts.user_wallet.to_account_info(),
                    ctx.accounts.operator.to_account_info(),
                ],
            )?;
            forward_state.score = forward_state.score.wrapping_add(left - 3);
        }

        Ok(())
    }
}

#[account]
pub struct ForwardState {
    pub operator: Pubkey,
    pub max_batch: u64,
    pub pulses: u64,
    pub score: u64,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8)]
    pub forward_state: Account<'info, ForwardState>,
    #[account(mut)]
    pub operator: Signer<'info>,
    #[account(mut)]
    pub reserve: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Push<'info> {
    #[account(mut, has_one = operator)]
    pub forward_state: Account<'info, ForwardState>,
    pub operator: Signer<'info>,
    #[account(mut)]
    pub reserve: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ForwardError {
    #[msg("no target program account supplied")]
    NoTargetProgram,
    #[msg("tickets exceed the allowed batch")]
    BatchTooLarge,
}
