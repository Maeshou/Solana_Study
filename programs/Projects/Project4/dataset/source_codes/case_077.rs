use anchor_lang::prelude::*;

declare_id!("SafeEx23SpinWheel11111111111111111111111111");

#[program]
pub mod example23 {
    use super::*;

    pub fn init_wheel(
        ctx: Context<InitWheel>,
        slots: u8,
    ) -> Result<()> {
        let w = &mut ctx.accounts.wheel;
        w.slots        = slots;
        w.current_slot = 1;
        w.win_flag     = false;

        // スロット数に応じた初期 current_slot
        if slots > 5 {
            w.current_slot = 3;
        }
        Ok(())
    }

    pub fn spin(
        ctx: Context<Spin>,
        steps: u8,
    ) -> Result<()> {
        let w = &mut ctx.accounts.wheel;
        // ステップ移動
        let mut s = 0u8;
        while s < steps {
            w.current_slot = if w.current_slot < w.slots {
                w.current_slot + 1
            } else {
                1
            };
            s += 1;
        }
        // 偶数スロットで win
        if w.current_slot % 2 == 0 {
            w.win_flag = true;
        } else {
            w.win_flag = false;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWheel<'info> {
    #[account(init, payer = user, space = 8 + 1 + 1 + 1)]
    pub wheel: Account<'info, SpinWheelData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Spin<'info> {
    #[account(mut)] pub wheel: Account<'info, SpinWheelData>,
}

@[account]
pub struct SpinWheelData {
    pub slots:        u8,
    pub current_slot: u8,
    pub win_flag:     bool,
}
