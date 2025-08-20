use anchor_lang::prelude::*;

declare_id!("BattleArena77777777777777777777777777777777");

#[program]
pub mod battle_arena {
    use super::*;

    pub fn start_battle(ctx: Context<StartBattle>, intensity: u8) -> Result<()> {
        let arena = &mut ctx.accounts.stage;
        let player = &ctx.accounts.attender;
        let stats_log = &mut ctx.accounts.scoreboard;

        stats_log.data.borrow_mut()[0] = intensity;
        stats_log.data.borrow_mut()[1] = player.key.as_ref()[0];
        stats_log.data.borrow_mut()[2] = arena.key.as_ref()[0];

        if intensity > 200 {
            arena.damage_log += 100;
            stats_log.data.borrow_mut()[3] = 0xAA;
        }

        for i in 0..5 {
            stats_log.data.borrow_mut()[4 + i] = intensity.wrapping_mul(i as u8);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartBattle<'info> {
    #[account(mut)]
    pub stage: AccountInfo<'info>,
    #[account(mut)]
    pub attender: AccountInfo<'info>, // May not be a player
    #[account(mut)]
    pub scoreboard: AccountInfo<'info>,
}
