use anchor_lang::prelude::*;

declare_id!("OwnChk1111111111111111111111111111111111");

#[program]
pub mod owner_check_fixed {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_value: u64) -> Result<()> {
        let data = &mut ctx.accounts.data_account;
        // ここで authority.key() を保存
        data.owner = ctx.accounts.authority.key();
        data.value = initial_value;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, new_value: u64) -> Result<()> {
        // ここで自動的に
        //   (1) data_account.owner == authority.key()
        //   (2) authority は実際に署名している
        // の２点が担保されたうえで処理される
        let data = &mut ctx.accounts.data_account;
        data.value = new_value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub data_account: Account<'info, DataAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// 更新時には必ず authority を Signer として渡し、
/// data_account.owner が authority に紐づくことを保証
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, has_one = owner)]
    pub data_account: Account<'info, DataAccount>,

    /// data_account.owner の Pubkey と一致し、
    /// 実際にトランザクションを署名していることを検証
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    /// initialize 時にセットした「このアカウントを操作できるユーザー」
    pub owner: Pubkey,
    pub value: u64,
}
