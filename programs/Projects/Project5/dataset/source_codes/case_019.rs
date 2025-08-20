// ============================================================================
// 4) Farm Fields（土地耕作）— PDA不使用 + constraint 三連（各アカウント同士≠）
//    防止法: すべて属性constraintで排除（require系未使用）
// ============================================================================
declare_id!("FARM44444444444444444444444444444444");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PlotState { Empty, Planted }

#[program]
pub mod farm_fields {
    use super::*;

    pub fn init_plot(ctx: Context<InitPlot>, fert: u32) -> Result<()> {
        ctx.accounts.plot.holder = ctx.accounts.farmer.key();
        ctx.accounts.plot.fertility = fert;
        ctx.accounts.plot.state = PlotState::Empty;

        ctx.accounts.worker.rank = 1;
        ctx.accounts.worker.stamina = 100;
        ctx.accounts.worker.boots = 1;

        ctx.accounts.tally.days = 0;
        ctx.accounts.tally.yield_hint = 0;
        Ok(())
    }

    pub fn cultivate(ctx: Context<Cultivate>, days: u32) -> Result<()> {
        // ループ
        for _ in 0..days {
            ctx.accounts.worker.stamina = ctx.accounts.worker.stamina.saturating_sub(1);
            ctx.accounts.tally.days = ctx.accounts.tally.days.saturating_add(1);
            ctx.accounts.tally.yield_hint = ctx.accounts.tally.yield_hint.saturating_add(ctx.accounts.plot.fertility as u64);
        }

        // 分岐
        if ctx.accounts.worker.stamina < 50 {
            ctx.accounts.plot.state = PlotState::Empty;
            ctx.accounts.worker.boots = 0;
            ctx.accounts.tally.yield_hint = ctx.accounts.tally.yield_hint.saturating_add(3);
            msg!("tired; pause planting");
        } else {
            ctx.accounts.plot.state = PlotState::Planted;
            ctx.accounts.worker.boots = 1;
            ctx.accounts.tally.yield_hint = ctx.accounts.tally.yield_hint.saturating_add(1);
            msg!("steady cultivation");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPlot<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub plot: Account<'info, Plot>,
    #[account(init, payer = payer, space = 8 + 4 + 4 + 1)]
    pub worker: Account<'info, Worker>,
    #[account(init, payer = payer, space = 8 + 4 + 8)]
    pub tally: Account<'info, HarvestTally>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub farmer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Cultivate<'info> {
    #[account(mut, constraint = plot.key() != worker.key(), error = FarmErr::Same)]
    pub plot: Account<'info, Plot>,
    #[account(mut, constraint = worker.key() != tally.key(), error = FarmErr::Same)]
    pub worker: Account<'info, Worker>,
    #[account(mut, constraint = plot.key() != tally.key(), error = FarmErr::Same)]
    pub tally: Account<'info, HarvestTally>,
}

#[account] pub struct Plot { pub holder: Pubkey, pub fertility: u32, pub state: PlotState }
#[account] pub struct Worker { pub rank: u32, pub stamina: u32, pub boots: u8 }
#[account] pub struct HarvestTally { pub days: u32, pub yield_hint: u64 }

#[error_code] pub enum FarmErr { #[msg("dup")] Same }
