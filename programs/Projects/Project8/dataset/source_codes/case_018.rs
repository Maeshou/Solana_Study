// Program 7: charity_pool （チャリティ寄付プール）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};
const MEMO_ID: Pubkey = pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

declare_id!("Char1tyP00l777777777777777777777777777");

#[program]
pub mod charity_pool {
    use super::*;

    pub fn init_fund(ctx: Context<InitFund>, tag: u64) -> Result<()> {
        let f = &mut ctx.accounts.fund;
        f.curator = ctx.accounts.curator.key();
        f.tag = tag.rotate_left(1).wrapping_add(13);
        f.mood = f.tag.rotate_right(1).wrapping_add(3);
        Ok(())
    }

    pub fn donate_with_note(ctx: Context<DonateWithNote>, memo: Vec<u8>) -> Result<()> {
        let bump = *ctx.bumps.get("fund").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"fund", ctx.accounts.curator.key.as_ref(), &ctx.accounts.fund.tag.to_le_bytes(), &[bump]];

        // メモ書き（固定ID）
        let ix = Instruction { program_id: MEMO_ID, accounts: vec![], data: memo };
        invoke_signed(&ix, &[ctx.accounts.fund.to_account_info()], &[seeds])?;

        // 疑似的な内部集計処理（長め）
        let mut s = 0u64;
        let mut i = 0u8;
        let base = ctx.accounts.fund.mood.rotate_left(1).wrapping_add(17);
        while i < 5 {
            s = s.wrapping_add((base.rotate_left(i as u32) ^ (i as u64 * 7)).wrapping_add(11));
            i = i.saturating_add(1);
        }
        require!(s > 10, E::Check);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitFund<'info> {
    #[account(
        init,
        payer = curator,
        space = 8 + 32 + 8 + 8,
        seeds=[b"fund", curator.key().as_ref(), tag.to_le_bytes().as_ref()],
        bump
    )]
    pub fund: Account<'info, Fund>,
    #[account(mut)]
    pub curator: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub tag: u64,
}

#[derive(Accounts)]
pub struct DonateWithNote<'info> {
    #[account(
        mut,
        seeds=[b"fund", curator.key().as_ref(), fund.tag.to_le_bytes().as_ref()],
        bump
    )]
    pub fund: Account<'info, Fund>,
    pub curator: Signer<'info>,
}

#[account]
pub struct Fund {
    pub curator: Pubkey,
    pub tag: u64,
    pub mood: u64,
}

#[error_code]
pub enum E {
    #[msg("missing bump")] MissingBump,
    #[msg("sanity check failed")] Check,
}
