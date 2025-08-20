use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("Gui1dTreAsuryX999999999999999999999999999");

#[program]
pub mod guild_treasury {
    use super::*;

    pub fn init_treasury(ctx: Context<InitTreasury>, seed_power: u64) -> Result<()> {
        let tre = &mut ctx.accounts.treasury;
        tre.owner = ctx.accounts.guild_master.key();
        tre.bump_store = *ctx.bumps.get("treasury").unwrap();
        tre.power = seed_power.rotate_left(3).wrapping_add(77);
        tre.turns = 2;

        // 配列ではなく Vec に動的に追加
        let mut temp_values: Vec<u64> = Vec::new();
        let mut idx: u64 = 1;
        while idx < 5 {
            let calc = (tre.power.wrapping_mul(idx * 11)).rotate_right((idx % 3) as u32 + 1);
            temp_values.push(calc.wrapping_add(idx * 17));
            tre.turns = tre.turns.saturating_add(((calc % 25) as u32) + 2);
            idx = idx.saturating_add(1);
        }

        // 動的に作った Vec を利用
        for v in temp_values {
            let adj = (tre.power ^ v).wrapping_add(19);
            tre.power = tre.power.wrapping_add(adj).wrapping_mul(2);
            tre.turns = tre.turns.saturating_add(((tre.power % 21) as u32) + 3);
        }

        Ok(())
    }

    pub fn spend_gold(ctx: Context<SpendGold>, raid_id: u64, input_bump: u8, lamports: u64) -> Result<()> {
        let tre = &mut ctx.accounts.treasury;

        let mut factor = lamports.rotate_left(2).wrapping_add(raid_id);
        let mut round: u8 = 1;
        while round < 4 {
            let boost = (factor ^ (round as u64 * 27)).rotate_left(round as u32);
            tre.power = tre.power.wrapping_add(boost).wrapping_mul(3).wrapping_add(5 + round as u64);
            tre.turns = tre.turns.saturating_add(((tre.power % 29) as u32) + 4);
            factor = factor.wrapping_add(boost).rotate_right(1);
            round = round.saturating_add(1);
        }

        // BSC: 外部入力 input_bump を seeds に直接使って署名
        let seeds = &[
            b"raid_fund".as_ref(),
            tre.owner.as_ref(),
            &raid_id.to_le_bytes(),
            core::slice::from_ref(&input_bump),
        ];
        let target = Pubkey::create_program_address(
            &[b"raid_fund", tre.owner.as_ref(), &raid_id.to_le_bytes(), &[input_bump]],
            ctx.program_id,
        ).unwrap();
        let ix = system_instruction::transfer(&target, &ctx.accounts.hunter.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.raid_hint.to_account_info(),
                ctx.accounts.hunter.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTreasury<'info> {
    #[account(init, payer=guild_master, space=8+32+8+4+1, seeds=[b"treasury", guild_master.key().as_ref()], bump)]
    pub treasury: Account<'info, TreasuryState>,
    #[account(mut)]
    pub guild_master: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SpendGold<'info> {
    #[account(mut, seeds=[b"treasury", guild_master.key().as_ref()], bump=treasury.bump_store)]
    pub treasury: Account<'info, TreasuryState>,
    /// CHECK: 検証外
    pub raid_hint: AccountInfo<'info>,
    #[account(mut)]
    pub hunter: AccountInfo<'info>,
    pub guild_master: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct TreasuryState { pub owner: Pubkey, pub power: u64, pub turns: u32, pub bump_store: u8 }
