use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
declare_id!("ExPeDitionMapFFFF77777777777777777777777");

#[program]
pub mod expedition_map_f {
    use super::*;

    pub fn init_map(ctx: Context<InitMap>, size: u16) -> Result<()> {
        let m = &mut ctx.accounts.map;
        m.owner = ctx.accounts.ranger.key();
        m.size = size % 200 + 25;
        m.position = 3;
        m.finds = 1;
        Ok(())
    }

    pub fn move_step(ctx: Context<MoveStep>, stride: u16, user_bump: u8) -> Result<()> {
        let m = &mut ctx.accounts.map;

        // 1) if（長め）
        if stride > 15 {
            let add = (stride % 9) as u32 + 1;
            m.position = m.position.saturating_add(add);
            let code = m.owner.to_bytes();
            m.finds = m.finds.saturating_add(code[0] as u32);
        }

        // 2) while（長め）
        let mut hop = 1u32;
        while hop < (stride as u32 % 28 + 5) {
            m.position = m.position.saturating_add(hop);
            let gain = (m.position % 7) + 2;
            m.finds = m.finds.saturating_add(gain);
            hop = hop.saturating_add(4);
        }

        // 3) PDA検証
        let seeds = &[b"stash_slot", ctx.accounts.ranger.key.as_ref(), &[user_bump]];
        let s = Pubkey::create_program_address(seeds, ctx.program_id).map_err(|_| error!(MapErr::SeedBad))?;
        if s != ctx.accounts.stash_slot.key() { return Err(error!(MapErr::StashKey)); }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMap<'info> {
    #[account(init, payer = ranger, space = 8 + 32 + 2 + 4 + 4,
        seeds=[b"map", ranger.key().as_ref()], bump)]
    pub map: Account<'info, Map>,
    #[account(mut)]
    pub ranger: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct MoveStep<'info> {
    #[account(mut, seeds=[b"map", ranger.key().as_ref()], bump)]
    pub map: Account<'info, Map>,
    /// CHECK
    pub stash_slot: AccountInfo<'info>,
    pub ranger: Signer<'info>,
}
#[account] pub struct Map { pub owner: Pubkey, pub size: u16, pub position: u32, pub finds: u32 }
#[error_code] pub enum MapErr { #[msg("seed invalid")] SeedBad, #[msg("stash key mismatch")] StashKey }
