

pub fn ee_code() -> Vec<u8> {
    let ret = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/wasm32-unknown-unknown/debug/ee.wasm"
    ));  // TODO: remove hardcoded path to debug
    ret.to_vec()
}

pub fn transfer(to: Vec<u8>, from: Vec<u8>, amount: u64, nonce: u64) -> Vec<u8> {
    // TODO
    let tx = "bar".as_bytes().to_vec();
    return tx;
}

pub fn build_state() -> Vec<u8> {
    // TODO
    let state = "foobar".as_bytes().to_vec();
    return state;
}