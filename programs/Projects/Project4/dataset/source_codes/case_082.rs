use anchor_lang::prelude::*;

declare_id!("SafeEx28Switch11111111111111111111111111111");

#[program]
pub mod example28 {
    use super::*;

    pub fn init_switch(
        ctx: Context<InitSwitch>,
        on_count: u32,
    ) -> Result<()> {
        let s = &mut ctx.accounts.switch;
        s.on_count  = on_count;
        s.off_count = 0;
        s.state     = false;

        // 初期状態をトグル回数分設定
        let mut i = 0u32;
        while i < on_count {
            s.state = !s.state;
            i += 1;
        }
        Ok(())
    }

    pub fn toggle(
        ctx: Context<Toggle>,
    ) -> Result<()> {
        let s = &mut ctx.accounts.switch;
        s.state = !s.state;
        if s.state {
            s.on_count = s.on_count.saturating_add(1);
        } else {
            s.off_count = s.off_count.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSwitch<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 1)]
    pub switch: Account<'info, SwitchData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Toggle<'info> {
    #[account(mut)] pub switch: Account<'info, SwitchData>,
}

#[account]
pub struct SwitchData {
    pub on_count:  u32,
    pub off_count: u32,
    pub state:     bool,
}
