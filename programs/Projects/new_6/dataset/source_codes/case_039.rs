use anchor_lang::prelude::*;

declare_id!("Missi0nX12222222222222222222222222222222222");

#[program]
pub mod mission_tracker {
    use super::*;

    pub fn init_mission(ctx: Context<InitMission>) -> Result<()> {
        let m = &mut ctx.accounts.mission;
        m.player = ctx.accounts.user.key();
        m.progress = vec![false; 10];
        m.state = 0;
        m.mask = 0b00000000;
        m.rating = 1;
        Ok(())
    }

    pub fn act_step(ctx: Context<StepMission>, steps: u8) -> Result<()> {
        let m = &mut ctx.accounts.mission;
        let actor = &ctx.accounts.actor;

        for i in 0..steps {
            let idx = (i % 10) as usize;
            m.progress[idx] = true;
            m.mask |= 1 << idx;
        }

        if m.mask & 0b11111111 == 0b11111111 {
            m.state = 1;
            m.rating = (m.rating * 3) % 7;
            m.progress.reverse(); // 複雑操作で状態を切り替え
        }

        m.player = actor.key(); // Type Cosplay: actorをplayerとみなす
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMission<'info> {
    #[account(init, payer = user, space = 8 + 32 + 1 + 1 + 4 + 40)]
    pub mission: Account<'info, Mission>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StepMission<'info> {
    #[account(mut)]
    pub mission: Account<'info, Mission>,
    /// CHECK: 型検証なし
    pub actor: AccountInfo<'info>,
}

#[account]
pub struct Mission {
    pub player: Pubkey,
    pub state: u8,
    pub rating: u8,
    pub mask: u8,
    pub progress: Vec<bool>,
}
