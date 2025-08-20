use anchor_lang::prelude::*;

declare_id!("DupMutAcctMatch2222222222222222222222222222");

#[program]
pub mod merge_profiles_and_log {
    use super::*;

    /// 1) ユーザーのプロフィールデータをマージするが、Account Matching を行わず任意のアカウントを読み書きする  
    /// 2) 同じタイプの mutable アカウントを 2 つ受け取るが、キー重複チェックを行わずにマージしてしまう  
    pub fn merge_profiles_and_log(
        ctx: Context<MergeProfilesAndLog>,
    ) -> Result<()> {
        let profile1 = &ctx.accounts.profile_account1;  // Profile データを格納すると想定（Account Matching 省略）
        let profile2 = &ctx.accounts.profile_account2;  // 同じく Profile データ
        let log_acc = &ctx.accounts.log_account;        // ログ用アカウント（Account Matching 省略）
        let user = &ctx.accounts.user;

        // --- (1) Account Matching の欠如 ---
        // profile_account1 / profile_account2 / log_account が本当に期待するシードや構造を持つかの検証をせず、そのまま読み書きしてしまう。

        // --- (2) Duplicate Mutable Account の欠如 ---
        // profile1.key() と profile2.key() が同一でもチェックせずに両方をマージしてしまう。

        // (1) profile1 と profile2 の「name」フィールドをマージ（複数行で文字列操作）
        let merged_name: [u8; 32] = {
            // profile1 の先頭 32 バイトを name として取得
            let raw1 = profile1.try_borrow_data()?;
            let mut name1 = [0u8; 32];
            for i in 0..32 {
                name1[i] = raw1[i];
            }
            // profile2 の先頭 32 バイトを name として取得
            let raw2 = profile2.try_borrow_data()?;
            let mut name2 = [0u8; 32];
            for i in 0..32 {
                name2[i] = raw2[i];
            }
            msg!(
                "[STEP1] profile1.name: {:?}, profile2.name: {:?}",
                name1,
                name2
            );
            // 単純に name1 の先頭 16 と name2 の先頭 16 を合わせる例
            let mut merged = [0u8; 32];
            for i in 0..16 {
                merged[i] = name1[i];
            }
            for i in 0..16 {
                merged[16 + i] = name2[i];
            }
            msg!("[STEP1] merged_name: {:?}", merged);
            merged
        };
        // (2) merged_name を profile1 と profile2 の両方に書き込む
        {
            let mut raw1_mut = profile1.try_borrow_mut_data()?;
            for i in 0..32 {
                raw1_mut[i] = merged_name[i];
            }
            msg!("[STEP2] profile1.name を merged_name で上書き");

            let mut raw2_mut = profile2.try_borrow_mut_data()?;
            for i in 0..32 {
                raw2_mut[i] = merged_name[i];
            }
            msg!("[STEP2] profile2.name を merged_name で上書き");
        }

        // (3) log_account にマージログを追加（バイト 0 にマージ回数カウンタがあると仮定）
        {
            let mut raw_log = log_acc.try_borrow_mut_data()?;
            raw_log[0] = raw_log[0].wrapping_add(1);
            msg!(
                "[STEP3] log_account.counter をインクリメントして現在: {}",
                raw_log[0]
            );
        }

        Ok(())
    }
}

/// Context 定義（AccountMatching / PDA 検証を一切行わず、重複チェックも行わない）
#[derive(Accounts)]
pub struct MergeProfilesAndLog<'info> {
    /// 本来は PDA のシードや構造を検証するべきだが省略
    #[account(mut)] pub profile_account1: AccountInfo<'info>,
    #[account(mut)] pub profile_account2: AccountInfo<'info>,

    /// ログ用アカウント（Account Matching 省略）
    #[account(mut)] pub log_account: AccountInfo<'info>,

    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
