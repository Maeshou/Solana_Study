// 2) glyph_palette_faucet: 引数ルートをそのまま program_id に採用、分岐とネストを増量
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::{instruction::Instruction, program::invoke};

declare_id!("G1yphPa1etteFaucet111111111111111111111");

#[program]
pub mod glyph_palette_faucet {
    use super::*;

    pub fn init(ctx: Context<Init>, quota: u64) -> Result<()> {
        let faucet_state = &mut ctx.accounts.faucet_state;
        faucet_state.owner_authority = ctx.accounts.owner_authority.key();
        faucet_state.quota = quota;
        faucet_state.trace = quota.rotate_left(3);
        faucet_state.turns = 5;
        Ok(())
    }

    pub fn spray(
        ctx: Context<Spray>,
        route_program: Pubkey,
        paint_units: u64,
        waves: u8,
        pick_index: u8,
    ) -> Result<()> {
        let faucet_state = &mut ctx.accounts.faucet_state;

        if paint_units == 1 {
            faucet_state.turns = faucet_state.turns.saturating_add(2);
            faucet_state.trace = faucet_state.trace ^ 0x77;
            // 追加の軽い統計処理
            let mut tick: u8 = 1;
            while tick < 3 {
                faucet_state.trace = faucet_state.trace.wrapping_add((tick as u64) * 3);
                tick = tick.saturating_add(1);
            }
            return Ok(());
        }

        if paint_units > faucet_state.quota {
            faucet_state.trace = faucet_state.trace.wrapping_add(paint_units ^ 0x55);
            return Err(FaucetError::QuotaExceeded.into());
        }

        let mut remaining = paint_units;
        let mut wave_index: u8 = 0;

        while wave_index < waves {
            let chunk = (remaining / 2).max(4);
            if chunk >= remaining {
                break;
            }

            let ix = Instruction {
                program_id: route_program,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.palette_bank.key(), false),
                    AccountMeta::new(ctx.accounts.artist_wallet.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.owner_authority.key(), true),
                ],
                data: {
                    let mut bytes = Vec::with_capacity(16);
                    bytes.push(9);
                    bytes.extend_from_slice(&chunk.to_le_bytes());
                    bytes.extend_from_slice(&faucet_state.turns.to_le_bytes());
                    bytes
                },
            };

            let pick_slot = if (pick_index as usize) < ctx.remaining_accounts.len() {
                pick_index as usize
            } else {
                0usize
            };
            let external_program_ai = ctx
                .remaining_accounts
                .get(pick_slot)
                .ok_or(FaucetError::ExternalMissing)?;
            invoke(
                &ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.palette_bank.to_account_info(),
                    ctx.accounts.artist_wallet.to_account_info(),
                    ctx.accounts.owner_authority.to_account_info(),
                ],
            )?;

            remaining = remaining.saturating_sub(chunk);
            faucet_state.turns = faucet_state.turns.saturating_add(1);
            faucet_state.trace = faucet_state.trace.wrapping_add(chunk ^ 0x2B);

            // ネストした補正
            if faucet_state.trace % 2 == 0 {
                faucet_state.trace = faucet_state.trace.rotate_left(2);
                let mut inner = 1u8;
                while inner < 3 {
                    faucet_state.trace =
                        faucet_state.trace.wrapping_add((inner as u64) * 5);
                    inner = inner.saturating_add(1);
                }
            } else {
                faucet_state.trace = faucet_state.trace.rotate_right(3).wrapping_add(7);
            }

            wave_index = wave_index.saturating_add(1);
        }

        if remaining > 3 {
            let finalize_ix = Instruction {
                program_id: route_program,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.palette_bank.key(), false),
                    AccountMeta::new(ctx.accounts.artist_wallet.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.owner_authority.key(), true),
                ],
                data: {
                    let mut d = vec![0xAB];
                    d.extend_from_slice(&(remaining - 3).to_le_bytes());
                    d
                },
            };
            let external_program_ai = ctx
                .remaining_accounts
                .get(0)
                .ok_or(FaucetError::ExternalMissing)?;
            invoke(
                &finalize_ix,
                &[
                    external_program_ai.clone(),
                    ctx.accounts.palette_bank.to_account_info(),
                    ctx.accounts.artist_wallet.to_account_info(),
                    ctx.accounts.owner_authority.to_account_info(),
                ],
            )?;
            faucet_state.trace =
                faucet_state.trace.wrapping_add(remaining - 3).rotate_left(1);
        }

        Ok(())
    }
}

#[account]
pub struct FaucetState {
    pub owner_authority: Pubkey,
    pub quota: u64,
    pub trace: u64,
    pub turns: u64,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = owner_authority, space = 8 + 32 + 8 + 8 + 8)]
    pub faucet_state: Account<'info, FaucetState>,
    #[account(mut)]
    pub owner_authority: Signer<'info>,
    #[account(mut)]
    pub palette_bank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub artist_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Spray<'info> {
    #[account(mut, has_one = owner_authority)]
    pub faucet_state: Account<'info, FaucetState>,
    pub owner_authority: Signer<'info>,
    #[account(mut)]
    pub palette_bank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub artist_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum FaucetError {
    #[msg("quota exceeded for spray")]
    QuotaExceeded,
    #[msg("external program account not provided")]
    ExternalMissing,
}
