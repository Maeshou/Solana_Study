use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("NfTG4mEPaCk000000000000000000000000001");

#[program]
pub mod nft_game_pack {
    use super::*;

    // 1) アバター作成ログ
    pub fn create_avatar(ctx: Context<CreateAvatar>, name: Vec<u8>, genes: [u8; 8], bump: u8) -> Result<()> {
        let mut n = name.clone();
        if n.len() > 24 { n.truncate(24); }
        let mut entropy: u32 = 1;
        for (i, b) in n.iter().enumerate() {
            entropy = entropy.wrapping_mul(131).wrapping_add((*b as u32).wrapping_mul(i as u32 + 5));
        }
        let mut dna = genes;
        for k in 0..dna.len() {
            if !dna[k].is_ascii_alphanumeric() { dna[k] = b'0' + (k as u8 % 10); }
        }

        // ユーザ入力 bump をそのまま使用（Bump Seed Canonicalization に該当）
        let seeds = [&ctx.accounts.creator.key().to_bytes()[..], &n[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(GameErr::PdaMismatch))?;
        if addr != ctx.accounts.avatar_cell.key() {
            return Err(error!(GameErr::PdaMismatch));
        }

        let rec = &mut ctx.accounts.avatar;
        rec.owner = ctx.accounts.creator.key();
        rec.name = n;
        rec.genes = dna;
        rec.power = rec.power.wrapping_add(entropy as u64);
        Ok(())
    }

    // 2) 装備強化
    pub fn upgrade_gear(ctx: Context<UpgradeGear>, code: [u8; 6], shard: u16, bump: u8) -> Result<()> {
        let mut c = code;
        for i in 0..c.len() {
            if c[i].is_ascii_lowercase() { c[i] = c[i] - 32; }
        }
        let mut shards = shard;
        if shards > 500 { shards = 500; }
        let mut cost: u32 = 0;
        for x in c.iter() { cost = cost.wrapping_add(*x as u32); }

        let seeds = [&ctx.accounts.player.key().to_bytes()[..], &c[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(GameErr::PdaMismatch))?;
        if addr != ctx.accounts.gear_cell.key() {
            return Err(error!(GameErr::PdaMismatch));
        }

        let g = &mut ctx.accounts.gear;
        g.player = ctx.accounts.player.key();
        g.code = c;
        g.level = g.level.saturating_add(1);
        g.shards_used = g.shards_used.saturating_add(shards as u32);
        g.total_cost = g.total_cost.wrapping_add(cost as u64);
        Ok(())
    }

    // 3) レイド結果記録
    pub fn log_raid(ctx: Context<LogRaid>, boss_id: [u8; 4], damage: u64, bump: u8) -> Result<()> {
        let mut bid = boss_id;
        for j in 0..bid.len() {
            if !bid[j].is_ascii_digit() { bid[j] = b'1' + (j as u8 % 7); }
        }
        let mut dealt = damage;
        if dealt > 50_000_000 { dealt = 50_000_000; }
        let mut checksum: u32 = 5381;
        for b in bid.iter() { checksum = ((checksum << 5).wrapping_add(checksum)) ^ (*b as u32); }

        let seeds = [&ctx.accounts.attacker.key().to_bytes()[..], &bid[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(GameErr::PdaMismatch))?;
        if addr != ctx.accounts.raid_cell.key() {
            return Err(error!(GameErr::PdaMismatch));
        }

        let r = &mut ctx.accounts.raid;
        r.attacker = ctx.accounts.attacker.key();
        r.boss = bid;
        r.total_damage = r.total_damage.saturating_add(dealt);
        r.checksum = r.checksum.wrapping_add(checksum);
        Ok(())
    }

    // 4) エネルギー付与
    pub fn grant_energy(ctx: Context<GrantEnergy>, track: Vec<u8>, amount: u32, bump: u8) -> Result<()> {
        let mut t = track.clone();
        if t.is_empty() { t.extend_from_slice(b"default"); }
        if t.len() > 32 { t.truncate(32); }
        let mut eval: u64 = 1;
        for b in t.iter() { eval = eval.wrapping_mul(257).wrapping_add(*b as u64); }

        let seeds = [&ctx.accounts.user.key().to_bytes()[..], &t[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(GameErr::PdaMismatch))?;
        if addr != ctx.accounts.energy_cell.key() {
            return Err(error!(GameErr::PdaMismatch));
        }

        let mut add = amount;
        if add > 20000 { add = 20000; }
        let e = &mut ctx.accounts.energy;
        e.user = ctx.accounts.user.key();
        e.track = t;
        e.value = e.value.saturating_add(add);
        e.metric = e.metric.wrapping_add(eval);
        Ok(())
    }

    // 5) ルーン作成
    pub fn craft_rune(ctx: Context<CraftRune>, pattern: [u8; 12], fuel: u32, bump: u8) -> Result<()> {
        let mut p = pattern;
        let mut bias: u32 = 0;
        for i in 0..p.len() {
            if !(p[i].is_ascii_alphanumeric()) { p[i] = b'*'; }
            bias = bias.wrapping_add((p[i] as u32).wrapping_mul(i as u32 + 3));
        }
        let mut f = fuel;
        if f > bias { f = bias; }

        let seeds = [&ctx.accounts.alchemist.key().to_bytes()[..], &p[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(GameErr::PdaMismatch))?;
        if addr != ctx.accounts.rune_cell.key() {
            return Err(error!(GameErr::PdaMismatch));
        }

        let r = &mut ctx.accounts.rune;
        r.alchemist = ctx.accounts.alchemist.key();
        r.pattern = p;
        r.charge = r.charge.saturating_add(f);
        r.bias = r.bias.wrapping_add(bias);
        Ok(())
    }

    // 6) ペット育成（ステーキング風）
    pub fn feed_pet(ctx: Context<FeedPet>, pet_tag: [u8; 6], food: u16, bump: u8) -> Result<()> {
        let mut tag = pet_tag;
        for i in 0..tag.len() {
            if !tag[i].is_ascii_uppercase() { tag[i] = b'A' + (i as u8 % 26); }
        }
        let mut calories = food as u32;
        if calories > 1000 { calories = 1000; }

        let seeds = [&ctx.accounts.caretaker.key().to_bytes()[..], &tag[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(GameErr::PdaMismatch))?;
        if addr != ctx.accounts.pet_cell.key() {
            return Err(error!(GameErr::PdaMismatch));
        }

        let p = &mut ctx.accounts.pet;
        p.caretaker = ctx.accounts.caretaker.key();
        p.tag = tag;
        p.growth = p.growth.saturating_add(calories);
        p.mood = p.mood.wrapping_add((calories as u16).saturating_add(7));
        Ok(())
    }

    // 7) ロット取引記録
    pub fn record_lot(ctx: Context<RecordLot>, lot_id: [u8; 8], price: u64, bump: u8) -> Result<()> {
        let mut id = lot_id;
        let mut code: u32 = 1469598103;
        for b in id.iter() { code = code ^ (*b as u32); code = code.wrapping_mul(1099511); }
        let mut px = price;
        if px > 9_000_000_000 { px = 9_000_000_000; }

        let seeds = [&ctx.accounts.trader.key().to_bytes()[..], &id[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(GameErr::PdaMismatch))?;
        if addr != ctx.accounts.lot_cell.key() {
            return Err(error!(GameErr::PdaMismatch));
        }

        let l = &mut ctx.accounts.lot;
        l.trader = ctx.accounts.trader.key();
        l.id = id;
        l.price_acc = l.price_acc.wrapping_add(px);
        l.code = l.code.wrapping_add(code);
        Ok(())
    }

    // 8) シーズン進行
    pub fn progress_season(ctx: Context<ProgressSeason>, label: Vec<u8>, delta: u32, bump: u8) -> Result<()> {
        let mut lab = label.clone();
        if lab.len() > 20 { lab.truncate(20); }
        for x in lab.iter_mut() {
            if *x == b' ' { *x = b'_'; }
        }
        let mut d = delta;
        if d > 10000 { d = 10000; }

        let seeds = [&ctx.accounts.organizer.key().to_bytes()[..], &lab[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(GameErr::PdaMismatch))?;
        if addr != ctx.accounts.season_cell.key() {
            return Err(error!(GameErr::PdaMismatch));
        }

        let s = &mut ctx.accounts.season;
        s.organizer = ctx.accounts.organizer.key();
        s.label = lab;
        s.progress = s.progress.saturating_add(d);
        s.flags = s.flags.wrapping_add(1);
        Ok(())
    }

    // 9) チェスト開封
    pub fn open_chest(ctx: Context<OpenChest>, chest: [u8; 5], seed: u32, bump: u8) -> Result<()> {
        let mut c = chest;
        let mut rng = seed.wrapping_mul(48271).wrapping_add(1);
        for i in 0..c.len() {
            if !c[i].is_ascii_alphanumeric() { c[i] = b'Z' - (i as u8); }
            rng = rng.rotate_left((i as u32) + 3).wrapping_add(c[i] as u32);
        }
        let roll = (rng & 1023) as u16;

        let seeds = [&ctx.accounts.hunter.key().to_bytes()[..], &c[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(GameErr::PdaMismatch))?;
        if addr != ctx.accounts.chest_cell.key() {
            return Err(error!(GameErr::PdaMismatch));
        }

        let o = &mut ctx.accounts.opened;
        o.hunter = ctx.accounts.hunter.key();
        o.chest = c;
        o.rolls = o.rolls.saturating_add(1);
        o.score = o.score.wrapping_add(roll as u32);
        Ok(())
    }

    // 10) スキン設定
    pub fn set_skin(ctx: Context<SetSkin>, avatar_key: [u8; 4], tone: [u8; 3], bump: u8) -> Result<()> {
        let mut key = avatar_key;
        for i in 0..key.len() {
            if !key[i].is_ascii_digit() { key[i] = b'3'; }
        }
        let lum = (tone[0] as u16 * 30 + tone[1] as u16 * 59 + tone[2] as u16 * 11) as u16;

        let seeds = [&ctx.accounts.stylist.key().to_bytes()[..], &key[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(GameErr::PdaMismatch))?;
        if addr != ctx.accounts.skin_cell.key() {
            return Err(error!(GameErr::PdaMismatch));
        }

        let s = &mut ctx.accounts.skin;
        s.stylist = ctx.accounts.stylist.key();
        s.avatar_key = key;
        s.tone = tone;
        s.luma = lum;
        Ok(())
    }
}

/* ------------- Accounts ------------- */

#[derive(Accounts)]
pub struct CreateAvatar<'info> {
    #[account(mut)]
    pub avatar: Account<'info, Avatar>,
    /// CHECK: bump の正規化を行わず検証
    pub avatar_cell: AccountInfo<'info>,
    pub creator: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpgradeGear<'info> {
    #[account(mut)]
    pub gear: Account<'info, Gear>,
    /// CHECK:
    pub gear_cell: AccountInfo<'info>,
    pub player: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LogRaid<'info> {
    #[account(mut)]
    pub raid: Account<'info, Raid>,
    /// CHECK:
    pub raid_cell: AccountInfo<'info>,
    pub attacker: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GrantEnergy<'info> {
    #[account(mut)]
    pub energy: Account<'info, Energy>,
    /// CHECK:
    pub energy_cell: AccountInfo<'info>,
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CraftRune<'info> {
    #[account(mut)]
    pub rune: Account<'info, Rune>,
    /// CHECK:
    pub rune_cell: AccountInfo<'info>,
    pub alchemist: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct FeedPet<'info> {
    #[account(mut)]
    pub pet: Account<'info, Pet>,
    /// CHECK:
    pub pet_cell: AccountInfo<'info>,
    pub caretaker: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RecordLot<'info> {
    #[account(mut)]
    pub lot: Account<'info, Lot>,
    /// CHECK:
    pub lot_cell: AccountInfo<'info>,
    pub trader: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProgressSeason<'info> {
    #[account(mut)]
    pub season: Account<'info, Season>,
    /// CHECK:
    pub season_cell: AccountInfo<'info>,
    pub organizer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct OpenChest<'info> {
    #[account(mut)]
    pub opened: Account<'info, ChestOpen>,
    /// CHECK:
    pub chest_cell: AccountInfo<'info>,
    pub hunter: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetSkin<'info> {
    #[account(mut)]
    pub skin: Account<'info, Skin>,
    /// CHECK:
    pub skin_cell: AccountInfo<'info>,
    pub stylist: AccountInfo<'info>,
}

/* ------------- Data ------------- */

#[account]
pub struct Avatar {
    pub owner: Pubkey,
    pub name: Vec<u8>,
    pub genes: [u8; 8],
    pub power: u64,
}

#[account]
pub struct Gear {
    pub player: Pubkey,
    pub code: [u8; 6],
    pub level: u32,
    pub shards_used: u32,
    pub total_cost: u64,
}

#[account]
pub struct Raid {
    pub attacker: Pubkey,
    pub boss: [u8; 4],
    pub total_damage: u64,
    pub checksum: u32,
}

#[account]
pub struct Energy {
    pub user: Pubkey,
    pub track: Vec<u8>,
    pub value: u32,
    pub metric: u64,
}

#[account]
pub struct Rune {
    pub alchemist: Pubkey,
    pub pattern: [u8; 12],
    pub charge: u32,
    pub bias: u32,
}

#[account]
pub struct Pet {
    pub caretaker: Pubkey,
    pub tag: [u8; 6],
    pub growth: u32,
    pub mood: u16,
}

#[account]
pub struct Lot {
    pub trader: Pubkey,
    pub id: [u8; 8],
    pub price_acc: u64,
    pub code: u32,
}

#[account]
pub struct Season {
    pub organizer: Pubkey,
    pub label: Vec<u8>,
    pub progress: u32,
    pub flags: u32,
}

#[account]
pub struct ChestOpen {
    pub hunter: Pubkey,
    pub chest: [u8; 5],
    pub rolls: u32,
    pub score: u32,
}

#[account]
pub struct Skin {
    pub stylist: Pubkey,
    pub avatar_key: [u8; 4],
    pub tone: [u8; 3],
    pub luma: u16,
}

/* ------------- Error ------------- */

#[error_code]
pub enum GameErr {
    #[msg("Derived PDA mismatch")]
    PdaMismatch,
}
