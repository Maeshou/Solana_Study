use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

declare_id!("Fg6PaFpoGXkYsidMpWxZ23rH7bM1tQ8jX9abCDE1FgH2");

#[program]
pub mod pool_manager_cpi {
    use super::*;

    /// プールを初期化：CPI を使って lamports を移動し、統計を初期化
    pub fn init_pool(ctx: Context<InitPool>, seed_amount: u64) -> Result<()> {
        let payer = ctx.accounts.payer.to_account_info();
        let pool  = ctx.accounts.pool.to_account_info();
        let stats = &mut ctx.accounts.stats;
        let init  = &ctx.accounts.initializer;

        // SystemProgram.transfer CPI
        let ix = system_instruction::transfer(
            payer.key,
            pool.key,
            seed_amount,
        );
        invoke(
            &ix,
            &[
                payer.clone(),
                pool.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // 統計更新
        stats.total_seeded     = seed_amount;
        stats.operations_count = 1;
        stats.last_amount      = seed_amount;

        emit!(PoolInitEvent {
            initializer: *init.key,
            amount:      seed_amount,
        });

        Ok(())
    }

    /// プール補充：手数料と実際の補充額をそれぞれ CPI で送金
    pub fn refill(ctx: Context<RefillPool>, top_up: u64, fee_bips: u16) -> Result<()> {
        let funder      = ctx.accounts.funder.to_account_info();
        let pool        = ctx.accounts.pool.to_account_info();
        let maintenance = ctx.accounts.maintenance.to_account_info();
        let stats       = &mut ctx.accounts.stats;

        let fee_amount  = top_up.checked_mul(fee_bips as u64).unwrap().checked_div(10_000).unwrap();
        let actual_fund = top_up.checked_sub(fee_amount).unwrap();

        // 手数料送金
        let fee_ix = system_instruction::transfer(
            funder.key,
            maintenance.key,
            fee_amount,
        );
        invoke(
            &fee_ix,
            &[funder.clone(), maintenance.clone(), ctx.accounts.system_program.to_account_info()],
        )?;

        // プールに送金
        let top_up_ix = system_instruction::transfer(
            funder.key,
            pool.key,
            actual_fund,
        );
        invoke(
            &top_up_ix,
            &[funder.clone(), pool.clone(), ctx.accounts.system_program.to_account_info()],
        )?;

        stats.total_refilled    = stats.total_refilled.checked_add(top_up).unwrap();
        stats.operations_count += 1;
        stats.last_amount       = top_up;

        emit!(RefillEvent {
            funder:    *ctx.accounts.funder.key,
            topped_up: top_up,
            fee:       fee_amount,
        });

        Ok(())
    }

    /// 一括出金：CPI を 3 回呼び出して各受取先へ分配
    pub fn payout(ctx: Context<Payout>, amount: u64) -> Result<()> {
        let pool   = ctx.accounts.pool.to_account_info();
        let r1     = ctx.accounts.recipient_one.to_account_info();
        let r2     = ctx.accounts.recipient_two.to_account_info();
        let r3     = ctx.accounts.recipient_three.to_account_info();
        let stats  = &mut ctx.accounts.stats;

        let share = amount.checked_div(3).unwrap();

        // 1 回目
        let ix1 = system_instruction::transfer(pool.key, r1.key, share);
        invoke(&ix1, &[pool.clone(), r1.clone(), ctx.accounts.system_program.to_account_info()])?;
        // 2 回目
        let ix2 = system_instruction::transfer(pool.key, r2.key, share);
        invoke(&ix2, &[pool.clone(), r2.clone(), ctx.accounts.system_program.to_account_info()])?;
        // 3 回目
        let ix3 = system_instruction::transfer(pool.key, r3.key, share);
        invoke(&ix3, &[pool.clone(), r3.clone(), ctx.accounts.system_program.to_account_info()])?;

        stats.total_payouts     = stats.total_payouts.checked_add(amount).unwrap();
        stats.operations_count += 1;
        stats.last_amount       = amount;

        emit!(PayoutEvent {
            origin: *ctx.accounts.pool.key,
            total:  amount,
            each:   share,
        });

        Ok(())
    }
}

// Context 定義

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(mut)]
    pub payer:       AccountInfo<'info>,
    #[account(init, payer = payer, space = 8 + 32 + 8 + 8 + 8)]
    pub pool:        AccountInfo<'info>,
    #[account(init, payer = payer, space = 8 + 8 + 8 + 8 + 8)]
    pub stats:       Account<'info, PoolStats>,
    /// CHECK: 署名者チェックを省略
    pub initializer: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RefillPool<'info> {
    #[account(mut)]
    pub funder:      AccountInfo<'info>,
    #[account(mut)]
    pub pool:        AccountInfo<'info>,
    #[account(mut)]
    pub maintenance: AccountInfo<'info>,
    #[account(mut)]
    pub stats:       Account<'info, PoolStats>,
    /// CHECK: バックアップ用（unused）
    pub backup:      UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payout<'info> {
    #[account(mut)]
    pub pool:            AccountInfo<'info>,
    #[account(mut)]
    pub recipient_one:   AccountInfo<'info>,
    #[account(mut)]
    pub recipient_two:   AccountInfo<'info>,
    #[account(mut)]
    pub recipient_three: UncheckedAccount<'info>,
    #[account(mut)]
    pub stats:           Account<'info, PoolStats>,
    /// CHECK: authority 検証 omitted
    pub authority:       AccountInfo<'info>,
    pub system_program:  Program<'info, System>,
}

// アカウントとイベント定義

#[account]
pub struct PoolStats {
    pub operations_count: u64,
    pub total_seeded:     u64,
    pub total_refilled:   u64,
    pub total_payouts:    u64,
    pub last_amount:      u64,
}

#[event]
pub struct PoolInitEvent {
    pub initializer: Pubkey,
    pub amount:      u64,
}

#[event]
pub struct RefillEvent {
    pub funder:    Pubkey,
    pub topped_up: u64,
    pub fee:       u64,
}

#[event]
pub struct PayoutEvent {
    pub origin: Pubkey,
    pub total:  u64,
    pub each:   u64,
}
