use anchor_lang::prelude::*;
use anchor_lang::sysvar::clock::Clock;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpD4E3F2G1H0J9K8L7M6N5O4P3Q2");

#[program]
pub mod doc_registry {
    use super::*;

    /// ドキュメントレジストリアカウントを初期化
    pub fn initialize_registry(
        ctx: Context<InitializeRegistry>,
        bump: u8,
    ) -> ProgramResult {
        let registry = &mut ctx.accounts.registry;
        registry.owner = *ctx.accounts.user.key;
        registry.bump = bump;
        registry.entries = Vec::new();
        Ok(())
    }

    /// ドキュメントIDとタイムスタンプを登録
    pub fn register_doc(
        ctx: Context<RegisterDoc>,
        doc_id: [u8; 32],
    ) -> ProgramResult {
        let registry = &mut ctx.accounts.registry;
        let timestamp = Clock::get()?.unix_timestamp;
        registry.entries.push((doc_id, timestamp));
        Ok(())
    }

    /// 全エントリをクリア
    pub fn clear_registry(
        ctx: Context<ClearRegistry>,
    ) -> ProgramResult {
        let registry = &mut ctx.accounts.registry;
        registry.entries.clear();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeRegistry<'info> {
    #[account(
        init,
        seeds = [b"registry", user.key().as_ref()],
        bump = bump,
        payer = user,
        space = 8 + 32 + 1 + 4 + 100 * (32 + 8), // owner + bump + Vec len + max 100 entries
    )]
    pub registry: Account<'info, DocRegistry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RegisterDoc<'info> {
    #[account(
        mut,
        seeds = [b"registry", registry.owner.as_ref()],
        bump = registry.bump,
        has_one = owner,
    )]
    pub registry: Account<'info, DocRegistry>,
    /// 登録操作を行う所有者
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClearRegistry<'info> {
    #[account(
        mut,
        seeds = [b"registry", registry.owner.as_ref()],
        bump = registry.bump,
        has_one = owner,
    )]
    pub registry: Account<'info, DocRegistry>,
    /// クリア操作を行う所有者
    pub owner: Signer<'info>,
}

#[account]
pub struct DocRegistry {
    /// アカウント所有者
    pub owner: Pubkey,
    /// PDA生成用バンプ
    pub bump: u8,
    /// 登録された(ドキュメントID, タイムスタンプ)リスト
    pub entries: Vec<([u8; 32], i64)>,
}

// 分岐やループを含まず、ドキュメント履歴を安全に管理する実装です。
