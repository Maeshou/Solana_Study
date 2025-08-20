use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("NftBreedLab11111111111111111111111111111");

#[program]
pub mod nft_breeding_lab {
    use super::*;
    pub fn breed(ctx: Context<Breed>, dna_a: u64, dna_b: u64, tries: u64) -> Result<()> {
        let st = &mut ctx.accounts.lab;
        st.sessions += 1;

        let mut program = ctx.accounts.router.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            program = ctx.remaining_accounts[0].clone();
            st.alt_paths += 1;
        } else {
            st.quarantine += 1;
            st.fail_notes.push(((dna_a ^ dna_b) & 0xffff) as u16);
            st.mix_bias = st.mix_bias.wrapping_add((dna_a.wrapping_add(dna_b)) & 7);
        }

        // tries でDNAを合成して単発送信
        let scale = ((Clock::get()?.slot & 15) + 1) as u64;
        let mut mixed: u128 = 0;
        let mut i = 0u64;
        while i < tries {
            mixed = mixed.wrapping_add(((dna_a ^ (i + st.mix_bias)) as u128) * ((dna_b + scale) as u128));
            i += 1;
        }

        let br = BreedBridge { parent_a: ctx.accounts.parent_a.to_account_info(), parent_b: ctx.accounts.parent_b.to_account_info() };
        let mut payload = Vec::with_capacity(24);
        payload.extend_from_slice(&st.sessions.to_le_bytes());
        payload.extend_from_slice(&(mixed as u64).to_le_bytes());
        payload.extend_from_slice(&tries.to_le_bytes());

        let cx = br.as_cpi(program.clone());
        br.send_mix(cx, payload)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Breed<'info> {
    #[account(init, payer = keeper, space = 8 + 8 + 8 + 8 + 1 + 4 + (4 + 64*2))]
    pub lab: Account<'info, LabState>,
    #[account(mut)] pub keeper: Signer<'info>,
    /// CHECK:
    pub parent_a: AccountInfo<'info>,
    /// CHECK:
    pub parent_b: AccountInfo<'info>,
    /// CHECK:
    pub router: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LabState {
    pub sessions: u64,
    pub alt_paths: u64,
    pub quarantine: u64,
    pub mix_bias: u64,
    pub last_child: u8,
    pub pad: u32,
    pub fail_notes: Vec<u16>,
}

#[derive(Clone)]
pub struct BreedBridge<'info> { pub parent_a: AccountInfo<'info>, pub parent_b: AccountInfo<'info> }
impl<'info> BreedBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, BreedBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.parent_a.key, false), AccountMeta::new_readonly(*self.parent_b.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.parent_a.clone(), self.parent_b.clone()] }
    pub fn send_mix(&self, cx: CpiContext<'_, '_, '_, 'info, BreedBridge<'info>>, bytes: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: bytes };
        invoke(&ix, &self.infos(&cx.program))?; Ok(())
    }
}
