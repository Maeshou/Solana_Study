use anchor_lang::prelude::*;

declare_id!("MultiRole11111111111111111111111111111111");

#[program]
pub mod multi_role_data {
    use super::*;

    /// 初期化：管理者と記録者を登録し、履歴に最初の値を追加
    pub fn init(ctx: Context<InitMulti>, initial: u64) -> Result<()> {
        let rec = &mut ctx.accounts.record;
        rec.value = initial;
        rec.manager = ctx.accounts.manager.key();
        rec.writer  = ctx.accounts.writer.key();
        rec.history = vec![initial];
        emit!(RecordCreated {
            by: rec.manager,
            value: initial
        });
        Ok(())
    }

    /// 追記＆閉鎖：記録者の署名でのみ呼び出せ、最後に残高を管理者に返却してアカウントを無効化
    pub fn append_and_close(ctx: Context<AppendClose>, extra: u64) -> Result<()> {
        let rec = &mut ctx.accounts.record;
        // 記録者のみが呼べることを保証
        require_keys_eq!(rec.writer, ctx.accounts.writer.key());

        // 値を更新して履歴に追加
        let new_val = rec
            .value
            .checked_add(extra)
            .ok_or(ErrorCode::Overflow)?;
        rec.value = new_val;
        rec.history.push(new_val);
        emit!(Appended {
            by: rec.writer,
            value: new_val
        });

        // アカウント閉鎖：残高を管理者に移動し、自アカウントを0に
        let record_ai = ctx.accounts.record.to_account_info();
        let manager_ai = ctx.accounts.manager_lamports.to_account_info();

        **manager_ai.lamports.borrow_mut() += **record_ai.lamports.borrow();
        **record_ai.lamports.borrow_mut() = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMulti<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 32 + 32 + 4 + (8 * 10))]
    pub record: Account<'info, MultiRecord>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub manager: Signer<'info>,
    pub writer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AppendClose<'info> {
    #[account(mut, has_one = manager, has_one = writer)]
    pub record: Account<'info, MultiRecord>,
    pub writer: Signer<'info>,
    /// CHECK: lamports を返却する先
    #[account(mut)]
    pub manager_lamports: AccountInfo<'info>,
}

#[account]
pub struct MultiRecord {
    pub value: u64,
    pub manager: Pubkey,
    pub writer: Pubkey,
    pub history: Vec<u64>,
}

#[event]
pub struct RecordCreated {
    pub by: Pubkey,
    pub value: u64,
}

#[event]
pub struct Appended {
    pub by: Pubkey,
    pub value: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("計算結果が大きすぎます")]
    Overflow,
}
