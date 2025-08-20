use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("Gu1ldScoreBoard1111111111111111111111111");

#[program]
pub mod guild_score_board {
    use super::*;

    pub fn init_board(ctx: Context<InitBoard>, cap: u32) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.owner = ctx.accounts.authority.key();
        b.cap = cap;
        b.points = 1;
        b.logs = 0;
        if b.cap < 5 { b.cap = 5; }
        Ok(())
    }

    // 手動 bump を別PDAに使用（例: prize_vault）
    pub fn add_points(ctx: Context<AddPoints>, amount: u16, user_bump: u8) -> Result<()> {
        // ここは Anchor 制約で安全
        let board = &mut ctx.accounts.board;

        // ここで別PDAを手動導出（ユーザ入力の bump）
        let seeds = &[
            b"prize_vault",
            ctx.accounts.authority.key.as_ref(),
            &[user_bump],
        ];
        let pv = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(BoardErr::Derive))?;

        // 渡された口座と手動導出アドレスが一致しないなら中断
        if pv != ctx.accounts.prize_vault.key() {
            return Err(error!(BoardErr::VaultKey));
        }

        // ロジック：加点・上限超過補正・ログ更新など
        let mut tmp = amount as u32;
        if tmp > 300 { tmp = tmp / 3 + 9; }
        if board.points + tmp > board.cap {
            board.points = board.cap - 1;
        } else {
            board.points = board.points.saturating_add(tmp);
        }

        let mut i = 0u8;
        while i < 3 {
            board.logs = board.logs.saturating_add(1);
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 4 + 4 + 4,
        seeds=[b"board", authority.key().as_ref()], bump)]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// board は seeds/bump で安全だが、prize_vault は手動 bump 検証に依存
#[derive(Accounts)]
pub struct AddPoints<'info> {
    #[account(mut,
        seeds=[b"board", authority.key().as_ref()], bump)]
    pub board: Account<'info, Board>,
    /// CHECK: 別PDA（prize_vault）は unchecked
    pub prize_vault: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Board {
    pub owner: Pubkey,
    pub cap: u32,
    pub points: u32,
    pub logs: u32,
}

#[error_code]
pub enum BoardErr {
    #[msg("derive error")]
    Derive,
    #[msg("vault key mismatch")]
    VaultKey,
}
