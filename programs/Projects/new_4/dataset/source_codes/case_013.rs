// 3. レコード管理＋完了／再設定
use anchor_lang::prelude::*;

declare_id!("Rec33333333333333333333333333333333");

#[program]
pub mod reinit_record_v2 {
    use super::*;

    // レコードを作成
    pub fn create_record(
        ctx: Context<CreateRecord>,
        id: u64,
        description: String,
    ) -> Result<()> {
        let record = &mut ctx.accounts.record;
        record.id = id;
        record.description = description;
        record.processed = false;
        Ok(())
    }

    // 完了フラグを立てる
    pub fn complete_record(
        ctx: Context<ModifyRecord>,
    ) -> Result<()> {
        let record = &mut ctx.accounts.record;
        record.processed = true;
        // ログ口座も毎回上書きされる
        let log = &mut ctx.accounts.log_account;
        log.data.push_str("completed;");
        Ok(())
    }

    // 再度レコードをクリア
    pub fn reset_record(
        ctx: Context<ModifyRecord>,
    ) -> Result<()> {
        let record = &mut ctx.accounts.record;
        record.description.clear();
        record.processed = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateRecord<'info> {
    #[account(mut)]
    pub record: Account<'info, RecordData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyRecord<'info> {
    #[account(mut)]
    pub record: Account<'info, RecordData>,
    /// CHECK: ログ用、任意に渡せるまま
    #[account(mut)]
    pub log_account: AccountInfo<'info>,
}

#[account]
pub struct RecordData {
    pub id: u64,
    pub description: String,
    pub processed: bool,
}
