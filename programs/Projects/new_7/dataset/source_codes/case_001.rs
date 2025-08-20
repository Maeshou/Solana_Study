// 1) token_transfer_passthrough: SPL Token 風の transfer を任意 program_id へ
use anchor_lang::prelude::*;
use anchor_spl::token::spl_token;
use solana_program::program::invoke;

declare_id!("ArbCpiA111111111111111111111111111111111");

#[program]
pub mod token_transfer_passthrough {
    use super::*;

    pub fn move_with_note(ctx: Context<MoveWithNote>, amount: u64, note: String) -> Result<()> {
        // ちょっとした前処理：簡易メモと閾値チェック
        let meta = &mut ctx.accounts.metadata;
        if meta.transfers < 5 {
            meta.last_note_len = note.len() as u32;
        }
        meta.transfers += 1;

        // 検証なし program_id で transfer を作成
        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),              // ← 検証なし
            ctx.accounts.source.key(),
            ctx.accounts.destination.key(),
            ctx.accounts.authority.key(),
            &[],
            amount,
        )?;

        // 周辺アカウントを渡して invoke
        invoke(
            &ix,
            &[
                ctx.accounts.source.to_account_info(),
                ctx.accounts.destination.to_account_info(),
                ctx.accounts.authority.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct Meta {
    pub transfers: u32,
    pub last_note_len: u32,
}

#[derive(Accounts)]
pub struct MoveWithNote<'info> {
    /// CHECK: 検証なし
    #[account(mut)]
    pub source: UncheckedAccount<'info>,
    /// CHECK: 検証なし
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,
    /// CHECK: 検証なし
    pub authority: UncheckedAccount<'info>,
    /// CHECK: 検証なし（本来は spl_token::ID などに固定すべき）
    pub token_program: UncheckedAccount<'info>,
    #[account(init_if_needed, payer = payer, space = 8 + 8)]
    pub metadata: Account<'info, Meta>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
