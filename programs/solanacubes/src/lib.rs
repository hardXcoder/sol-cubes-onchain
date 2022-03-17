use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("7e5rKi62msvxHugpdHhFYgCXN1kQcUa49eogzDHLFT42");

#[program]
pub mod solana_cubes {
    use super::*;
   
    pub fn place_bet(ctx: Context<PlaceBet>, amount : i64, is_placed_on_tail : bool) -> ProgramResult {
        let bet : &mut Account<Bet> = &mut ctx.accounts.bet;
        let author : &Signer = &ctx.accounts.author;
        let clock : Clock = Clock::get().unwrap();

        bet.author = *author.key;
        bet.timestamp = clock.unix_timestamp;
        bet.amount = amount;

        // This needs to be randomized
        bet.is_tail = true;

        if bet.is_tail == is_placed_on_tail 
        {
          bet.is_win = true;
           msg!("You have won the match");
        }
        else
        {
        msg!("You have lost the match");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
   #[account(init, payer = author, space = Bet::LEN)]
    pub bet: Account<'info, Bet>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Bet {
    pub is_tail : bool,
    pub author : Pubkey,
    pub amount : i64,
    pub is_win : bool,
    pub timestamp :  i64,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const AMOUNT_LENGTH: usize = 8;
const IS_WIN: usize = 8;
const TIMESTAMP_LENGTH: usize = 8;

impl Bet {
    const LEN: usize = DISCRIMINATOR_LENGTH
        +PUBLIC_KEY_LENGTH // Author.
        +AMOUNT_LENGTH // Bet amount
        +IS_WIN // Bet Result
        +TIMESTAMP_LENGTH ;// Timestamp.
}

#[error]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum.")]
    ContentTooLong,
}