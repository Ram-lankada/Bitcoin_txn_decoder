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

    // let version : u32 = read_version_from_str("0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000") ; 
    
    // let txn : &str = "0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000" ;
    // let txn : &str = "01000000017642bfe0fc3fc8766a8bfde98343a8e6859c394537a0818d40ab4158be0a1cab130000006a4730440220382ccdd5202b5d6f01cebdb99a3e8c4b1938ea33ed420500656131df7d51333a02206a2dd40ce559e14ba7b428f9b8767d67869007707db346943a9aacd9f077d14e012103680e8dcef369b9bd925951b1183fe8394066d9914fb85acfc0c0822429a53341ffffffff33e8030000000000001976a914fd601e959385142e3f475fa9b4ad2e1d99b545c588ace8030000000000001976a914acbfcee7ee1deab6e9540d14155b252c7f1ac13c88ace8030000000000001976a9142ab30007b1143a03ccf5278442c6928b3e71901488ace8030000000000001976a914fdec3415f290de57ab8656a0ccde1b762fec3d5988ace8030000000000001976a91424850b9dca74256d9b802ecb4aa8436f8e70f3c688ace8030000000000001976a9146d2b7dedf739e0b588a3cac71e4bf08b4265819c88ace8030000000000001976a9140ace4038933494781df4004d16886e84f25e2c5d88ace8030000000000001976a9144ff640406e2bb42b5ed0d9070f38377f0784b31f88ace8030000000000001976a914d75de3825dae4160f8bc9521ab9d4ecfe379a2b288ace8030000000000001976a9148cc6b3e4ecd7725e57bcdbd5aa6d97c34911d1a188ace8030000000000001976a914aee7ca076b8535a490338c2f9b0ca3c67921bd0d88ace8030000000000001976a914f30d3f8b603c095e2c069b91ea712eaf9e02d62588ace8030000000000001976a914750e78d12971fe493f9434bd0068701cc8ebf29088ace8030000000000001976a9148358f74edcfcae1efcca0983e2bd5578b33abbf188ace8030000000000001976a91432485e7d2afbf99f33afb9d88453a0494f478eb188ace8030000000000001976a914a7892fa244e295f746f62adbce56556488e0a2e588ace8030000000000001976a914639d0552d366e07f479b5f8d224232d6c99b127c88ace8030000000000001976a9140afce59e4196430fb9bba000ddf37e0c2eebe90b88ace8030000000000001976a914036f8dcef01e6c3ee90cc928aece4831f06adb9e88ace8030000000000001976a9141e09bf77145900bc3b33ff283ebadf234ab2297088ace8030000000000001976a9146950f036f3b2943f8edb3e53bb146129d0c021a488ace8030000000000001976a9145a33d89da024b56d5e7da463c5fcc032088e8a8288ace8030000000000001976a9144dd14a23b5ba519e1074a0c86df12e4ca175f85688ace8030000000000001976a9141150ca680b086e528b3f0d33927bb92f1a6f0b8888ace8030000000000001976a9148865580ec997f1507bf1e0dacb76dfe46220e70c88ace8030000000000001976a914f54657c745bb7ac2cc364c3a7f6e3c283d57379d88ace8030000000000001976a914b585c8d58443f84fe6f152dbb7f84578935b9c7488ace8030000000000001976a914b73a0c5d8e72f938906c71ff1cbc7259e683a4ca88ace8030000000000001976a914ce41196fb5e1f86fc3d9da21a0f8e2a058a62f1a88ace8030000000000001976a91441980114fc5d047283dcb63705f97e40e2702a2a88ac84f53703000000001976a91484a6562983e91feecb1c8daaacfcdaa433c648a288ace8030000000000001976a9141fd9dfe62940a2602533a0b0cc22d725f0f57c7888ace8030000000000001976a91467b8703ef254b100e837f1248bcb2649fed424aa88ace8030000000000001976a9141e423577925fb6069aa4eea362c04514e0b631d588ace8030000000000001976a914b6b0027b872c1e58ef06243361e4d3090d87352888ace8030000000000001976a9144a96fc718f4130daad0b31c7b33bdc1e2c7153f088ace8030000000000001976a9146f7d6f30c27ade0814588609a8c771381e45e5ed88ace8030000000000001976a914ed9834b6f237a471e35a44a1303b1e82f94d1e0788ace8030000000000001976a91401b629752904900e2b6c2aba7a7c6091cde5521888ace8030000000000001976a914722ea8e067d10cdf1a1af3e523d30baa93fd101788ace8030000000000001976a91416a8553d55761ac913eeabefe582c611318a073088ace8030000000000001976a914ee9cc7f9f7c404c38f6a26dcc11237c65f31c16488ace8030000000000001976a9148b9004f85b5ad8447f82e7dfc2c0845d09ad263888ace8030000000000001976a914ba6bd5985da7ac1bbc09bd1743499d8be1f549b788ace8030000000000001976a914a2fa99e1c41baa736eb7c0b0e5dfdfce2eb981d288ace8030000000000001976a914668503cae3537086a4c21d409d647124876a9f6d88ace8030000000000001976a9147d3b15ed2d58aff2549996f4686e482923bb387f88ace8030000000000001976a9143ee8899f4860c643c66a887f4567f3e005237e6d88ace8030000000000001976a914611107e9607fd9c54ee7507586128b7725efcff788ace8030000000000001976a91407a9a338e50edd8f3f8a6698397c2bfe41926d7b88ace8030000000000001976a914a22a029e45bd5e791c16520baebbfa07c302936588ac00000000" ; 

    let cli = Cli::parse() ; 

    match txn_decoder::decode(cli.txn_hex) {
        Ok(json) => println!("Transaction : {}" , json ) , 
        Err(e) => println!("Error : {}" , e )  
    }

    // // Builder CLI 
    // let matches = Command::new("Transaction Decoder")
    //     .version("1.0")
    //     .about("Bitcoin Transaction Decoder")
    //     .arg(Arg::new("transaction_hex")
    //             .required(true)
    //             .help("(string, required) Raw transaction hex"))
    //     .get_matches();

    // let transaction_hex = matches
    //     .get_one::<String>("transaction_hex")
    //     .expect("required")
    //     .to_string();

    // match transaction_decoder::decode_txn(transaction_hex) {
    //     Ok(json) => println!("{}", json),
    //     Err(e) => eprintln!("{}", e)
    // }
    
    // return Ok(()) // Returning empty tuple



    // let mut vec = Vec::new() ; 
    // vec.push(1); 
    // vec.push(2); 

    // let mut vecc = vec![1,2,3] ; 

    // println!("vec len : {}" , vec.len() ) ; 
    // println!("vec len : {}" , vecc.len() ) ;

    // let fruit = Fruit::Orange("ripe".to_string()) ; 

    // if let Fruit::Banana(adj) = banana { // binding the content of banana into new enum 
    //     println!("Adjective is {}" , adj ) ; 
    // }

    // match fruit {
    //     Fruit::Banana(adj) => println!("matched is banana") , 
    //     Fruit::Apple(adj) => println!("matched : Fruit") , 
    //     _ => println!("matched rest ")
    // }

    // // Result Enum 
    // let x : Result< i32 , &str > = Ok(-2) ; 
    // let y : i32 = 4 ; 

    // println!("the sum of x & y is : {}" , x? + y ) ; 

    // let vec = vec![1,2,3,4] ; 
    // let mut arr = [ 0_u8 ; 2 ] ; 
    
    // arr.copy_from_slice((&vec[0..2])) ; 
    // // if length mis-matched - exec will be panicked 
    // // to avoid we can use Result enum
    // // But the prog is in rapid 
    // // which will still throw the panick --> therefore use traits 

    // // let x : Result< [ u8 ; 2 ] , &str > = Ok(&vec[0..2]) ; 

    // println!("{:?}" , arr ) ; 


    // // Shared ownership - moved ownership - mutable referencing 
    // let mut s1 : String = "string".to_string() ; 
    // let len = calc_len( &mut s1 ) ; 
    // println!("length of {} is {:?}" , s1, len ) ; 

    // // Moving ownership isn't applied for array as it's completely stack allocated and Copy trait is impl on all stack allocated elements 
    // // While String is stored on the Heap and in-order to avoid shared ownership and dangling pointers - ownership is moved 
    // // And we handle it using mutable reference. 

    // // At any given time we can have only : 
    // // 1 Mutable reference - 1 writer 
    // // Multiple shared reference - multiple readers 

    // // Mutable borrow on Immutable Borrow 
    // // Can't read and write to the loop at the same time 
    // let mut arr = vec![1,2,3,4]; 

    // for &e in &arr { // cannot borrow `arr` as mutable because it is also borrowed as immutable
    //     if e%2 == 0 { 
    //         arr.push(e+1) ; // mutable borrow 
    //     }
    // }

    // // arr will be immutable while reading 
    // // and you're mutating in that immutable borrow. 

}



#[cfg(test)]
mod test{
    use txn_decoder::read_compact_size ; 
    use std::error::Error ; 

    #[test]
    fn test_compact_size() -> Result< () , Box< dyn Error > > {
        let mut byte = [1_u8].as_slice() ; 
        let count = read_compact_size(&mut byte )? ; 
        
        assert_eq!(count , 1_u64) ;

        let mut bytes = [253_u8 , 1 , 1].as_slice() ; 
        let count = read_compact_size(&mut bytes)? ; 

        assert_eq!(count , 257_u64) ;

        let hex = "fd204e" ; 
        let decoded = hex::decode(hex)? ; 
        let mut bytes = decoded.as_slice() ; 
        let count = read_compact_size(&mut bytes)? ; 

        assert_eq!(count,20_000_u64) ; 
        Ok(())
    }
}