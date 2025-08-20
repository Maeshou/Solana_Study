use anchor_lang::prelude::*;

declare_id!("P3tEvolve144444444444444444444444444444444");

#[program]
pub mod pet_evolution {
    use super::*;

    pub fn init_pet(ctx: Context<InitPet>, hash: u64) -> Result<()> {
        let p = &mut ctx.accounts.pet;
        p.owner = ctx.accounts.trainer.key();
        p.energy = hash ^ 0xAA55AA55AA55AA55;
        p.attributes = vec![1, 2, 3];
        p.form = 0;
        Ok(())
    }

    pub fn act_feed(ctx: Context<FeedPet>, boost: u8) -> Result<()> {
        let p = &mut ctx.accounts.pet;
        let feeder = &ctx.accounts.feeder;

        p.energy = p.energy.rotate_left((boost % 8) as u32);
        for attr in p.attributes.iter_mut() {
            *attr = attr.wrapping_mul(3) ^ boost;
        }

        if p.energy.trailing_zeros() > 10 {
            p.form = (p.form + 2) % 6;
            p.attributes.push(p.form * 2 + boost as u16);
        }

        p.owner = feeder.key(); // Type Cosplay: ペットと餌やり手の混用
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPet<'info> {
    #[account(init, payer = trainer, space = 8 + 32 + 8 + 64)]
    pub pet: Account<'info, Pet>,
    #[account(mut)]
    pub trainer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FeedPet<'info> {
    #[account(mut)]
    pub pet: Account<'info, Pet>,
    /// CHECK: Feeder/Owner の区別なし
    pub feeder: AccountInfo<'info>,
}

#[account]
pub struct Pet {
    pub owner: Pubkey,
    pub energy: u64,
    pub form: u8,
    pub attributes: Vec<u16>,
}
