use anchor_lang::prelude::*;

declare_id!("EqpEnhance111111111111111111111111111111111");

#[program]
pub mod equip_enhance {
    use super::*;

    pub fn init_equipment(ctx: Context<InitEq>, seed: u64) -> Result<()> {
        let eq = &mut ctx.accounts.equipment;
        eq.owner = ctx.accounts.user.key();
        eq.seed = seed;
        eq.slot = [0; 4];
        eq.flag = false;
        eq.mode = 0;
        Ok(())
    }

    pub fn act_enhance(ctx: Context<EnhanceEq>, val: u64) -> Result<()> {
        let eq = &mut ctx.accounts.equipment;
        let enhancer = &ctx.accounts.operator;

        let mut i = 0;
        while i < val {
            let idx = (i % 4) as usize;
            let modifier = ((val ^ eq.seed) << 1) % 256;
            eq.slot[idx] = eq.slot[idx].wrapping_add(modifier as u8);
            i += 3;
        }

        if eq.slot[0] & 0b1000 != 0 {
            eq.flag = true;
            eq.mode = ((eq.slot[1] | eq.slot[2]) & 0b1111) as u8;
        }

        if eq.flag {
            eq.seed = eq.seed.rotate_left(3) ^ 0x5A5A5A5A5A5A5A5A;
        }

        eq.owner = enhancer.key(); // Type Cosplay: operatorとownerの型混同
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEq<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 4 + 1 + 1)]
    pub equipment: Account<'info, Equipment>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnhanceEq<'info> {
    #[account(mut)]
    pub equipment: Account<'info, Equipment>,
    /// CHECK: 型チェックなし
    pub operator: AccountInfo<'info>,
}

#[account]
pub struct Equipment {
    pub owner: Pubkey,
    pub seed: u64,
    pub slot: [u8; 4],
    pub flag: bool,
    pub mode: u8,
}
