use anchor_lang::prelude::*;

declare_id!("E65MHJjJT2ihTLgMfHdWtsNYMA6iJsCjraf7UrxU2sDP");

#[program]
pub mod poseidons_protocol {
    use super::*;
    use rand::Rng;
    
    //imports random number generator and generates a random number between 0 and 36





    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())

    }
    pub fn game(ctx: Context<Initialize>) -> Result<()> {
        let mut rng = rand::thread_rng();
        let random_number: u8 = rng.gen_range(0..37);
        let mut user: Vec<&str> = Vec::new();//for test purposes, identifies the user
        let mut betValue: Vec<f64> = Vec::new();
        let mut chosenHole: Vec<i32> = Vec::new();
        let mut gainedValue: Vec<f64> = Vec::new();//will determine the value that was obtained
        let mut cont = 0;
        let mut rabbitHole: i32;
        let mut totalBetLost = 0.0;
        let mut totalBetWon = 0.0;
        let mut multiplier : f64;
        
        rabbitHole = random_number;
        println!("The rabbit has chosen the {}° hole", rabbitHole);
        for i in 0.. cont{
           if chosenHole[i] == rabbitHole{
                totalBetWon +=  betValue[i];
                println!("User {} has won", user[i]);
            }else{
                totalBetLost += betValue[i];
                println!("User {} has lost", user[i]);
            }
        }
        println!("ok");
        multiplier = totalBetLost/totalBetWon;
        for i in 0..cont{
            if chosenHole[i] == rabbitHole{
              gainedValue[i] = (betValue[i]*multiplier)+betValue[i];/*makes it so winning users
              always get their entry ammount back + multiplier*/
             println!("User {} won R${}",user[i],gainedValue[i]);
            
            }else{
              gainedValue[i] = 0.0;
            }
        }
    }
}

#[derive(Accounts)]
pub struct Initialize {}
