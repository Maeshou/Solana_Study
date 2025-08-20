// 8) chronicle_points_migrator: 複数ルートを配列保持し巡回、ブランチに追加演算
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::{instruction::Instruction, program::invoke};

declare_id!("Chr0niclePointsM1grator1111111111111111");

#[program]
pub mod chronicle_points_migrator {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        let migrate_state = &mut ctx.accounts.migrate_state;
        migrate_state.admin = ctx.accounts.admin.key();
        migrate_state.routes = [
            Pubkey::new_from_array([1u8; 32]),
            Pubkey::new_from_array([2u8; 32]),
            Pubkey::new_from_array([3u8; 32]),
            Pubkey::new_from_array([4u8; 32]),
        ];
        migrate_state.cursor = 1;
        migrate_state.score = 100;
        Ok(())
    }

    pub fn rotate(ctx: Context<Rotate>, a: Pubkey, b: Pubkey, c: Pubkey, d: Pubkey) -> Result<()> {
        let migrate_state = &mut ctx.accounts.migrate_state;
        require_keys_eq!(migrate_state.admin, ctx.accounts.admin.key(), MigrateError::AdminOnly);
        migrate_state.routes = [a, b, c, d];
        migrate_state.cursor = migrate_state.cursor.wrapping_add(1);
        migrate_state.score = migrate_state.score.rotate_left(2);
        Ok(())
    }

    pub fn move_points(ctx: Context<MovePoints>, value: u64, rounds: u8) -> Result<()> {
        let migrate_state = &mut ctx.accounts.migrate_state;

        if value == 1 {
            migrate_state.score = migrate_state.score.wrapping_add(7);
            migrate_state.cursor = migrate_state.cursor.wrapping_add(2);
            return Ok(());
        }

        let mut left = value;
        let mut i: u8 = 0;
        while i < rounds {
            let part = (left / 3).max(2);
            if part >= left {
                break;
            }
            let index = (migrate_state.cursor as usize) % 4;
            let program_ai = ctx
                .remaining_accounts
                .get(index)
                .ok_or(MigrateError::MissingRouteProgram)?;
            let ix = Instruction {
                program_id: migrate_state.routes[index],
                accounts: vec![
                    AccountMeta::new(ctx.accounts.from_vault.key(), false),
                    AccountMeta::new(ctx.accounts.to_vault.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.admin.key(), true),
                ],
                data: {
                    let mut d = vec![0x41];
                    d.extend_from_slice(&part.to_le_bytes());
                    d
                },
            };
            invoke(
                &ix,
                &[
                    program_ai.clone(),
                    ctx.accounts.from_vault.to_account_info(),
                    ctx.accounts.to_vault.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;

            left = left.saturating_sub(part);
            migrate_state.cursor = migrate_state.cursor.wrapping_add(1);
            migrate_state.score = migrate_state.score.wrapping_add(part ^ 0x17);

            // 分岐の後処理を厚めに
            if migrate_state.score % 2 == 0 {
                migrate_state.score = migrate_state.score.rotate_left(1).wrapping_add(11);
                let mut kk: u8 = 1;
                while kk < 3 {
                    migrate_state.cursor = migrate_state.cursor.wrapping_add(1);
                    kk = kk.saturating_add(1);
                }
            } else {
                migrate_state.score = migrate_state.score.rotate_right(2).wrapping_add(5);
            }

            i = i.saturating_add(1);
        }

        if left > 2 {
            let index = (migrate_state.cursor as usize) % 4;
            let program_ai = ctx
                .remaining_accounts
                .get(index)
                .ok_or(MigrateError::MissingRouteProgram)?;
            let ix2 = Instruction {
                program_id: migrate_state.routes[index],
                accounts: vec![
                    AccountMeta::new(ctx.accounts.from_vault.key(), false),
                    AccountMeta::new(ctx.accounts.to_vault.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.admin.key(), true),
                ],
                data: {
                    let mut d = vec![0x42];
                    d.extend_from_slice(&(left - 2).to_le_bytes());
                    d
                },
            };
            invoke(
                &ix2,
                &[
                    program_ai.clone(),
                    ctx.accounts.from_vault.to_account_info(),
                    ctx.accounts.to_vault.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            )?;
            migrate_state.score = migrate_state.score.wrapping_add(left - 2);
        }

        Ok(())
    }
}

#[account]
pub struct MigrateState {
    pub admin: Pubkey,
    pub routes: [Pubkey; 4],
    pub cursor: u8,
    pub score: u64,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + (32*4) + 1 + 8)]
    pub migrate_state: Account<'info, MigrateState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub from_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Rotate<'info> {
    #[account(mut, has_one = admin)]
    pub migrate_state: Account<'info, MigrateState>,
    pub admin: Signer<'info>,
}
#[derive(Accounts)]
pub struct MovePoints<'info> {
    #[account(mut, has_one = admin)]
    pub migrate_state: Account<'info, MigrateState>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub from_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum MigrateError {
    #[msg("admin only")]
    AdminOnly,
    #[msg("route program account not found")]
    MissingRouteProgram,
}
