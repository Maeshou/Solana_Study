// 6) cargo_router: 積載量に応じて2段ヘッダ + ペイロード長を付与
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
declare_id!("CargoRoute111111111111111111111111111111");

#[program]
pub mod cargo_router {
    use super::*;
    pub fn ship(ctx: Context<Ship>, weight: u64) -> Result<()> {
        let st = &mut ctx.accounts.cargo;
        st.trips += 1;

        let mut program = ctx.accounts.route_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            st.route_a += weight;
            program = ctx.remaining_accounts[0].clone();
        } else {
            st.route_b += weight;
        }

        let header1 = st.trips.to_le_bytes();
        let header2 = weight.to_le_bytes();
        let mut data = Vec::with_capacity(24);
        data.extend_from_slice(&header1);
        data.extend_from_slice(&header2);
        data.extend_from_slice(&[12u8, 34u8, 56u8, 78u8, 0u8, 0u8, 0u8, 0u8]);

        let br = CargoBridge { dock: ctx.accounts.dock_buf.to_account_info(), bay: ctx.accounts.bay_buf.to_account_info() };
        let cx = br.as_cpi(program.clone());
        br.send(cx, data)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Ship<'info> {
    #[account(init, payer = admin, space = 8 + 8 + 8 + 8)]
    pub cargo: Account<'info, CargoState>,
    #[account(mut)] pub admin: Signer<'info>,
    /// CHECK:
    pub dock_buf: AccountInfo<'info>,
    /// CHECK:
    pub bay_buf: AccountInfo<'info>,
    /// CHECK:
    pub route_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct CargoState { pub trips: u64, pub route_a: u64, pub route_b: u64 }
#[derive(Clone)] pub struct CargoBridge<'info> { pub dock: AccountInfo<'info>, pub bay: AccountInfo<'info> }
impl<'info> CargoBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, CargoBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.dock.key, false), AccountMeta::new_readonly(*self.bay.key, false)] }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.dock.clone(), self.bay.clone()] }
    pub fn send(&self, ctx: CpiContext<'_, '_, '_, 'info, CargoBridge<'info>>, bytes: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: bytes };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
