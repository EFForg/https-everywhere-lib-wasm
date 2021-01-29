use wasm_bindgen::prelude::*;
use js_sys::{Array, Number, JsString, Uint8Array};
pub use bloomfilter::Bloom as UpstreamBloom;

pub trait ToJavaScript {
    fn to_javascript(&self) -> JsValue;
}

/// A newtype for rulesets, wrapping all the JS functionality
#[wasm_bindgen]
pub struct Bloom(UpstreamBloom<str>);

#[wasm_bindgen]
impl Bloom {

    #[wasm_bindgen]
    /// Returns a new Bloom struct
    pub fn from_existing(bitmap: &Uint8Array, bitmap_bits: &Number, k_num: &Number, sip_keys: &Array) -> Bloom {
        let bitmap: &[u8] = &bitmap.to_vec();
        let bitmap_bits: u64 = if Number::is_integer(bitmap_bits) {
            bitmap_bits.value_of() as u64
        } else {
            panic!("bitmap_bits should be an integer");
        };
        let k_num: u32 = if Number::is_integer(k_num) {
            k_num.value_of() as u32
        } else {
            panic!("k_num should be an integer");
        };
        let sip_keys: [(u64, u64); 2] = if sip_keys.length() == 2 {
            let sip_keys_0 = sip_keys.get(0);
            let sip_keys_1 = sip_keys.get(1);
            if Array::is_array(&sip_keys_0) && Array::is_array(&sip_keys_1) {
                let sip_keys_0 = Array::from(&sip_keys_0);
                let sip_keys_1 = Array::from(&sip_keys_1);
                if sip_keys_0.length() == 2 && sip_keys_1.length() == 2 {
                    let sip_keys_0_0 = sip_keys_0.get(0);
                    let sip_keys_0_1 = sip_keys_0.get(1);
                    let sip_keys_1_0 = sip_keys_1.get(0);
                    let sip_keys_1_1 = sip_keys_1.get(1);
                    if sip_keys_0_0.is_string() &&
                       sip_keys_0_1.is_string() &&
                       sip_keys_1_0.is_string() &&
                       sip_keys_1_1.is_string() {
                        let sip_keys_0_0 = sip_keys_0_0.as_string().unwrap().parse::<u64>();
                        let sip_keys_0_1 = sip_keys_0_1.as_string().unwrap().parse::<u64>();
                        let sip_keys_1_0 = sip_keys_1_0.as_string().unwrap().parse::<u64>();
                        let sip_keys_1_1 = sip_keys_1_1.as_string().unwrap().parse::<u64>();
                        if sip_keys_0_0.is_ok() &&
                           sip_keys_0_1.is_ok() &&
                           sip_keys_1_0.is_ok() &&
                           sip_keys_1_1.is_ok() {
                            [(sip_keys_0_0.unwrap(),
                              sip_keys_0_1.unwrap()),
                             (sip_keys_1_0.unwrap(),
                              sip_keys_1_1.unwrap())]
                        } else {
                            panic!("sip_keys should be a nested array with 2 elements, each containing an array of length 2 of strings");
                        }
                    } else {
                        panic!("sip_keys should be a nested array with 2 elements, each containing an array of length 2 of strings");
                    }
                } else {
                    panic!("sip_keys should be a nested array with 2 elements, each containing an array of length 2");
                }
            } else {
                panic!("sip_keys should be a nested array with 2 elements, each containing an array");
            }
        } else {
            panic!("sip_keys should be an array of length 2");
        };
        log::info!("{:?}", bitmap);
        log::info!("{:?}", bitmap_bits);
        log::info!("{:?}", k_num);
        log::info!("{:?}", sip_keys);

        Bloom(UpstreamBloom::from_existing(bitmap, bitmap_bits, k_num, sip_keys))
    }

    #[wasm_bindgen]
    pub fn check(&self, item: &JsString) -> bool {
        self.0.check(&item.as_string().unwrap())
    }
}
