use hex ;
use serde::{ser::{SerializeStruct,SerializeSeq},Serialize,Serializer} ; 
use sha2::{Digest, Sha256, digest::core_api::Buffer} ; 
use std::{io::{BufRead,Write}} ; 

// use serde_json::{Result,Value} ; 

// use std::io::{BufRead,Error as ioError} ;
// use std::error::Error ; 
use core::fmt ; 

#[derive(Debug)]
pub enum Error{
     Io( std::io::Error ) , 
     UnsupportedSegwitFlag(u8) , 
     ParseFailed(& 'static str)
}

impl fmt::Display for Error {
    fn fmt( &self , f: &mut fmt::Formatter ) -> fmt::Result{
        match *self{
            Error::Io(ref e) => write!( f , " IO Error : {} " , e ) , 
            Error::UnsupportedSegwitFlag(swflag) => write!( f , "Unsupported segwit version {}" , swflag) , 
            Error::ParseFailed(wit_msg) => write!(f , "Witness Parse failed : {}" , wit_msg )

        }
    }
}

impl std::error::Error for Error{}


#[derive(Debug)]
pub struct TxId([u8;32]) ; 

impl TxId{
    // pub fn from_bytes( txn_bytes : [u8;32] ) -> TxId{
    //     TxId(txn_bytes)
    // }

    pub fn from_hash( txn_bytes_hash : [u8;32] ) -> TxId{
        TxId(txn_bytes_hash) 
    }

    pub fn from_raw_txn( txn_bytes : Vec<u8> ) -> TxId{
        let mut hasher = Sha256::new() ; 
        hasher.update(txn_bytes) ; 
        let hash1 = hasher.finalize() ; 

        let mut hasher = Sha256::new() ; 
        hasher.update(hash1) ; 
        let hash2 = hasher.finalize() ; 

        TxId::from_hash(hash2.into())
    }
}

// trait EndianFormat{
//     fn as_be(&self) -> String ; 
// }

// impl EndianFormat for TxId{
//     fn as_be(&self) -> String{
//         let mut bytes = self.0.clone() ; 
//         bytes.reverse() ;  
//         hex::encode(bytes) 
//     }
// }

// fn as_be<S: Serializer, T: EndianFormat>( t: &T , s: S ) -> Result< S::Ok , S::Error> {
//     let be_txid = t.as_be() ;
//     s.serialize_str(&be_txid)
// }

impl Serialize for TxId{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error>{
        let mut bytes = self.0.clone() ; 
        bytes.reverse() ;  
        s.serialize_str(&hex::encode(&bytes))
    }
}

#[derive(Debug)]
pub struct Amount(u64) ; 

impl Amount{
    pub fn from_sat( satoshi : u64 ) -> Amount { 
        Amount(satoshi) 
    }
}

// impl Amount {
//     pub fn to_btc(&self) -> f64 { 
//         self.0 as f64 / 100_000_000.0 
//     }
// }


trait BitcoinValue { 
    fn to_btc(&self) -> f64 ; 
}

impl BitcoinValue for Amount { 
    fn to_btc(&self) -> f64{ 
        self.0 as f64 / 100_000_000.0
    }
}

// Custom serializer 
fn as_btc<S:Serializer , T: BitcoinValue>( t: &T , s: S ) -> Result< S::Ok , S::Error > { 
    // there's no trait mentioned explicitly , therefore to_btc() will be applied on a generic type rather than the Amount. 
    // Therefore trait should be explicity mentioned for Generic type < T > 
    let btc = t.to_btc() ; 
    s.serialize_f64(btc)
}


#[derive(Debug)]
pub struct TxIn {
    // #[serde(serialize_with="as_be")]
    pub txid : TxId , 
    pub vout_index : u32 , 
    pub script_sig : String , 
    pub witness    : Witness , 
    pub sequence   : u32 
}

impl Serialize for TxIn {
    fn serialize<S: Serializer>( &self , s: S ) -> Result< S::Ok , S::Error > {
        let mut txin = s.serialize_struct("TxIn", 5)? ; 
        txin.serialize_field("txid", &self.txid )? ; 
        txin.serialize_field("vout_index", &self.vout_index )? ; 
        txin.serialize_field("script_sig", &self.script_sig )? ; 
        if !&self.witness.content.is_empty(){
            txin.serialize_field("txinwitness", &self.witness )? ; 
        }
        txin.serialize_field("sequence", &self.sequence )? ;
        txin.end() 
    }

}
#[derive(Debug)]
pub struct Witness{
    pub content : Vec<Vec<u8>>
}

impl Witness{
    pub fn new() -> Self { 
        Witness { content: vec![] }
    }

    // self.content.is_empty() can be directly used in other impl's 
    // pub fn is_empty(&self) -> bool {
    //     self.content.is_empty() 
    // }

}

#[derive(Debug,Serialize)]
pub struct TxOut{
    #[serde(serialize_with="as_btc")]
    pub amount : Amount, 
    pub script_pubkey : String
}

// #[derive(Debug,Serialize)] // impl - ing custom Serialize trait for Transaction as tx-id has to serialized in a custom manner for legacy & segwit 
#[derive(Debug)]
pub struct Transaction{
    // #[serde(serialize_with="as_be")]
    // pub txn_id   : TxId, // commenting out as TxId is different for Legacy and Segwit txns  
    pub version  : u32 , 
    pub vin      : Vec<TxIn> , 
    pub vout     : Vec<TxOut> , 
    pub locktime : u32 , 
}

impl Transaction{
    pub fn compute_txid( &self ) -> TxId{
        let mut txn = Vec::new() ;
        self.version.consensus_encode(&mut txn).expect("Writing version to a vec shouldn't fail") ;
        self.vin.consensus_encode(&mut txn).expect("Writing vin to a vec shouldn't fail") ;
        self.vout.consensus_encode(&mut txn).expect("Writing vout to a vec shouldn't fail") ;
        self.locktime.consensus_encode(&mut txn).expect("Writing locktime to a vec shouldn't fail") ;

        TxId::from_raw_txn(txn)
    }
}

impl Serialize for Transaction {
    fn serialize<S: Serializer>( &self , serializer: S) -> Result< S::Ok , S::Error > {
        let mut tx = serializer.serialize_struct( "Transaction" , 5 )? ; 
        tx.serialize_field("txn_id",&self.compute_txid())? ; 
        tx.serialize_field("version",&self.version)? ; 
        tx.serialize_field("vin",&self.vin  )? ; 
        tx.serialize_field("vout",&self.vout)? ; 
        tx.serialize_field("locktime",&self.locktime)? ; 
        tx.end()
    }
}


pub trait Decodable : Sized {
    fn consensus_decode< R: BufRead + ?Sized >( r : &mut R ) -> Result< Self , Error > ;
}

impl Decodable for u8 { 
    fn consensus_decode< R: BufRead + ?Sized >( r : &mut R ) -> Result< Self , Error > {
        let mut buffer = [0_u8 ; 1] ; 
        r.read_exact(&mut buffer).map_err(Error::Io)? ;
        Ok(u8::from_le_bytes(buffer)) 
    }
}

impl Decodable for u16 {
     fn consensus_decode< R: BufRead + ?Sized >( r : &mut R ) -> Result< Self , Error > {
        let mut buffer = [0 ; 2] ;
        r.read_exact(&mut buffer).map_err(Error::Io)? ; 
        Ok(u16::from_le_bytes(buffer))
    }
}

impl Decodable for u32 { // Equivalent to read_u32_4bytes
    fn consensus_decode< R: BufRead + ?Sized >( r : &mut R ) -> Result< Self , Error > {
       let mut buffer = [0 ; 4] ; 
       r.read_exact(&mut buffer).map_err(Error::Io)? ;
       Ok(u32::from_le_bytes(buffer))
   }
}

impl Decodable for u64 { // Equivalent to read_u64_8bytes
    fn consensus_decode< R: BufRead + ?Sized >( r : &mut R ) -> Result< Self , Error > {
       let mut buffer = [0 ; 8] ;
       r.read_exact(&mut buffer).map_err(Error::Io)? ;
       Ok(u64::from_le_bytes(buffer))
   }
}

pub struct CompactSize( pub u64 ) ; 

impl Decodable for CompactSize{

    fn consensus_decode< R: BufRead + ?Sized >( r: &mut R ) -> Result< Self , Error > {

        let n = u8::consensus_decode(r)? ; 
        
        match n{
            0xFD => {
                let x = u16::consensus_decode(r)? ; 
                Ok(CompactSize(x as u64) )  
            }, 
            0xFE => {
                let x = u32::consensus_decode(r)? ; 
                Ok( CompactSize(x as u64) ) 
            },
            0xFF => {
                let x = u64::consensus_decode(r)? ; 
                Ok(CompactSize(x)) 
            }, 
            n => Ok(CompactSize(n as u64))
        }
    }
}


impl Decodable for String{
    fn consensus_decode< R: BufRead + ?Sized>( r: &mut R ) -> Result< Self , Error > {
        let len: usize = CompactSize::consensus_decode(r)?.0 as usize ; 
        let mut buffer: Vec<u8> = vec![0;len] ; 
        r.read_exact(&mut buffer).map_err(Error::Io)? ; 

        Ok(hex::encode(buffer)) 
    }
}

impl Decodable for TxId{
    fn consensus_decode< R: BufRead + ?Sized >( r: &mut R ) -> Result< Self , Error > {
        let mut buffer = [0;32] ; 
        r.read_exact(&mut buffer).map_err(Error::Io)? ; 

        Ok(TxId(buffer)) 
    }
}

impl Decodable for Witness {
    fn consensus_decode< R: BufRead + ?Sized >( r : &mut R ) -> Result< Self , Error > {
        let mut witness_items = vec![] ; 
        let stack_count = u8::consensus_decode(r)? ;

        for _ in 0..stack_count{
            let len = CompactSize::consensus_decode(r)?.0 ; 
            // CompactSize::consensus_decode(r) in-turn returns a CompactSize as CompactSize(5 or any num )  
            // Therefore we're un-wrapping it using .0  
            // let mut buffer = Vec::with_capacity(len as usize) ; // Error :  IO Error : failed to fill whole buffer 
            let mut buffer = vec![0; len as usize] ; 

            r.read_exact(&mut buffer).map_err(Error::Io)? ; 
            witness_items.push(buffer);
        }

        Ok(Witness { content: witness_items })
    }
}

impl Serialize for Witness {
    fn serialize<S: Serializer >(&self, s: S) -> Result<S::Ok, S::Error>{
        // let mut witness = self.content  ; 
        // // create a buffer of vec< vec<u8> > 
        // // clone the witness into buffer 
        // // iter_mut -> hex::encode -> serialize_str 

        // for &mut wit in witness.iter_mut() {
        //     wit = s.serialize_str(&hex::encode( &wit ) ) ; 
        // }
        
        // // s.serialize_str(buffer) ; 
        // s.serialize_struct(witness)

        let mut seq = s.serialize_seq(Some(self.content.len()))? ; 
        
        // for wit in &self.content {
        //     let hex = hex::encode(wit) ; 
        //     let _ = seq.serialize_element(&hex) ;
        // }

        for wit in self.content.iter() {
            seq.serialize_element(&hex::encode(&wit))? ; 
        }

        seq.end() 

    }
}

impl Decodable for TxIn {
    fn consensus_decode< R: BufRead + ?Sized>( r: &mut R ) -> Result< Self , Error > {
        Ok(
            TxIn{
                txid : TxId::consensus_decode(r)? , 
                vout_index : u32::consensus_decode(r)? ,
                script_sig : String::consensus_decode(r)? , 
                witness    : Witness::new() , 
                sequence   : u32::consensus_decode(r)? 
            }
        )
    }
}

impl Decodable for Vec<TxIn> {
    fn consensus_decode< R: BufRead + ?Sized >( r: &mut R ) -> Result< Self , Error > {
        let vin_count = CompactSize::consensus_decode(r)?.0 ; 
        let mut inputs = Vec::with_capacity(vin_count as usize) ; 
        for _ in 0..vin_count {
            inputs.push(TxIn::consensus_decode(r)?) ;
        }

        Ok(inputs) 
    }
}

impl Decodable for TxOut{
    fn consensus_decode< R: BufRead + ?Sized >( r: &mut R ) -> Result< Self , Error > {
        Ok( 
            TxOut{
                amount : Amount::from_sat(u64::consensus_decode(r)?),
                script_pubkey : String::consensus_decode(r)?
            }
        )
    }
}

impl Decodable for Vec<TxOut>{
    fn consensus_decode< R: BufRead + ?Sized >( r: &mut R ) -> Result< Self , Error > {
        let vout_count = CompactSize::consensus_decode(r)?.0 ; 
        let mut outputs = Vec::with_capacity( vout_count as usize) ; 
        for _ in 0..vout_count{
            outputs.push(TxOut::consensus_decode(r)?) ; 
        }
        Ok(outputs) 
    }
}

impl Decodable for Transaction{
    fn consensus_decode< R: BufRead + ?Sized>( r: &mut R ) -> Result< Self , Error > {

        let version = u32::consensus_decode(r)? ; 
        let inputs = Vec::<TxIn>::consensus_decode(r)? ; 

        if inputs.is_empty() {
            let segwit_flag = u8::consensus_decode(r)? ; 
            match segwit_flag{
                1 => {
                    let mut inputs = Vec::<TxIn>::consensus_decode(r)? ; 
                    let outputs = Vec::<TxOut>::consensus_decode(r)? ; 
                    for txin in inputs.iter_mut() {
                        txin.witness = Witness::consensus_decode(r)? 
                    }
                    if !inputs.is_empty() && inputs.iter().all( |input| input.witness.content.is_empty() ) 
                    {
                        Err( Error::ParseFailed("Witness flag set but no witnesses present ! ") ) 
                    }
                    else {
                        Ok(
                            Transaction { 
                                version, 
                                vin: inputs, 
                                vout: outputs, 
                                locktime: u32::consensus_decode(r)? 
                            }
                        )
                    }
                }, 
                x=> Err(Error::UnsupportedSegwitFlag(x))
            }
        }
        else{
            Ok(
                Transaction{
                    version   , 
                    vin      : inputs , 
                    vout     : Vec::<TxOut>::consensus_decode(r)? , 
                    locktime : u32::consensus_decode(r)?
                }
            )
        }
    }
}

trait Encodable{
    fn consensus_encode< W : Write >( &self, w: &mut W ) -> Result< usize , Error > ;
}

impl Encodable for u8{
    fn consensus_encode< W : Write >( &self, w: &mut W ) -> Result< usize , Error > {
        let len = w.write([*self].as_slice()).map_err(Error::Io)? ; 
        Ok(len)
    }
}

// impl Encodable for u16{
//     fn consensus_encode< W : Write >( &self, w: &mut W ) -> Result< usize , Error > {
//         let bytes = self.to_le_bytes() ; 
//         let len = w.write(bytes.as_slice()).map_err(Error::Io)? ; 
//         Ok(len)
//     }
// }

// Macro to generate Encodable implementations for integer types using to_le_bytes
macro_rules! impl_encodable_int {
    ($($t:ty),*) => {
        $(
            impl Encodable for $t {
                fn consensus_encode< W : Write >( &self, w: &mut W ) -> Result< usize , Error > {
                    let bytes = self.to_le_bytes() ; 
                    let len = w.write(bytes.as_slice()).map_err(Error::Io)? ; 
                    Ok(len)
                }
            }
        )*
    };
}

impl_encodable_int!(u16, u32, u64);

impl Encodable for [u8;32] {
    fn consensus_encode< W : Write >( &self, w: &mut W ) -> Result< usize , Error > {
        let len = w.write(self).map_err(Error::Io)? ; 
        Ok(len)
    }
}

impl Encodable for CompactSize {
    fn consensus_encode< W : Write >( &self, w: &mut W ) -> Result< usize , Error > {
        match self.0 { 
            0..=0xFC => {
                (self.0 as u8).consensus_encode(w)? ; 
                Ok(1) 
            }, 
            0xFD..=0xFFFF=>{
                w.write([0xFD].as_slice()).map_err(Error::Io)? ; 
                (self.0 as u16).consensus_encode(w)? ; 
                Ok(3)
            }, 
            0x100000..=0xFFFFFFFF=>{
                w.write([0xFE].as_slice()).map_err(Error::Io)? ; 
                (self.0 as u32).consensus_encode(w)? ; 
                Ok(5)
            }, 
            _ => {
                w.write([0xFF].as_slice()).map_err(Error::Io)? ; 
                (self.0).consensus_encode(w)? ; 

                Ok(9) 
            }
        }
    }
}

impl Encodable for String {
    fn consensus_encode< W : Write >( &self, w: &mut W ) -> Result< usize , Error > {
        let b = hex::decode(self).expect("Valid Hex String needed!") ; 
        let compact_size_len = CompactSize(b.len() as u64).consensus_encode(w)? ; 
        let b_len = w.write(&b).map_err(Error::Io)? ; 
        Ok(compact_size_len + b_len)
        
    }
}

impl Encodable for TxId {
    fn consensus_encode< W : Write >( &self, w: &mut W ) -> Result< usize , Error > {
        Ok(self.0.consensus_encode(w)?) 
        // Why self.0 for a [u8;32] ? 
    }
}

impl Encodable for TxIn {
    fn consensus_encode< W : Write >( &self, w: &mut W ) -> Result< usize , Error > {
        let mut len = 0 ; 
        len += self.txid.consensus_encode(w)? ; 
        len += self.vout_index.consensus_encode(w)? ; 
        len += self.script_sig.consensus_encode(w)? ; 
        len += self.sequence.consensus_encode(w)?   ;

        Ok(len) 
    }
}

impl Encodable for Vec<TxIn> {
    fn consensus_encode< W : Write >( &self, w: &mut W ) -> Result< usize , Error > {
        let mut len = 0 ; 
        len += CompactSize(self.len() as u64 ).consensus_encode(w)? ; 
        for tx in self.iter(){
            len += tx.consensus_encode(w)? ; 
        }

        Ok(len) 
    }
}

impl Encodable for Amount {
    fn consensus_encode< W : Write >( &self, w: &mut W ) -> Result< usize , Error > {
        let len = (self.0).consensus_encode(w)?  ; 
        // Why not mut here ? 
        // coz owner ship of len is not being moved by doing contious on-the-go addtion 
        // which is the case in vecs 
        Ok(len)
    }
}

impl Encodable for TxOut {
    fn consensus_encode< W : Write >( &self, w: &mut W ) -> Result< usize , Error > {
        let mut len = 0 ; 
        len += self.amount.consensus_encode(w)? ; 
        len += self.script_pubkey.consensus_encode(w)? ; 

        Ok(len)
    }
}

impl Encodable for Vec<TxOut> {
    fn consensus_encode< W : Write >( &self, w: &mut W ) -> Result< usize , Error > {
        let mut len = 0 ; 
        len += CompactSize( self.len() as u64).consensus_encode(w)? ; 

        for tx in self.iter() {
            len += tx.consensus_encode(w)? ; 
        }

        Ok(len)
    }
}