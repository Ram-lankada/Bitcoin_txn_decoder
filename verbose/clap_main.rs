use clap::{Parser,Subcommand} ; 

#[derive(Parser)]
#[command(name="Bitcoin CLI")]
#[command(version="1.0.0")]
#[command(about="Bitcoin CLI about")]
struct Cli{
    #[command(subcommand)]
    command : Option<Commands> 
}


#[derive(Subcommand)]
enum Commands{
    /// Returns the block height of Bitcoin  
    GetBlockHash{
        #[arg(
            required = true , 
            help = "(required integer)"
        )]
        height : u64
    }
}


fn main() {
    // let cli = Cli::Parse ;
    // The Trait name shouldn't be mentioned - since it is being used as a fn on top of the struct  
    let cli = Cli::parse() ; 

    match &cli.command {
        Some( Commands::GetBlockHash {height} ) => {
            println!("Return the block hash of height {height:?}")
        }, 
        None => {
            println!("Error") ; 
        }
    }
}