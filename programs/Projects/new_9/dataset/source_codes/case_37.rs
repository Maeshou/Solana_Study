use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("UseAfterClose1111111111111111111111111111");

const LOG_ID: Pubkey = pubkey!("LoGPrgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"); // 例: 固定ログ用プログラム

#[program]
pub mod use_after_close_example {
    use super::*;

    /// NFTの「入場パス」を破棄しつつ、最後に外部ログへCPIする関数。
    /// pass は #[account(close = collector)] になっており「閉じる」指定。
    /// しかし関数の末尾で pass.to_account_info() を CPI に渡しているため、
    /// “閉じた前提”が崩れ、同一TX内で外部が当該口座を再利用できる余地が生まれる。
    pub fn archive_pass_and_log(ctx: Context<ArchivePassAndLog>, note: u64) -> Result<()> {
        // ここでいろいろ後片付け（メモの書き込みなど）
        let written = note.rotate_left(3).wrapping_add(777);
        ctx.accounts.pass_meta.last_note = written;

        // --- ここが問題の核 ---------------------------------------------------
        // pass は #[account(close = collector)] だが、closeは「命令終了時」に実行される。
        // つまり、今この瞬間は口座が“まだ存在”している。
        //
        // それにもかかわらず、下の CPI で pass の AccountInfo を外部へ渡す設計だと、
        // 外部側が「存在する口座」として扱い、同一TX内で再初期化などを仕掛けられる。
        // （例: 別の命令や外部が前段で資金を戻したうえで allocate/assign など）
        // ---------------------------------------------------------------------

        let metas = vec![
            AccountMeta::new(ctx.accounts.pass.to_account_info().key(), true), // ← “閉じる予定”の口座を可変で渡している
            AccountMeta::new_readonly(ctx.accounts.owner.key(), true),
            AccountMeta::new(ctx.accounts.collector.key(), false),
        ];
        let data = written.to_le_bytes().to_vec();

        // program_id は固定 LOG_ID だとしても、「どのアカウントを渡すか」はこちらの設計次第。
        // “閉じ済み前提”で pass を渡すと、実際にはまだ生きていて外部に触らせてしまう。
        let ix = Instruction { program_id: LOG_ID, accounts: metas, data };
        invoke(
            &ix,
            &[
                ctx.accounts.log_hint.to_account_info(),
                ctx.accounts.pass.to_account_info(),      // ← まだ存在している
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.collector.to_account_info(), // close で受け取り予定
            ],
        )?;

        // ここでOkを返すと drop が走り、#[account(close = collector)] が実行される。
        // しかし CPI は既に完了しており、同一TXの他命令との組み合わせで「復活」を許し得る。
        Ok(())
    }

    /// 比較用：同じことをしたいが、pass の AccountInfo を後段に渡さない版。
    /// 口座の Pubkey やメモ値だけを使い、CPI 側に“元口座そのもの”を触らせない。
    pub fn archive_pass_and_log_safely(ctx: Context<ArchivePassAndLog>, note: u64) -> Result<()> {
        let written = note.rotate_left(3).wrapping_add(777);
        ctx.accounts.pass_meta.last_note = written;

        // pass の Pubkey だけ埋めた「ダミー」データにする等、CPI 先が当該口座を
        // 可変参照しない契約にしておく（ここでは例示のためそのまま同じ LOG_ID を使用）。
        let metas = vec![
            AccountMeta::new_readonly(ctx.accounts.pass.key(), false), // ← writable にしない／口座自体を渡さない設計にする
            AccountMeta::new_readonly(ctx.accounts.owner.key(), true),
            AccountMeta::new_readonly(ctx.accounts.collector.key(), false),
        ];
        let data = written.to_le_bytes().to_vec();

        let ix = Instruction { program_id: LOG_ID, accounts: metas, data };
        invoke(
            &ix,
            &[
                ctx.accounts.log_hint.to_account_info(),
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.collector.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ArchivePassAndLog<'info> {
    /// “閉じる予定”の口座。命令終了時に close が実行される。
    #[account(mut, close = collector)]
    pub pass: Account<'info, PassPage>,
    #[account(mut)]
    pub pass_meta: Account<'info, PassMeta>,
    pub owner: Signer<'info>,
    /// CHECK: 受け取り先
    #[account(mut)]
    pub collector: UncheckedAccount<'info>,
    /// CHECK: ログ用ヒント（固定IDのプログラムに渡すためのアカウント群の一部）
    pub log_hint: UncheckedAccount<'info>,
}

#[account]
pub struct PassPage {
    pub stamp: u64,
}

#[account]
pub struct PassMeta {
    pub last_note: u64,
}
