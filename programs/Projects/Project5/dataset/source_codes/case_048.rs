// 5) Rune Dice Hash — ダイス合成（PDAなし）
declare_id!("RDHX555555555555555555555555555555555");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum TableState { Prep, Cast, Lock }

#[program]
pub mod rune_dice_hash {
    use super::*;
    use TableState::*;

    pub fn init_table(ctx: Context<InitTable>) -> Result<()> {
        let s = &mut ctx.accounts;
        s.table.host = s.host.key();
        s.table.state = Prep;
        Ok(())
    }

    pub fn cast(ctx: Context<CastRunes>, turns: u32) -> Result<()> {
        let s = &mut ctx.accounts;

        for t in 0..turns {
            let h = hashv(&[s.table.host.as_ref(), &s.bag.rolls.to_le_bytes(), &t.to_le_bytes()]);
            let v = u32::from_le_bytes([h.0[0], h.0[1], h.0[2], h.0[3]]);
            s.bag.rolls = s.bag.rolls.wrapping_add(v & 0x3FF);
            s.log.faces = s.log.faces.rotate_left((v % 31) + 1);
            s.log.turns = s.log.turns.wrapping_add(1);
        }

        if (s.bag.rolls & 1) == 1 {
            s.table.state = Lock;
            s.log.flags = s.log.flags.wrapping_add(2);
            s.bag.rolls ^= s.log.faces;
            s.log.faces = s.log.faces.rotate_right(4);
            msg!("odd parity: lock, flags+2, xor+rotate");
        } else {
            s.table.state = Cast;
            s.log.turns = s.log.turns.wrapping_mul(2);
            s.bag.rolls = s.bag.rolls.wrapping_add(7);
            s.log.faces ^= 0xDEAD_BEEF;
            msg!("even parity: cast, turns*2, rolls+7, faces xor");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTable<'info> {
    #[account(init, payer=payer, space=8+32+1)]
    pub table: Account<'info, DiceTable>,
    #[account(init, payer=payer, space=8+4)]
    pub bag: Account<'info, DiceBag>,
    #[account(init, payer=payer, space=8+4+4)]
    pub log: Account<'info, DiceLog>,
    #[account(mut)] pub payer: Signer<'info>,
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CastRunes<'info> {
    #[account(mut, has_one=host)]
    pub table: Account<'info, DiceTable>,
    #[account(
        mut,
        constraint = bag.key() != table.key() @ RdErr::Dup,
        constraint = bag.key() != log.key() @ RdErr::Dup
    )]
    pub bag: Account<'info, DiceBag>,
    #[account(
        mut,
        constraint = log.key() != table.key() @ RdErr::Dup
    )]
    pub log: Account<'info, DiceLog>,
    pub host: Signer<'info>,
}
#[account] pub struct DiceTable { pub host: Pubkey, pub state: TableState }
#[account] pub struct DiceBag { pub rolls: u32 }
#[account] pub struct DiceLog { pub faces: u32, pub turns: u32, pub flags: u32 }
#[error_code] pub enum RdErr { #[msg("dup")] Dup }
