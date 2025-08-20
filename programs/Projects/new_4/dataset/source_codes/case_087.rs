use anchor_lang::prelude::*;

declare_id!("Repertory16Lottery11111111111111111111111111");

#[program]
pub mod lottery {
    use super::*;

    // ロットを初期化
    pub fn init_lottery(ctx: Context<InitLottery>, max_ticket: u32) -> Result<()> {
        let l = &mut ctx.accounts.lottery;
        l.max_ticket = max_ticket;
        l.drawn = false;
        Ok(())
    }

    // 抽選を行い当選IDを記録
    pub fn draw(ctx: Context<Draw>, rand_numbers: Vec<u32>) -> Result<()> {
        let l = &mut ctx.accounts.lottery;         // ← initなし：既存参照
        if !l.drawn {
            let mut winner = 0u32;
            for &r in rand_numbers.iter() {
                if r < l.max_ticket {
                    winner = r;
                    break;
                }
            }
            l.winner = winner;
            l.drawn = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLottery<'info> {
    #[account(init, payer = user, space = 8 + 4 + 1 + 4)]
    pub lottery: Account<'info, LotteryData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Draw<'info> {
    pub lottery: Account<'info, LotteryData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LotteryData {
    pub max_ticket: u32,
    pub drawn: bool,
    pub winner: u32,
}
