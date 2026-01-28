use clap::{Parser} ; 
// use clap::{Arg, Command};

#[derive(Parser)]
#[command(name="Bitcoin Transaction Decoder")]
#[command(version="1.0.0")]
#[command(about="Bitcoin Transaction Decoder")]
struct Cli {
    #[arg(
        required = true , 
        help = "Required transaction hex" 
    )]
    txn_hex : String  
}



fn main() { 
    let cli = Cli::parse() ; 

    match txn_decoder::decode(cli.txn_hex) {
        Ok(json) => println!("Transaction : {}" , json ) , 
        Err(e) => println!("Error : {}" , e )  
    }


}



// #[cfg(test)]
// mod test{
//     use txn_decoder::read_compact_size ; 
//     use std::error::Error ; 

//     #[test]
//     fn test_compact_size() -> Result< () , Box< dyn Error > > {
//         let mut byte = [1_u8].as_slice() ; 
//         let count = read_compact_size(&mut byte )? ; 
        
//         assert_eq!(count , 1_u64) ;

//         let mut bytes = [253_u8 , 1 , 1].as_slice() ; 
//         let count = read_compact_size(&mut bytes)? ; 

//         assert_eq!(count , 257_u64) ;

//         let hex = "fd204e" ; 
//         let decoded = hex::decode(hex)? ; 
//         let mut bytes = decoded.as_slice() ; 
//         let count = read_compact_size(&mut bytes)? ; 

//         assert_eq!(count,20_000_u64) ; 
//         Ok(())
//     }
// }