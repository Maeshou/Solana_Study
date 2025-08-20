use anchor_lang::prelude::*;

declare_id!("SafeEx22Lottery1111111111111111111111111111");

#[program]
pub mod example22 {
    use super::*;

    pub fn init_lottery(
        ctx: Context<InitLottery>,
        tickets: u32,
    ) -> Result<()> {
        let l = &mut ctx.accounts.lottery;
        l.tickets    = tickets;
        l.winners    = 0;
        l.draw_flag  = false;

        // チケット数の平方根を winners に設定
        let mut w = 0u32;
        while (w+1)*(w+1) <= tickets {
            w += 1;
        }
        l.winners = w;
        Ok(())
    }

    pub fn draw(
        ctx: Context<Draw>,
    ) -> Result<()> {
        let l = &mut ctx.accounts.lottery;
        if !l.draw_flag {
            // 当選フラグを反転
            l.draw_flag = true;
        } else {
            l.draw_flag = false;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLottery<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 1)]
    pub lottery: Account<'info, LotteryData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Draw<'info> {
    #[account(mut)] pub lottery: Account<'info, LotteryData>,
}

#[account]
pub struct LotteryData {
    pub tickets:   u32,
    pub winners:   u32,
    pub draw_flag: bool,
}
