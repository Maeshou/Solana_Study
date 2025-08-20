// 10) artifact_assembly_mini: アーティファクト組立の軽量版
use anchor_lang::prelude::*;
declare_id!("ArTiFaCtAsSeMbLyMiNi111111111111111111");

#[program]
pub mod artifact_assembly_mini {
    use super::*;

    pub fn init_assembly(ctx: Context<InitAssembly>) -> Result<()> {
        let a = &mut ctx.accounts.assembly;
        a.engineer = ctx.accounts.engineer.key();
        a.rank = 1;
        a.quality = 12;
        Ok(())
    }

    pub fn assemble(ctx: Context<Assemble>, parts: u32, scheme_index: u8) -> Result<()> {
        let a = &mut ctx.accounts.assembly;

        if a.rank < 10 {
            let mut i = 0u8;
            while i < 3 {
                a.rank = a.rank.saturating_add(((i as u32) << 1).saturating_add(2));
                i = i.saturating_add(1);
            }
        }

        let coef: [u32; 3] = [5, 7, 11];
        let idx = if scheme_index > 2 { 2 } else { scheme_index } as usize;
        let mut score = parts.saturating_mul(coef[idx]);

        let mut r = 0u8;
        while r < 4 {
            score = score.rotate_left(1);
            r = r.saturating_add(1);
        }

        a.quality = a.quality.saturating_add(score % 23);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAssembly<'info> {
    #[account(init, payer = engineer, space = 8 + 32 + 4 + 4)]
    pub assembly: Account<'info, AssemblyState>,
    #[account(mut)] pub engineer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Assemble<'info> {
    #[account(mut, has_one = engineer)]
    pub assembly: Account<'info, AssemblyState>,
    pub engineer: Signer<'info>,
}
#[account]
pub struct AssemblyState {
    pub engineer: Pubkey,
    pub rank: u32,
    pub quality: u32,
}
