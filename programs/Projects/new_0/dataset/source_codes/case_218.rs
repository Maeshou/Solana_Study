use anchor_lang::prelude::*;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpX1z2y3w4v5u6t7s8r9q0p1o2n3");

#[program]
pub mod message_board {
    use super::*;

    /// ボードアカウントを初期化
    pub fn initialize_board(
        ctx: Context<InitializeBoard>,
        bump: u8,
    ) -> ProgramResult {
        let board = &mut ctx.accounts.board;
        board.owner = *ctx.accounts.owner.key;
        board.bump = bump;
        board.message = String::new();
        Ok(())
    }

    /// メッセージを投稿（最大280文字）
    pub fn post_message(
        ctx: Context<PostMessage>,
        bump: u8,
        new_message: String,
    ) -> ProgramResult {
        require!(new_message.len() <= 280, ErrorCode::MessageTooLong);
        let board = &mut ctx.accounts.board;
        board.message = new_message;
        Ok(())
    }

    /// メッセージをクリア
    pub fn clear_message(
        ctx: Context<ClearMessage>,
    ) -> ProgramResult {
        let board = &mut ctx.accounts.board;
        board.message.clear();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeBoard<'info> {
    #[account(
        init,
        seeds = [b"board", owner.key().as_ref()],
        bump = bump,
        payer = owner,
        space = 8 + 32 + 1 + 4 + 280,
    )]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct PostMessage<'info> {
    #[account(
        mut,
        seeds = [b"board", board.owner.as_ref()],
        bump = bump,
        has_one = owner,
    )]
    pub board: Account<'info, Board>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClearMessage<'info> {
    #[account(
        mut,
        seeds = [b"board", board.owner.as_ref()],
        bump = board.bump,
        has_one = owner,
    )]
    pub board: Account<'info, Board>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Board {
    pub owner: Pubkey,
    pub bump: u8,
    pub message: String,
}

#[error]
pub enum ErrorCode {
    #[msg("Message too long.")]
    MessageTooLong,
}
