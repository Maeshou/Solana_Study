use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke, memo};
use memo::id as memo_program_id;

declare_id!("SaFeMeMoCPI1111111111111111111111111111");

#[program]
pub mod safe_memo {
    use super::*;

    pub fn write_memo(ctx: Context<WriteMemo>, text: String) -> Result<()> {
        // メモプログラムIDを固定して採用
        let pid = memo_program_id();

        // 追加のガード（任意）：もし受け口をAccountInfoで受けているなら一致検証
        if let Some(prog) = ctx.accounts.memo_program.as_ref() {
            require_keys_eq!(prog.key(), pid);
        }

        let ix = Instruction {
            program_id: pid,
            accounts: vec![], // Memoは口座不要
            data: text.as_bytes().to_vec(),
        };
        invoke(&ix, &[])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WriteMemo<'info> {
    // 任意: メモプログラム口座を受けても良いが、使う前に鍵検証すること
    pub memo_program: Option<AccountInfo<'info>>,
}
