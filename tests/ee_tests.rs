use wrapper::{ee_code,transfer,build_state};
use ease::{primitives,engine};

#[test]
fn test() {
    // TODO

    // build initial state
    let state = build_state();
    // get generated ee wasm code
    let code = ee_code();
    // deploy code with state
    // engine.deploy(code,state);


    assert_eq!(1+1, 2);
}