// Guild Treasury v2 — 初期化ロジックを多様化した版（置き換え用）
use anchor_lang::prelude::*;
declare_id!("Gu1ldTr3asuryRew0rk00000000000000000000001");

#[program]
pub mod guild_treasury_v2 {
    use super::*;
    use VoucherKind::*;

    /// Treasury初期化：
    /// - nameからfingerprintを算出
    /// - config_nonceから初期epochを導出
    /// - lanes_maskで使用可能レーン集合を指定（ビットマスク）
    /// - rolling配列はnonceとnameを混ぜてループで初期化
    pub fn init_treasury(
        ctx: Context<InitTreasury>,
        name: String,
        config_nonce: u32,
        lanes_mask: u32,
    ) -> Result<()> {
        let t = &mut ctx.accounts.treasury;
        t.owner = ctx.accounts.chair.key();

        // 名前は最大64バイト相当に制限（超過は切り詰め）
        let mut nm = name;
        if nm.len() > 64 { nm.truncate(64); }
        t.name = nm;

        // 簡易指紋：名前バイトを畳み込み → 0/32bit両端回転でミックス
        let mut fp: u32 = 0x811C9DC5 ^ config_nonce;
        for (i, b) in t.name.as_bytes().iter().enumerate() {
            fp = fp.rotate_left((i as u32) & 7) ^ (*b as u32);
            fp = fp.wrapping_mul(0x01000193);
        }
        t.fingerprint = fp;

        // 初期エポックはnonceを縮約して設定（0は避ける）
        t.epoch = ((config_nonce % 97) + 1) as u16;

        // 使用可能レーン集合（上位は無効のまま許容）
        t.lanes_mask = lanes_mask;

        // ローリング初期化（擬似乱数風）
        let mut state = (config_nonce as u64) ^ 0x9E37_79B9_7F4A_7C15;
        for i in 0..t.rolling.len() {
            state = state.rotate_left(9) ^ (fp as u64).rotate_right((i as u32) & 31);
            t.rolling[i] = (state as u32) ^ (fp.rotate_left(i as u32));
        }

        // ネットは0開始、監査フラグはname長で初期化
        t.net = 0;
        t.audit_flags = (t.name.len() as u32) & 0x3FF;

        Ok(())
    }

    /// Voucher初期化：
    /// - laneの範囲／許可ビット検証
    /// - seedとkindから初期amountを非線形に生成
    /// - window配列もseedとhintで初期化
    pub fn init_voucher(
        ctx: Context<InitVoucher>,
        kind: VoucherKind,
        lane: u8,
        hint: u32,
        seed: u64,
    ) -> Result<()> {
        let v = &mut ctx.accounts.voucher;
        v.treasury = ctx.accounts.treasury.key();
        v.kind = kind;

        // lane検証：0..31かつTreasuryのlanes_maskで許可されていること
        require!(lane < 32, ErrCode::LaneOutOfRange);
        let allowed = (ctx.accounts.treasury.lanes_mask >> lane) & 1;
        require!(allowed == 1, ErrCode::LaneDisabled);
        v.lane = lane;

        // 初期amount：seed・hint・kindをブレンド
        let base = seed
            .rotate_left((lane as u32) & 31)
            .wrapping_add(hint as u64)
            ^ 0xC3A5_C85C_97CB_3127u64;
        let weight = match kind {
            Expense => 3u64,
            Income  => 5u64,
            Transfer => 2u64,
        };
        v.amount = ((base & 0xFFFF_FFFF) * weight).saturating_rem(1_000_000);

        // window配列初期化：XOR-rotateで拡散
        let mut s = base ^ (v.amount as u64);
        for i in 0..v.window.len() {
            s = s.rotate_left(7) ^ ((hint as u64) << (i as u32 & 15));
            v.window[i] = ((s ^ (i as u64 * 0x9E37)) as u32) & 0x3FFF_FFFF;
        }

        Ok(())
    }

    /// 既存の settle（更新系命令）はそのまま利用可：
    /// - 異なるVoucher（lane不一致）を要求
    /// - has_one/ownerで型・所有者・親子リンクを担保
    pub fn settle(ctx: Context<Settle>, batch: [i64; 5]) -> Result<()> {
        let tr = &mut ctx.accounts.treasury;
        let debit = &mut ctx.accounts.debit;
        let credit = &mut ctx.accounts.credit;
        let q = &mut ctx.accounts.queue;

        // ローリング集計＋クリップ
        let mut acc: i128 = 0;
        for i in 0..5 {
            let scaled = (batch[i] as i128) * ((i as i128) + 1);
            acc = (acc + scaled).clamp(-1_000_000, 1_000_000);
            debit.window[i] = debit.window[i].saturating_add((scaled.unsigned_abs() as u32) & 0x3FFF);
        }

        if debit.kind == VoucherKind::Expense {
            let delta = (acc.unsigned_abs() as u64) & 0x00FF_FFFF;
            tr.net = tr.net.saturating_sub(delta);
            q.cursor = q.cursor.saturating_add(1);
            q.mix = q.mix ^ (delta.rotate_left(11));
            q.last = q.last.wrapping_add(delta as u128);
            msg!("Expense path applied");
        } else {
            let delta = ((acc.abs() as u64) & 0x01FF_FFFF).saturating_add(credit.amount & 0xFFFF);
            tr.net = tr.net.saturating_add(delta);
            q.cursor = q.cursor.saturating_add(2);
            q.mix = q.mix.rotate_right(5) ^ delta;
            q.last = q.last ^ ((delta as u128) << 16);
            msg!("Non-expense path applied");
        }

        // 追記のローリング更新
        for i in 0..tr.rolling.len() {
            tr.rolling[i] = tr.rolling[i].rotate_left((i as u32) & 7) ^ (q.mix & 0xFFFF);
        }

        Ok(())
    }
}

// ───────── Accounts ─────────

#[derive(Accounts)]
pub struct InitTreasury<'info> {
    #[account(init, payer = chair, space = 8 + Treasury::MAX)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub chair: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitVoucher<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub treasury: Account<'info, Treasury>,
    #[account(init, payer = user, space = 8 + Voucher::MAX)]
    pub voucher: Account<'info, Voucher>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub treasury: Account<'info, Treasury>,

    #[account(mut, has_one = treasury, owner = crate::ID)]
    pub queue: Account<'info, TxQueue>,

    #[account(mut, has_one = treasury, owner = crate::ID)]
    pub debit: Account<'info, Voucher>,

    #[account(
        mut,
        has_one = treasury,
        owner = crate::ID,
        // 一意フィールド（lane）の不一致で、同一口座の二重渡しを遮断
        constraint = debit.lane != credit.lane @ ErrCode::CosplayBlocked
    )]
    pub credit: Account<'info, Voucher>,

    pub owner: Signer<'info>,
}

// ───────── Data ─────────

#[account]
pub struct Treasury {
    pub owner: Pubkey,
    pub name: String,
    pub net: u64,
    pub epoch: u16,
    pub fingerprint: u32,
    pub lanes_mask: u32,
    pub audit_flags: u32,
    pub rolling: [u32; 5],
}
impl Treasury {
    // nameを最大64文字として概算
    pub const MAX: usize = 32 + 4 + 64 + 8 + 2 + 4 + 4 + 4 + (4 * 5);
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum VoucherKind { Expense, Income, Transfer }

#[account]
pub struct Voucher {
    pub treasury: Pubkey,
    pub kind: VoucherKind,
    pub lane: u8,          // 一意フィールド
    pub amount: u64,
    pub window: [u32; 5],
}
impl Voucher {
    pub const MAX: usize = 32 + 1 + 1 + 8 + (4 * 5);
}

#[account]
pub struct TxQueue {
    pub treasury: Pubkey,
    pub mix: u32,
    pub cursor: u32,
    pub last: u128,
}
impl TxQueue {
    pub const MAX: usize = 32 + 4 + 4 + 16;
}

#[error_code]
pub enum ErrCode {
    #[msg("Type Cosplay blocked by unique field mismatch")]
    CosplayBlocked,
    #[msg("Lane index must be < 32")]
    LaneOutOfRange,
    #[msg("This lane is disabled by treasury lanes_mask")]
    LaneDisabled,
}
