mod utils;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, crc32-wasm!");
// }

#[wasm_bindgen]
pub fn crc32fast(mgs:String) ->String{
    let checksum = crc32fast::hash(mgs.as_bytes());
    return  format!("{}",checksum);
}


#[cfg(test)]
mod tests {
    use crate::{ crc32fast};
    #[test]
    fn test_crc32fast() {
        let body="00.8.80\x0195\x010000000172query_history_k_data_plus\x01anonymous\x011\x0110000\x01sh.600029\x01code,date,open,high,low,close,preclose,volume,amount,adjustflag,turn,tradestatus,pctChg,isST\x012023-11-01\x012023-11-21\x01d\x013";
        let checksum =   crc32fast(body.to_string());
        println!("{}",checksum)
     
    }

   
}
