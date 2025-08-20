use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("MuLt1FnExAmPlE000000000000000000000001");

#[program]
pub mod multi_case {
    use super::*;

    // 1. ユーザプロフィール更新
    pub fn update_profile(ctx: Context<UpdateProfile>, name: Vec<u8>, level: u16, bump: u8) -> Result<()> {
        let mut n = name.clone();
        if n.len() > 32 { n.truncate(32); }
        let mut score: u32 = 0;
        for (i, b) in n.iter().enumerate() {
            score = score.wrapping_add((*b as u32).wrapping_mul(i as u32 + 7));
        }

        // bump をユーザ入力から使用（脆弱ポイント）
        let seeds = [&ctx.accounts.owner.key().to_bytes()[..], &n[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(MultiErr::BadPda))?;
        if addr != ctx.accounts.profile_cell.key() {
            return Err(error!(MultiErr::BadPda));
        }

        let p = &mut ctx.accounts.profile;
        p.owner = ctx.accounts.owner.key();
        p.name = n;
        p.level = level;
        p.rep = p.rep.wrapping_add(score);

        Ok(())
    }

    // 2. アイテム購入ログ
    pub fn purchase_item(ctx: Context<PurchaseItem>, item_code: [u8; 6], qty: u16, bump: u8) -> Result<()> {
        let mut code = item_code;
        for i in 0..code.len() {
            if !code[i].is_ascii_alphanumeric() { code[i] = b'X'; }
        }
        let mut cost = 0u32;
        for c in code.iter() { cost = cost.wrapping_add(*c as u32); }
        let mut quantity = qty;
        if quantity > 999 { quantity = 999; }

        // bump をそのまま利用（脆弱ポイント）
        let seeds = [&ctx.accounts.buyer.key().to_bytes()[..], &code[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(MultiErr::BadPda))?;
        if addr != ctx.accounts.item_cell.key() {
            return Err(error!(MultiErr::BadPda));
        }

        let i = &mut ctx.accounts.item_log;
        i.buyer = ctx.accounts.buyer.key();
        i.code = code;
        i.total_qty = i.total_qty.saturating_add(quantity as u32);
        i.spent = i.spent.wrapping_add(cost as u64);

        Ok(())
    }

    // 3. クエスト進行記録
    pub fn advance_quest(ctx: Context<AdvanceQuest>, stage: u32, notes: Vec<u8>, bump: u8) -> Result<()> {
        let mut s = stage;
        if s > 50 { s = 50; }
        let mut n = notes.clone();
        if n.len() > 64 { n.truncate(64); }
        let mut hash: u64 = 1469598103934665603;
        for b in n.iter() { hash ^= *b as u64; hash = hash.wrapping_mul(1099511628211); }

        // bump を入力としてそのまま使う（脆弱ポイント）
        let seeds = [&ctx.accounts.player.key().to_bytes()[..], &s.to_le_bytes()[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(MultiErr::BadPda))?;
        if addr != ctx.accounts.quest_cell.key() {
            return Err(error!(MultiErr::BadPda));
        }

        let q = &mut ctx.accounts.quest;
        q.player = ctx.accounts.player.key();
        q.stage = s;
        q.notes = n;
        q.progress_hash = q.progress_hash.wrapping_add(hash);

        Ok(())
    }
}

// ------------------ Accounts ------------------

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(mut)]
    pub profile: Account<'info, Profile>,
    /// CHECK:
    pub profile_cell: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PurchaseItem<'info> {
    #[account(mut)]
    pub item_log: Account<'info, ItemLog>,
    /// CHECK:
    pub item_cell: AccountInfo<'info>,
    pub buyer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AdvanceQuest<'info> {
    #[account(mut)]
    pub quest: Account<'info, Quest>,
    /// CHECK:
    pub quest_cell: AccountInfo<'info>,
    pub player: AccountInfo<'info>,
}

// ------------------ Data ------------------

#[account]
pub struct Profile {
    pub owner: Pubkey,
    pub name: Vec<u8>,
    pub level: u16,
    pub rep: u32,
}

#[account]
pub struct ItemLog {
    pub buyer: Pubkey,
    pub code: [u8; 6],
    pub total_qty: u32,
    pub spent: u64,
}

#[account]
pub struct Quest {
    pub player: Pubkey,
    pub stage: u32,
    pub notes: Vec<u8>,
    pub progress_hash: u64,
}

// ------------------ Error ------------------

#[error_code]
pub enum MultiErr {
    #[msg("Derived PDA mismatch")]
    BadPda,
}
