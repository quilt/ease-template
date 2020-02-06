extern crate ease;
use ease::engine::Engine;
use wrapper::{build_state, ee_code, transfer};

#[test]
fn test() {
    // TODO

    let engine = Engine::new();
    // build initial state
    let state = build_state();
    // get generated ee wasm code
    let code = ee_code();
    // deploy code with state
    engine.deploy(code, state);
    // create a tx
    let tx = transfer(
        "zero".as_bytes().to_vec(),
        "one".as_bytes().to_vec(),
        100,
        1,
    );
    // execute tx on node
    engine.run(tx);
    // update local state
    //state.get_mut(Address::zero).set_balance(0);
    //state.get_mut(Address::one).set_balance(200);
    assert_eq!(1 + 1, 2);
}
