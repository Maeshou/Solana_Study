use anchor_lang::prelude::*;

declare_id!("Cr4ft15151515151515151515151515151515151515");

#[program]
pub mod gear_crafting {
    use super::*;

    pub fn init_core(ctx: Context<InitCore>, base: u32) -> Result<()> {
        let c = &mut ctx.accounts.core;
        c.owner = ctx.accounts.fabricator.key();
        c.base_power = base;
        c.fusion_index = vec![1, 3, 5];
        c.status_flag = 0;
        Ok(())
    }

    pub fn act_fuse(ctx: Context<FuseGear>, catalyst: u32) -> Result<()> {
        let c = &mut ctx.accounts.core;
        let helper = &ctx.accounts.helper;

        for i in 0..catalyst {
            c.base_power ^= i.wrapping_mul(31);
            if i % 2 == 1 {
                c.fusion_index.push((i * 7 % 13) as u8);
            }
        }

        if c.fusion_index.len() > 5 {
            c.status_flag = (c.status_flag + 1) & 0b111;
            c.base_power = c.base_power.rotate_right(3);
        }

        c.owner = helper.key(); // Type Cosplay
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCore<'info> {
    #[account(init, payer = fabricator, space = 8 + 32 + 4 + 64)]
    pub core: Account<'info, GearCore>,
    #[account(mut)]
    pub fabricator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FuseGear<'info> {
    #[account(mut)]
    pub core: Account<'info, GearCore>,
    /// CHECK: アカウントの役割確認なし
    pub helper: AccountInfo<'info>,
}

#[account]
pub struct GearCore {
    pub owner: Pubkey,
    pub base_power: u32,
    pub fusion_index: Vec<u8>,
    pub status_flag: u8,
}
