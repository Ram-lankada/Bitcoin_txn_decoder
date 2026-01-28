use hex ;
use std::io::{Read} ;
use std::error::Error ; 
// use core::fmt ; 

// use serde::{Serialize,Serializer} ; 
// use serde_json::{Result,Value} ; 

mod transaction ; 
use self::transaction::{Decodable, Transaction} ; 



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

pub fn decode( txn : String ) -> Result<String, Box<dyn Error> > {
    let txn_bytes = hex::decode(txn)? ; // Vec<u8>
    let mut txn_bytes_slice = txn_bytes.as_slice() ; // [u8]

    let transaction = Transaction::consensus_decode(&mut txn_bytes_slice)? ;

    Ok(serde_json::to_string_pretty(&transaction)?) 
    
}
