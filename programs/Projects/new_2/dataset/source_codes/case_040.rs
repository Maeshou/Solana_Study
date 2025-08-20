use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqUnlockAch");

#[program]
pub mod nft_achievement {
    use super::*;

    /// プレイヤーのアチーブメントアカウントに新しいアチーブメントを追加する  
    /// （owner チェックを行っていないため、任意のアカウントで他人の実績を解放できます）
    pub fn unlock_achievement(
        ctx: Context<UnlockAchievement>,
        achievement_id: u8,  // 解放する実績の ID
    ) -> Result<()> {
        let acct = &mut ctx.accounts.achievement_acc.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── レイアウト想定 ──
        // [0]        : u8   現在の実績数 N
        // [1..1+M]   : u8×M 実績 ID リスト（最大 M 件まで）
        const MAX_ACH: usize = 16;
        let required = 1 + MAX_ACH;

        // 1) 必要バイト数チェック
        if data.len() < required {
            return err!(ErrorCode::DataTooShort);
        }

        // 先頭バイトを現在数として読み出し
        let current = data[0] as usize;

        // 2) 現在数が上限未満か？
        if current < MAX_ACH {
            let idx = 1 + current;

            // 3) 次に書き込むインデックスが範囲内か？
            if idx < required {
                // 解放したいアチーブメント ID を書き込み
                data[idx] = achievement_id;
                // カウントを増やす (saturating_add で溢れ防止)
                data[0] = data[0].saturating_add(1);

                msg!(
                    "Achievement {} unlocked for {} (total: {})",
                    achievement_id,
                    ctx.accounts.player.key(),
                    data[0]
                );
            } else {
                return err!(ErrorCode::IndexOutOfBounds);
            }
        } else {
            return err!(ErrorCode::ListFull);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UnlockAchievement<'info> {
    /// CHECK: owner == program_id チェックを省略している危険な AccountInfo
    #[account(mut)]
    pub achievement_acc: AccountInfo<'info>,

    /// 実績を解放するプレイヤーの署名のみを検証
    pub player: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
    #[msg("実績リストが既に満杯です")]
    ListFull,
    #[msg("実績リストのインデックスが範囲外です")]
    IndexOutOfBounds,
}
