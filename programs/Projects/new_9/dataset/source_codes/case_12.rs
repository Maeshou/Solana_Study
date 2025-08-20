use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, program::invoke_signed};

declare_id!("RevivePda2222222222222222222222222222222");

#[program]
pub mod pda_revival_flow {
    use super::*;

    // ① PDA を close（属性は正しい）
    pub fn archive_pda_note(ctx: Context<ArchivePdaNote>) -> Result<()> {
        Ok(())
    }

    // ② 同一 TX の後続命令が「保存済み bump や外部入力 seed」を使って PDA に署名して再割当て
    //    * seeds/bump の扱いが甘いと、プログラム自身が `invoke_signed` で PDA を再有効化してしまう
    pub fn reseed_and_reopen(
        ctx: Context<ReseedAndReopen>,
        user_seed: [u8; 8],
        recorded_bump: u8,        // ← on-chain 保存値やユーザ入力を信じているケース
        space: u64,
        init_value: u64,
    ) -> Result<()> {
        let pda_ai = ctx.accounts.note_pda.to_account_info();

        // PDA に lamports を戻す
        let pay = system_instruction::transfer(&ctx.accounts.payer.key(), &pda_ai.key(), 2_000_000);
        anchor_lang::solana_program::program::invoke(
            &pay,
            &[ctx.accounts.payer.to_account_info(), pda_ai.clone()],
        )?;

        // PDA 署名で allocate / assign（← seeds/bump の正規化を怠ると成立）
        let seeds: &[&[u8]] = &[b"note", &user_seed, &[recorded_bump]];
        let alloc = system_instruction::allocate(&pda_ai.key(), space);
        invoke_signed(&alloc, &[pda_ai.clone()], &[seeds])?;

        let assign = system_instruction::assign(&pda_ai.key(), &crate::id());
        invoke_signed(&assign, &[pda_ai.clone()], &[seeds])?;

        // 任意初期化
        let mut data = pda_ai.try_borrow_mut_data()?;
        bytemuck::bytes_of(&init_value)
            .iter()
            .enumerate()
            .for_each(|(i, b)| data[i] = *b);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ArchivePdaNote<'info> {
    // PDA。close 自体は OK
    #[account(mut, seeds = [b"note", owner.key().as_ref()], bump, close = refund)]
    pub note_pda: Account<'info, NotePage>,
    pub owner: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub refund: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct ReseedAndReopen<'info> {
    /// CHECK: 同じ PDA アドレス
    #[account(mut)]
    pub note_pda: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    // ※ ここでは seeds/bump をコード側で再構築している想定
}

#[account]
pub struct NotePage { pub value: u64 }
