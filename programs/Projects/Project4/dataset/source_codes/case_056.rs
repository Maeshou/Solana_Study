use anchor_lang::prelude::*;

declare_id!("SafeEx09XXXXXXX9999999999999999999999999999");

#[program]
pub mod example9 {
    use super::*;

    pub fn init_session(
        ctx: Context<InitSession>,
        max_players: u8,
    ) -> Result<()> {
        let sess = &mut ctx.accounts.session;
        sess.max = max_players;
        sess.joined = 0;

        let pc = &mut ctx.accounts.playercount;
        pc.count = 0;

        let ff = &mut ctx.accounts.fullflag;
        ff.full = false;
        Ok(())
    }

    pub fn join(
        ctx: Context<Join>,
    ) -> Result<()> {
        let sess = &mut ctx.accounts.session;
        if sess.joined < sess.max {
            sess.joined += 1;
            ctx.accounts.playercount.count += 1;
        }
        ctx.accounts.fullflag.full = sess.joined == sess.max;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSession<'info> {
    #[account(init, payer = user, space = 8 + 1 + 1)]
    pub session: Account<'info, SessionData>,
    #[account(init, payer = user, space = 8 + 1)]
    pub playercount: Account<'info, PlayerCountData>,
    #[account(init, payer = user, space = 8 + 1)]
    pub fullflag: Account<'info, FullFlagData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Join<'info> {
    #[account(mut)] pub session: Account<'info, SessionData>,
    #[account(mut)] pub playercount: Account<'info, PlayerCountData>,
    #[account(mut)] pub fullflag: Account<'info, FullFlagData>,
}

#[account]
pub struct SessionData {
    pub max: u8,
    pub joined: u8,
}

#[account]
pub struct PlayerCountData {
    pub count: u8,
}

#[account]
pub struct FullFlagData {
    pub full: bool,
}
