use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqSkinEq01");

#[program]
pub mod skin_equipment {
    use super::*;

    /// キャラクターにスキンを装備する  
    /// （`equipment_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人の装備アカウントを指定し、自分のスキンを装備できます）
    pub fn equip_skin(
        ctx: Context<EquipSkin>,
        new_skin: Pubkey,    // 装備するスキンの Pubkey
    ) -> Result<()> {
        let acct = &mut ctx.accounts.equipment_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── データレイアウト想定 ──
        // [0..32]   Pubkey 装備中スキン
        // [32..36]  u32  スキンバリエーションID
        const MIN_LEN: usize = 32 + 4;
        if data.len() < MIN_LEN {
            return err!(ErrorCode::DataTooShort);
        }

        // 1) スキン Pubkey 部分を新しいものに上書き
        data.chunks_mut(32)
            .next()
            .map(|slot| slot.copy_from_slice(new_skin.as_ref()));

        msg!(
            "Equipped skin {} on account {}",
            new_skin,
            acct.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EquipSkin<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない
    #[account(mut)]
    pub equipment_account: AccountInfo<'info>,

    /// 呼び出し元ユーザーの署名のみ検証
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("装備アカウントのデータ領域が不足しています")]
    DataTooShort,
}
