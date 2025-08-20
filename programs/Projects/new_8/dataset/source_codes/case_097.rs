use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("FiNdVsUserBump222222222222222222222222222");

#[program]
pub mod find_vs_user_bump_drift {
    use super::*;

    pub fn init_cell(ctx: Context<InitCell>, tag: u64) -> Result<()> {
        let c = &mut ctx.accounts.cell;
        c.owner = ctx.accounts.admin.key();
        c.tag = tag;
        c.bump_store = *ctx.bumps.get("cell").unwrap();
        Ok(())
    }

    // 典型パターン2: find_program_address で正規 bump を得るが、invoke では user_bump を使う
    pub fn drain_cell(ctx: Context<DrainCell>, user_bump: u8, lamports: u64) -> Result<()> {
        let c = &ctx.accounts.cell;

        let (correct, _good_bump) = Pubkey::find_program_address(
            &[b"cell", c.owner.as_ref(), &c.tag.to_le_bytes()],
            ctx.program_id,
        );

        let seeds = &[
            b"cell".as_ref(),
            c.owner.as_ref(),
            &c.tag.to_le_bytes(),
            core::slice::from_ref(&user_bump), // ← find の戻りではなく外部入力 bump
        ];

        let ix = system_instruction::transfer(&correct, &ctx.accounts.sweeper.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.cell_hint.to_account_info(),
                ctx.accounts.sweeper.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds], // ← 不一致の可能性
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCell<'info> {
    #[account(init, payer=admin, space=8+32+8+1, seeds=[b"cell", admin.key().as_ref(), tag.to_le_bytes().as_ref()], bump)]
    pub cell: Account<'info, CellState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DrainCell<'info> {
    #[account(mut, seeds=[b"cell", admin.key().as_ref(), cell.tag.to_le_bytes().as_ref()], bump=cell.bump_store)]
    pub cell: Account<'info, CellState>,
    /// CHECK
    pub cell_hint: AccountInfo<'info>,
    #[account(mut)]
    pub sweeper: AccountInfo<'info>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct CellState { pub owner: Pubkey, pub tag: u64, pub bump_store: u8 }
