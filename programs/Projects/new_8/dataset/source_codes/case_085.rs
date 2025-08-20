use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("AlCheMyX11111111111111111111111111111111");

#[program]
pub mod alchemy_station_router {
    use super::*;

    pub fn init_alchemy(ctx: Context<InitAlchemy>, mana_seed: u64) -> Result<()> {
        let st = &mut ctx.accounts.alchemy;
        st.owner = ctx.accounts.mage.key();
        st.bump_saved = *ctx.bumps.get("alchemy").ok_or(error!(EA::NoBump))?;
        st.mana = mana_seed.rotate_left(2).wrapping_add(101);
        st.steps = 2;

        // if の先行パターン：分岐の中で Vec を組み立ててから for
        if st.mana > 200 {
            let mut values = Vec::new();
            for i in 1..4 {
                let v = (st.mana.wrapping_mul(i * 17)).rotate_right(1).wrapping_add(9 + i as u64);
                values.push(v);
                st.steps = st.steps.saturating_add(((v % 23) as u32) + 2);
            }
            for v in values {
                st.mana = st.mana.wrapping_add(v).wrapping_mul(2).wrapping_add(13);
                st.steps = st.steps.saturating_add(((st.mana % 29) as u32) + 3);
            }
        } else {
            // while を後段に
            let mut phase = 1u8;
            let mut acc = st.mana.rotate_right(1).wrapping_add(33);
            while phase < 4 {
                let mix = (acc ^ (phase as u64 * 23)).rotate_left(1);
                acc = acc.wrapping_add(mix);
                st.mana = st.mana.wrapping_add(mix).wrapping_mul(3).wrapping_add(7 + phase as u64);
                st.steps = st.steps.saturating_add(((st.mana % 31) as u32) + 4);
                phase = phase.saturating_add(1);
            }
        }
        Ok(())
    }

    pub fn spend_elixir(ctx: Context<SpendElixir>, recipe_id: u64, user_bump: u8, lamports: u64) -> Result<()> {
        let st = &mut ctx.accounts.alchemy;

        // fold 的→分岐→for
        let sources: Vec<u64> = (1..5).map(|i| st.mana.wrapping_mul(i * 9)).collect();
        let total = sources.iter().fold(0u64, |acc, v| acc.wrapping_add(*v));
        if total > st.mana {
            for j in 1..4 {
                let inc = (total ^ (j as u64 * 19)).rotate_left(1);
                st.mana = st.mana.wrapping_add(inc).wrapping_mul(2).wrapping_add(15 + j as u64);
                st.steps = st.steps.saturating_add(((st.mana % 27) as u32) + 5);
            }
        }

        // BSC: 外部入力 user_bump を seeds に注入し、未検証PDAへ署名
        let seeds = &[
            b"elixir_cell".as_ref(),
            st.owner.as_ref(),
            &recipe_id.to_le_bytes(),
            core::slice::from_ref(&user_bump),
        ];
        let pda = Pubkey::create_program_address(
            &[b"elixir_cell", st.owner.as_ref(), &recipe_id.to_le_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(EA::SeedCompute))?;
        let ix = system_instruction::transfer(&pda, &ctx.accounts.alchemist.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.elixir_hint.to_account_info(),
                ctx.accounts.alchemist.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAlchemy<'info> {
    #[account(init, payer=mage, space=8+32+8+4+1, seeds=[b"alchemy", mage.key().as_ref()], bump)]
    pub alchemy: Account<'info, AlchemyState>,
    #[account(mut)]
    pub mage: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SpendElixir<'info> {
    #[account(mut, seeds=[b"alchemy", mage.key().as_ref()], bump=alchemy.bump_saved)]
    pub alchemy: Account<'info, AlchemyState>,
    /// CHECK
    pub elixir_hint: AccountInfo<'info>,
    #[account(mut)]
    pub alchemist: AccountInfo<'info>,
    pub mage: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct AlchemyState { pub owner: Pubkey, pub mana: u64, pub steps: u32, pub bump_saved: u8 }
#[error_code] pub enum EA { #[msg("no bump")] NoBump, #[msg("seed compute failed")] SeedCompute }
