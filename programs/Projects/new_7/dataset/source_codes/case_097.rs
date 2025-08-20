// 7) echo_oracle_bridge: A/B 二段ブリッジ（両方とも外部指定），分岐後の補正を増量
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::{instruction::Instruction, program::invoke};

declare_id!("Echo0racleBr1dge111111111111111111111111");

#[program]
pub mod echo_oracle_bridge {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        let bridge_state = &mut ctx.accounts.bridge_state;
        bridge_state.admin = ctx.accounts.admin.key();
        bridge_state.metric = 77;
        bridge_state.route_a = Pubkey::new_unique();
        bridge_state.route_b = Pubkey::new_unique();
        Ok(())
    }

    pub fn set_routes(ctx: Context<SetRoutes>, a: Pubkey, b: Pubkey) -> Result<()> {
        let bridge_state = &mut ctx.accounts.bridge_state;
        require_keys_eq!(bridge_state.admin, ctx.accounts.admin.key(), BridgeError::AdminOnly);
        bridge_state.route_a = a;
        bridge_state.route_b = b;
        bridge_state.metric = bridge_state.metric.wrapping_add(5);
        Ok(())
    }

    pub fn pipe(ctx: Context<Pipe>, amount: u64) -> Result<()> {
        let bridge_state = &mut ctx.accounts.bridge_state;
        if amount == 0 {
            bridge_state.metric = bridge_state.metric.rotate_left(1);
            let mut bump: u8 = 1;
            while bump < 3 {
                bridge_state.metric = bridge_state.metric.wrapping_add(bump as u64);
                bump = bump.saturating_add(1);
            }
            return Ok(());
        }

        // A 経由
        let ix_a = Instruction {
            program_id: bridge_state.route_a,
            accounts: vec![
                AccountMeta::new(ctx.accounts.buffer.key(), false),
                AccountMeta::new(ctx.accounts.pool.key(), false),
                AccountMeta::new_readonly(ctx.accounts.admin.key(), true),
            ],
            data: {
                let mut d = vec![1];
                d.extend_from_slice(&(amount / 2).to_le_bytes());
                d
            },
        };
        let prog_a = ctx
            .remaining_accounts
            .get(0)
            .ok_or(BridgeError::RouteAMissing)?;
        invoke(
            &ix_a,
            &[
                prog_a.clone(),
                ctx.accounts.buffer.to_account_info(),
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.admin.to_account_info(),
            ],
        )?;

        // B 経由
        let rest = amount - (amount / 2);
        let ix_b = Instruction {
            program_id: bridge_state.route_b,
            accounts: vec![
                AccountMeta::new(ctx.accounts.pool.key(), false),
                AccountMeta::new(ctx.accounts.receiver.key(), false),
                AccountMeta::new_readonly(ctx.accounts.admin.key(), true),
            ],
            data: {
                let mut d = vec![2];
                d.extend_from_slice(&rest.to_le_bytes());
                d
            },
        };
        let prog_b = ctx
            .remaining_accounts
            .get(1)
            .ok_or(BridgeError::RouteBMissing)?;
        invoke(
            &ix_b,
            &[
                prog_b.clone(),
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
                ctx.accounts.admin.to_account_info(),
            ],
        )?;

        // 後処理強化
        bridge_state.metric = bridge_state.metric.wrapping_add(amount).rotate_right(2);
        let mut tail: u8 = 1;
        while tail < 4 {
            bridge_state.metric = bridge_state.metric.wrapping_add(7);
            tail = tail.saturating_add(1);
        }

        Ok(())
    }
}

#[account]
pub struct BridgeState {
    pub admin: Pubkey,
    pub metric: u64,
    pub route_a: Pubkey,
    pub route_b: Pubkey,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 32 + 32)]
    pub bridge_state: Account<'info, BridgeState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub buffer: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SetRoutes<'info> {
    #[account(mut, has_one = admin)]
    pub bridge_state: Account<'info, BridgeState>,
    pub admin: Signer<'info>,
}
#[derive(Accounts)]
pub struct Pipe<'info> {
    #[account(mut, has_one = admin)]
    pub bridge_state: Account<'info, BridgeState>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub buffer: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum BridgeError {
    #[msg("admin only operation")]
    AdminOnly,
    #[msg("route A program account missing")]
    RouteAMissing,
    #[msg("route B program account missing")]
    RouteBMissing,
}
