use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("NftEquipForge1111111111111111111111111111");

#[program]
pub mod nft_equip_forge {
    use super::*;
    pub fn enhance(ctx: Context<Enhance>, power: u64) -> Result<()> {
        let st = &mut ctx.accounts.forge;
        st.calls += 1;

        // 既定は alt_program、指定があれば remaining_accounts[0]
        let mut program = ctx.accounts.alt_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            program = ctx.remaining_accounts[0].clone();
            st.path_a_enhance += power;
        } else {
            // else を厚く：耐性履歴・ペナルティ・最終値などを更新
            st.path_b_fallback += power;
            st.last_fallback_power = power;
            st.durability_penalty = st.durability_penalty.saturating_add(power / 7);
            st.resist_log.push((Clock::get()?.slot as u32, (st.base_resist ^ power) as u32));
            st.base_resist = st.base_resist.wrapping_add((power & 15) + st.calls);
        }

        // 装備NFT(gear_mint) と プレイヤー(avatar) を関与させる
        let br = EquipBridge {
            avatar: ctx.accounts.avatar.to_account_info(),
            gear_mint: ctx.accounts.gear_mint.to_account_info(),
        };

        // 可変チャンク：プレイヤーレベル(seed)とスロット位相
        let slot = Clock::get()?.slot;
        let chunk = ((slot & 3) as u64 + 2) * (1 + (st.calls & 1) as u64);
        let mut left = power;
        while left > 0 {
            let send = if left > chunk { chunk } else { left };
            let mut payload = Vec::with_capacity(24);
            payload.extend_from_slice(&st.calls.to_le_bytes()); // call id
            payload.extend_from_slice(&power.to_le_bytes());    // requested power
            payload.extend_from_slice(&send.to_le_bytes());     // this chunk

            let cx = br.as_cpi(program.clone());
            br.invoke_enhance(cx, payload)?;
            st.total_sent += send;
            left -= send;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Enhance<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8 + 8 + 8 + 8 + 4 + (4 + 64 * 8))]
    pub forge: Account<'info, ForgeState>,
    #[account(mut)] pub user: Signer<'info>,
    /// CHECK: プレイヤーアバター
    pub avatar: AccountInfo<'info>,
    /// CHECK: 装備NFTミント
    pub gear_mint: AccountInfo<'info>,
    /// CHECK: 既定の呼び先
    pub alt_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ForgeState {
    pub calls: u64,
    pub total_sent: u64,
    pub path_a_enhance: u64,
    pub path_b_fallback: u64,
    pub last_fallback_power: u64,
    pub durability_penalty: u64,
    pub base_resist: u64,
    pub resist_log: Vec<(u32, u32)>, // (slot, resist hash)
}

#[derive(Clone)]
pub struct EquipBridge<'info> { pub avatar: AccountInfo<'info>, pub gear_mint: AccountInfo<'info> }
impl<'info> EquipBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, EquipBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(*self.avatar.key, false),
            AccountMeta::new(*self.gear_mint.key, false),
        ]
    }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.avatar.clone(), self.gear_mint.clone()] }
    pub fn invoke_enhance(&self, cx: CpiContext<'_, '_, '_, 'info, EquipBridge<'info>>, bytes: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: bytes };
        invoke(&ix, &self.infos(&cx.program))?; Ok(())
    }
}
