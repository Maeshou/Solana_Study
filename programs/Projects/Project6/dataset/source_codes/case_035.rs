// lib.rs

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod type_cosplay_secure {
    use super::*;

    // ユーザープロファイルを作成する命令
    pub fn create_user_profile(ctx: Context<CreateUserProfile>, name: String) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = *ctx.accounts.authority.key;
        user_profile.name = name;
        Ok(())
    }

    // ユーザープロファイルの名前を更新する命令
    // ★ 安全なポイント: Context<UpdateUserProfile> により、渡された user_profile アカウントが
    //                  本当に UserProfile 型であるか、Anchorがディスクリミネーターを検証してくれる。
    pub fn update_user_name(ctx: Context<UpdateUserProfile>, new_name: String) -> Result<()> {
        ctx.accounts.user_profile.name = new_name;
        msg!("User profile name updated to: {}", ctx.accounts.user_profile.name);
        Ok(())
    }

    // Treasuryアカウントを作成する命令
    pub fn create_treasury(ctx: Context<CreateTreasury>, initial_balance: u64) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.authority = *ctx.accounts.authority.key;
        treasury.balance = initial_balance;
        Ok(())
    }
}

// === アカウントの定義 ===

// ユーザープロファイルのアカウント
#[account]
pub struct UserProfile {
    pub authority: Pubkey, // 権限者 (32 bytes)
    pub name: String,      // 名前 (4 + n bytes)
}

// 資金庫のアカウント
#[account]
pub struct Treasury {
    pub authority: Pubkey, // 権限者 (32 bytes)
    pub balance: u64,      // 残高 (8 bytes)
}


// === 命令のコンテキスト定義 ===

#[derive(Accounts)]
pub struct CreateUserProfile<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 4 + 50)] // 8バイトのディスクリミネーター空間を確保
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateUserProfile<'info> {
    // ★ 安全なポイント: user_profile.authority == authority.key() の制約に加え、
    //                  このアカウントのディスクリミネーターが UserProfile のものであることを暗黙的にチェック
    #[account(mut, has_one = authority)]
    pub user_profile: Account<'info, UserProfile>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreateTreasury<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8)] // 8バイトのディスクリミネーター空間を確保
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}