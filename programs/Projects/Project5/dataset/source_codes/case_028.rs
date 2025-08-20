// ============================================================================
// 4) Totem Orchard — トーテム果樹園（PDAなし・ベクタ処理・定数）
//    防止: has_one + constraint三連
// ============================================================================
declare_id!("TTOC44444444444444444444444444444444");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Cycle { Dawn, Zenith, Dusk }

const BLOOM_PATTERN: [u32; 5] = [3, 5, 8, 13, 21];

#[program]
pub mod totem_orchard {
    use super::*;

    pub fn init_orchard(ctx: Context<InitOrchard>, harmony: u32) -> Result<()> {
        let a = &mut ctx.accounts;
        a.orchard.caretaker = a.caretaker.key();
        a.orchard.harmony = harmony;
        a.orchard.cycle = Cycle::Dawn;
        Ok(())
    }

    pub fn chant(ctx: Context<Chant>, rounds: u8) -> Result<()> {
        let a = &mut ctx.accounts;

        // ベクタ＋iterでまとめて加点（ctx直叙を減らす）
        let seq: Vec<u32> = BLOOM_PATTERN.iter().cycle().take(rounds as usize).copied().collect();
        for v in seq {
            a.totem.power = a.totem.power.saturating_add(v);
            a.totem.runes = a.totem.runes.saturating_add(v / 2);
            a.ledger.echo = a.ledger.echo.saturating_add((v as u64) + 1);
        }

        if a.totem.power > a.orchard.harmony {
            a.orchard.cycle = Cycle::Zenith;
            a.ledger.bless = a.ledger.bless.saturating_add(9);
            a.totem.runes = a.totem.runes.saturating_add(4);
            msg!("zenith: bless+9 runes+4");
        } else {
            a.orchard.cycle = Cycle::Dusk;
            a.ledger.echo = a.ledger.echo.saturating_add(5);
            a.totem.power = a.totem.power.saturating_add(6);
            msg!("dusk: echo+5 power+6");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitOrchard<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub orchard: Account<'info, Orchard>,
    #[account(init, payer = payer, space = 8 + 4 + 1)]
    pub totem: Account<'info, Totem>,
    #[account(init, payer = payer, space = 8 + 8 + 8)]
    pub ledger: Account<'info, ChantLedger>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub caretaker: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Chant<'info> {
    #[account(mut, has_one = caretaker, constraint = orchard.key() != totem.key(), error = OrchardErr::Dup)]
    pub orchard: Account<'info, Orchard>,
    #[account(mut, constraint = totem.key() != ledger.key(), error = OrchardErr::Dup)]
    pub totem: Account<'info, Totem>,
    #[account(mut, constraint = orchard.key() != ledger.key(), error = OrchardErr::Dup)]
    pub ledger: Account<'info, ChantLedger>,
    pub caretaker: Signer<'info>,
}

#[account] pub struct Orchard { pub caretaker: Pubkey, pub harmony: u32, pub cycle: Cycle }
#[account] pub struct Totem { pub power: u32, pub runes: u32 }
#[account] pub struct ChantLedger { pub echo: u64, pub bless: u64 }

#[error_code] pub enum OrchardErr { #[msg("dup")] Dup }