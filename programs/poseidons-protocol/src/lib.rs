use anchor_lang::prelude::*;

declare_id!("E65MHJjJT2ihTLgMfHdWtsNYMA6iJsCjraf7UrxU2sDP");

#[program]
pub mod poseidons_protocol {
    use super::*;

    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    

}
#[derive(Accounts)]
pub struct Initialize {}
