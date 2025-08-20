use anchor_lang::prelude::*;

declare_id!("GAMElogic111111111111111111111111111111111");

#[program]
pub mod complex_constraints {
    use super::*;
    pub fn mint_item(ctx: Context<MintItem>) -> Result<()> {
        // ミント処理
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintItem<'info> {
    #[account(
        mut,
        seeds = [b"game_state"],
        bump = game_state.bump,
        // 追加の制約1: ゲームがアクティブであること
        constraint = game_state.is_active @ GameError::GameNotActive,
        // 追加の制約2: ミント上限に達していないこと
        constraint = game_state.mints < game_state.max_mints @ GameError::MintLimitExceeded
    )]
    pub game_state: Account<'info, GameState>,

    // ユーザーが持つNFTが正しいコレクションに属することの検証は
    // CPIや別の方法で行うが、ここではアカウントの制約に焦点を当てる
    pub user: Signer<'info>,
    // ...
}

#[account]
pub struct GameState {
    pub is_active: bool,
    pub mints: u64,
    pub max_mints: u64,
    pub bump: u8,
}

#[error_code]
pub enum GameError {
    GameNotActive,
    MintLimitExceeded,
}