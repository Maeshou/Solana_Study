// ======================================================================
// 10) Robo Lab：組み立てライン（初期化＝行列風ミックスで初期トルク）
// ======================================================================
declare_id!("ROBO10101010101010101010101010101010101010");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LabState { Boot, Assemble, Halt }

#[program]
pub mod robo_lab {
    use super::*;
    use LabState::*;

    pub fn init_lab(ctx: Context<InitLab>, a11: u32, a12: u32, a21: u32, a22: u32) -> Result<()> {
        let lab = &mut ctx.accounts.lab;
        lab.owner = ctx.accounts.chief.key();
        lab.max = (a11 + a22) * 8 + 200;
        lab.state = Boot;

        let arm_a = &mut ctx.accounts.arm_a;
        let arm_b = &mut ctx.accounts.arm_b;
        let board = &mut ctx.accounts.board;

        // 2x2「風」行列積で初期トルクを合成
        let v1 = a11.saturating_mul(3) + a12.saturating_mul(2);
        let v2 = a21.saturating_mul(2) + a22.saturating_mul(3);

        arm_a.lab = lab.key(); arm_a.dock = (a11 & 7) as u8; arm_a.torque = v1 + 13;
        arm_b.lab = lab.key(); arm_b.dock = (a22 & 7) as u8; arm_b.torque = v2 + 11;

        board.lab = lab.key(); board.dock = 9; board.ticks = 0; board.code = (v1 as u64) << 16 | v2 as u64;
        Ok(())
    }

    pub fn assemble(ctx: Context<Assemble>, reps: u32) -> Result<()> {
        let lab = &mut ctx.accounts.lab;
        let a = &mut ctx.accounts.arm_a;
        let b = &mut ctx.accounts.arm_b;
        let brd = &mut ctx.accounts.board;

        for i in 0..reps {
            let mix = ((a.torque ^ b.torque) as u64).wrapping_mul(780291637);
            a.torque = a.torque.checked_add(((mix & 31) as u32) + 2).unwrap_or(u32::MAX);
            b.torque = b.torque.saturating_add((((mix >> 5) & 31) as u32) + 3);
            brd.ticks = brd.ticks.saturating_add(1);
            brd.code ^= mix.rotate_left((i % 13) as u32);
        }

        let total = a.torque + b.torque;
        if total > lab.max {
            lab.state = Halt;
            a.dock ^= 1; b.dock = b.dock.saturating_add(1);
            brd.dock = brd.dock.saturating_add(1);
            msg!("halt: dock tweaks & board move");
        } else {
            lab.state = Assemble;
            a.torque = a.torque.saturating_add(9);
            b.torque = b.torque / 2 + 11;
            brd.code ^= 0x0F0F_F0F0;
            msg!("assemble: adjust torque & code flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLab<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub lab: Account<'info, Lab>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub arm_a: Account<'info, Arm>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub arm_b: Account<'info, Arm>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub board: Account<'info, ControlBoard>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub chief: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Assemble<'info> {
    #[account(mut, has_one=owner)]
    pub lab: Account<'info, Lab>,
    #[account(
        mut,
        has_one=lab,
        constraint = arm_a.dock != arm_b.dock @ RoboErr::Dup
    )]
    pub arm_a: Account<'info, Arm>,
    #[account(
        mut,
        has_one=lab,
        constraint = arm_b.dock != board.dock @ RoboErr::Dup
    )]
    pub arm_b: Account<'info, Arm>,
    #[account(mut, has_one=lab)]
    pub board: Account<'info, ControlBoard>,
    pub chief: Signer<'info>,
}

#[account] pub struct Lab { pub owner: Pubkey, pub max: u32, pub state: LabState }
#[account] pub struct Arm { pub lab: Pubkey, pub dock: u8, pub torque: u32 }
#[account] pub struct ControlBoard { pub lab: Pubkey, pub dock: u8, pub ticks: u64, pub code: u64 }

#[error_code] pub enum RoboErr { #[msg("duplicate mutable account")] Dup }




