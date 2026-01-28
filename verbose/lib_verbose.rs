use hex ;
use std::io::{Read,Error as ioError} ;
use std::error::Error ; 
// use core::fmt ; 

// use serde::{Serialize,Serializer} ; 
// use serde_json::{Result,Value} ; 

mod transaction ; 
use transaction::{TxIn, TxOut, Transaction, Amount, TxId} ; 



#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
fn read_version_from_str( txn_hex : &str ) -> u32 {

    // let mut vec = Vec::with_capacity(10) ; 

    let txn_bytes = hex::decode(txn_hex).unwrap() ;
    // let version_bytes = txn_bytes[0..4] ; // this slice represents dynamically sized data on heap - and that heap data can't be placed on our program stack - therefore throws "doesn't have a size at compile time" ioError
    
    // Data with known size at compile time -> Stack 
    // Dynamically sized data -> heap 
    // Can't store unknown sized data from heap onto stack 
    // Therefore point to the slice  

    // copying heap slice into array ( stack ) 
    let mut arr : [u8;4] = [0;4] ;
    arr.copy_from_slice(&txn_bytes[0..4]) ;
    
    //  Pointer reference to the heap slice - instead direct slice mention 
    let version_bytes : [u8;4] = <[u8;4]> :: try_from(&txn_bytes[0..4]).unwrap() ; // scope resolution / path separator should be there for every module used ! 
    
    // try_from(<T>) of U = try_into(U) for <T> 
    let version_bytes : [u8;4] = (&txn_bytes[0..4]).try_into().unwrap() ; // scope resolution / path separator should be there for every module used ! 
    // let le_version_bytes = u32::from_le_bytes(version_bytes) ;  // expects array of bytes of type u8 with constant length = 4 - [ u8 ; 4 ]

    // Buffer Read
    let mut buffer_slice = txn_bytes.as_slice() ; 
    let mut buffer = [0_u8 ; 4] ; 
    buffer_slice.read(&mut buffer).unwrap() ; 

    // println!("txn hex : {:?}" , txn_bytes) ; 
    // println!("version_bytes : {:?}" , version_bytes) ;
    // println!("le_version_bytes : {:?}" , le_version_bytes) ;

    u32::from_le_bytes(buffer)
}


fn read_u32_4bytes( txn_bytes_slice : &mut &[u8] ) -> Result<u32,ioError> {
    let mut buffer = [0_u8;4] ; 
    txn_bytes_slice.read(&mut buffer)? ; 
    
    Ok(u32::from_le_bytes(buffer))
}

fn read_amount( txn_bytes_slice : &mut &[u8] ) -> Result<Amount,ioError> {
    let mut buffer = [0_u8;8] ; 
    txn_bytes_slice.read(&mut buffer)?; 

    Ok(Amount::from_sat(u64::from_le_bytes(buffer)))
}

pub fn read_compact_size( txn_bytes_slice : &mut &[u8] ) -> Result< u64 , ioError > {
    let mut compact_size = [0_u8;1] ; 
    txn_bytes_slice.read(&mut compact_size)? ; // ? will the below match in the above return data type  

    // The about Result unwrap() will panic if the slice is of invalid data. 
    // Therefore ioError handling for that ioError should be implement w.r.t return type 
    // match (txn_bytes_slice.read(&mut compact_size)) { 
    //     Ok(t) => t , 
    //     Err(e) => return Err(e.into()) // Converting into ioError type of Result Enum 
    // }

    // 0 - `253     u8          1 bytes  
    // 253`         u16         2 bytes 
    // 254          u32         4 bytes 
    // 255          u64         8 bytes 

    // if(0..253).contains(compact_size[0]) {
    //     compact_size[0] as u64 
    // }
    // else if( compact_size[0] == 253 ) {
    //     let mut buffer = [0 ; 2] ; 
    //     txn_bytes_slice.read(&mut buffer)? ; 

    //     u16::from_le_bytes(buffer) as u64 
    // }
    // else if( compact_size[0] == 254 ) {
    //     let mut buffer = [0;4] ; 
    //     txn_bytes_slice.read(&mut buffer)? ; 

    //     u32::from_le_bytes(buffer) as u64
    // }
    // else{
    //     let mut buffer = [0;8] ; 
    //     txn_bytes_slice.read(&mut buffer)? ; 

    //     u64::from_le_bytes(buffer)
    // }


    match compact_size[0] { 
        0..=252 => {
            Ok(compact_size[0] as u64) 
        } , 
        253 => {
            let mut buffer = [0 ; 2] ; 
            txn_bytes_slice.read(&mut buffer)? ; 
    
            Ok(u16::from_le_bytes(buffer) as u64) 
        }, 
        254 => {
            let mut buffer = [0 ; 4] ; 
            txn_bytes_slice.read(&mut buffer)? ; 
    
            Ok(u32::from_le_bytes(buffer) as u64) 
        }, 
        255 => {
            let mut buffer = [0 ; 8] ; 
            txn_bytes_slice.read(&mut buffer)? ; 
    
            Ok(u64::from_le_bytes(buffer))
        }, 
    }
}

fn read_txid( txn_bytes_slice : &mut &[u8] ) -> Result<TxId,ioError> {
    let mut buffer = [0;32] ; // fixed size mutable array 
    txn_bytes_slice.read(&mut buffer)? ;  
    // Read trait always expects a slice / dynamically sized data  / unsized data 
    // while we're passing a fixed size mutable array 
    // Read trait implicitly does : 
    // Unsized coercion ( fixed size type --> unsized type )

    // buffer.reverse() ; 
    // hex::encode(buffer) 

    Ok(TxId::from_bytes(buffer))
}

fn read_script( txn_bytes_slice : &mut &[u8] ) -> Result<String,ioError> {
    let script_size = read_compact_size(txn_bytes_slice)? as usize ; // primitve type cast // can be script_sig / script_pubkey
    let mut buffer = vec!( 0_u8 ; script_size )  ; 
    txn_bytes_slice.read(&mut buffer)? ;
    // read expected = slice - passed = vec 
    // Read Trait does Implicit deref coercion ( Vec --> slice ) 
    // DerefMut : 
    // deref_mut( &mut self ) -> &mut [T] 
    //            &mut buffer -> &mut [buffer]

    Ok(hex::encode(buffer)) 
}

// fn txn_hash( txn_bytes_slice : &[u8] ) -> TxId {
//     let mut hasher = Sha256::new() ; 
//     hasher.update(txn_bytes_slice) ; 
//     let hash1 = hasher.finalize()  ; 
    
//     let mut hasher = Sha256::new() ; 
//     hasher.update(hash1) ;
//     let hash2 = hasher.finalize() ; 
    
//     TxId::from_bytes(hash2.into()) 
// }



// impl fmt::Debug for TxIn{
//     fn fmt( &self , f : &mut fmt::Formatter) -> fmt::Result{
//         f.debug_struct("TxIn") 
//             .field("txid" , &self.txid)
//             .field("vout_index" , &self.vout_index)
//             .field("script_sig" , &self.script_sig)
//             .field("sequence" , &self.sequence)
//             .finish() 
//     }
// }

// enum Fruit{
//     Banana(String) , 
//     Apple(String) , 
//     Orange(String)
// }

// fn calc_len( s : &mut String ) -> usize{
//     s.pop() ; 
//     s.len()
// }

pub fn decode( txn : String ) -> Result<String, Box<dyn Error> > {
    let txn_bytes = hex::decode(txn)? ; // Vec<u8>
    let mut txn_bytes_slice = txn_bytes.as_slice() ; // [u8]

    // ------------------------ Non Modular Implementation ---------------
    let version : u32 = read_u32_4bytes(&mut txn_bytes_slice)? ;  
    let vin_count : u64 = read_compact_size(&mut txn_bytes_slice)? ; 
    
    // println!("bytes_slice : {:?} " , txn_bytes_slice) ;
    // println!("main fn version : {}" , version );
    // println!("compact_size : {}" , vin_count );

    let mut inputs = vec![] ; 

    for _ in 0..vin_count {
        let txid : TxId  = read_txid(&mut txn_bytes_slice )?   ; 
        let vout_index : u32 = read_u32_4bytes(&mut txn_bytes_slice)? ;
        let script_sig : String  = read_script(&mut txn_bytes_slice)? ; 
        let sequence   : u32 = read_u32_4bytes(&mut txn_bytes_slice)? ; 
        
        inputs.push(
            TxIn{
                txid  , 
                vout_index   , 
                script_sig  , 
                sequence 
            }
        ); 

    }

    let mut outputs = vec![] ; 

    let vout_count = read_compact_size( &mut txn_bytes_slice )? ; 

    for _ in 0..vout_count{

        let amount = read_amount(&mut txn_bytes_slice)? ; 
        let script_pubkey = read_script( &mut txn_bytes_slice )? ; 
        
        outputs.push(
            TxOut{
                amount, 
                script_pubkey
            }
        ) ; 
    }

    let locktime : u32 = read_u32_4bytes(&mut txn_bytes_slice)? ; 
    let txn_id : TxId = txn_hash(&txn_bytes) ; 

    let transaction = Transaction{
        version , 
        vin  : inputs , 
        vout : outputs  , 
        locktime , 
        txn_id

    } ; 

    Ok(serde_json::to_string_pretty(&transaction)?)
    // ---------------------------------------------------------------
    
    
    
}
