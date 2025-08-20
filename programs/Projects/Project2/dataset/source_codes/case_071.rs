use anchor_lang::prelude::*;

declare_id!("RcdInit11111111111111111111111111111111111");

#[program]
pub mod record_init {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        let rec = &mut ctx.accounts.rec;
        // 初期値設定
        rec.value = data;
        rec.creator = ctx.accounts.user.key();
        rec.count = 0;
        // イベント発行
        emit!(DataInitialized {
            initializer: ctx.accounts.user.key(),
            data
        });
        Ok(())
    }

    pub fn update_data(ctx: Context<UpdateData>, new_data: u64) -> Result<()> {
        let rec = &mut ctx.accounts.rec;
        // 呼び出し者のみが変更できるように保証
        require_keys_eq!(rec.creator, ctx.accounts.user.key());
        // 値を更新し、更新回数を記録
        rec.value = new_data;
        rec.count = rec.count.checked_add(1).unwrap();
        emit!(DataUpdated {
            updater: ctx.accounts.user.key(),
            new_data,
            times: rec.count
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32 + 8)]
    pub rec: Account<'info, DataRecord>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub rec: Account<'info, DataRecord>,
    pub user: Signer<'info>,
}

#[account]
pub struct DataRecord {
    pub value: u64,
    pub creator: Pubkey,
    pub count: u64,
}

#[event]
pub struct DataInitialized {
    pub initializer: Pubkey,
    pub data: u64,
}

#[event]
pub struct DataUpdated {
    pub updater: Pubkey,
    pub new_data: u64,
    pub times: u64,
}
