use anchor_lang::prelude::*;

declare_id!("5Y5s4n7wKzMhXk1VjE6s9a2b3c4d5f6G7h8i9J0K1L2");

#[program]
pub mod equip_extra {
    use super::*;

    pub fn swap(ctx: Context<SwapSlot>, idx: u8, new_id: u64) -> Result<()> {
        let s = &mut ctx.accounts.slots;
        let i = idx as usize;

        // 1. 範囲外インデックスはエラー
        require!(i < s.slots.len(), ErrorCode::IndexOutOfRange);
        // 2. new_id はゼロ不可
        require!(new_id != 0, ErrorCode::InvalidNewId);
        // 3. 署名者チェック
        require!(ctx.accounts.authority.is_signer, ErrorCode::MissingSigner);
        // 4. 所有者チェック
        require!(s.authority == ctx.accounts.authority.key(), ErrorCode::InvalidOwner);

        if s.slots[i] != 0 {
            // 既存装備と入れ替え
            let old = s.slots[i];
            s.slots[i] = new_id;
            s.swap_count = s.swap_count.checked_add(1).ok_or(ErrorCode::Overflow)?;
            s.last_swapped = old;
        } else {
            // 空スロットなら装備
            s.slots[i] = new_id;
            s.equipped_count = s.equipped_count.checked_add(1).ok_or(ErrorCode::Overflow)?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SwapSlot<'info> {
    #[account(mut, has_one = authority)]
    pub slots: Account<'info, SlotsExtraData>,
    pub authority: Signer<'info>,
}

#[account]
pub struct SlotsExtraData {
    pub authority: Pubkey,
    pub slots: [u64; 4],
    pub swap_count: u64,
    pub last_swapped: u64,
    pub equipped_count: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Missing signer on authority account")]
    MissingSigner,
    #[msg("Authority mismatch")]
    InvalidOwner,
    #[msg("Index out of range")] 
    IndexOutOfRange,
    #[msg("New equipment ID must be non-zero")]
    InvalidNewId,
    #[msg("Arithmetic overflow/underflow")] 
    Overflow,
}
