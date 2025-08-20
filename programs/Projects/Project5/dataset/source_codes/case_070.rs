// ======================================================================
// 3) Garden Weave：畝と貯蔵（初期化=三角数Tnを使って土壌指数算出）
// ======================================================================
declare_id!("GARD33333333333333333333333333333333333333");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Soil { Dry, Moist, Rest }

#[program]
pub mod garden_weave {
    use super::*;
    use Soil::*;

    pub fn init_plot(ctx: Context<InitPlot>, rows: u32) -> Result<()> {
        let farm = &mut ctx.accounts.farm;
        farm.owner = ctx.accounts.gardener.key();
        farm.rows = rows;
        farm.state = Dry;

        let bed = &mut ctx.accounts.bed_a;
        let bed2 = &mut ctx.accounts.bed_b;
        let silo = &mut ctx.accounts.silo;

        // Tn = n(n+1)/2 を使って初期soil値を作る
        let t = rows.saturating_mul(rows + 1) / 2;
        bed.parent = farm.key();  bed.row = (rows & 7) as u8;  bed.soil = t % 97 + 7;
        bed2.parent = farm.key(); bed2.row = ((rows >> 1) & 7) as u8; bed2.soil = (t.rotate_left(3) % 83) + 9;

        silo.parent = farm.key(); silo.bin = 5; silo.stock = (t as u64) & 0xFFFF; silo.hash = 1;
        Ok(())
    }

    pub fn tend(ctx: Context<Tend>, loops: u32) -> Result<()> {
        let farm = &mut ctx.accounts.farm;
        let a = &mut ctx.accounts.bed_a;
        let b = &mut ctx.accounts.bed_b;
        let s = &mut ctx.accounts.silo;

        for i in 0..loops {
            // 簡易GCDで雑草指数の正規化
            let mut x = (a.soil as u64) + 17;
            let mut y = (b.soil as u64) + 11;
            while y != 0 { let r = x % y; x = y; y = r; }
            let g = (x as u32).max(1);
            a.soil = a.soil.checked_add(g % 13).unwrap_or(u32::MAX);
            b.soil = b.soil.saturating_add((g % 7) + 1);
            s.hash = s.hash.rotate_left((g % 5) as u32) ^ (a.soil as u64);
        }

        let sum = a.soil + b.soil;
        if sum > farm.rows * 12 {
            farm.state = Rest;
            s.stock = s.stock.saturating_add((sum as u64) & 63);
            a.row ^= 0x1;
            b.row = b.row.saturating_add(1);
            msg!("rest: rotate rows, stock+");
        } else {
            farm.state = Moist;
            a.soil = a.soil.saturating_add(5);
            b.soil = b.soil / 2 + 7;
            s.hash ^= 0x0F0F_F0F0;
            msg!("moist: adjust soils, hash flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPlot<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub farm: Account<'info, Farm>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub bed_a: Account<'info, Bed>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub bed_b: Account<'info, Bed>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub silo: Account<'info, SiloBox>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub gardener: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Tend<'info> {
    #[account(mut, has_one=owner)]
    pub farm: Account<'info, Farm>,
    #[account(
        mut,
        has_one=farm,
        constraint = bed_a.row != bed_b.row @ GardenErr::Dup
    )]
    pub bed_a: Account<'info, Bed>,
    #[account(
        mut,
        has_one=farm,
        constraint = bed_b.row != silo.bin @ GardenErr::Dup
    )]
    pub bed_b: Account<'info, Bed>,
    #[account(mut, has_one=farm)]
    pub silo: Account<'info, SiloBox>,
    pub gardener: Signer<'info>,
}

#[account] pub struct Farm { pub owner: Pubkey, pub rows: u32, pub state: Soil }
#[account] pub struct Bed { pub parent: Pubkey, pub row: u8, pub soil: u32 }
#[account] pub struct SiloBox { pub parent: Pubkey, pub bin: u8, pub stock: u64, pub hash: u64 }

#[error_code] pub enum GardenErr { #[msg("duplicate mutable account")] Dup }
