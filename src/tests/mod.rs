use crate::*;
use dbg_hex::dbg_hex;
const BUFFER_SIZE: usize = 512;

fn get_buffer_with_length() -> ([u8; BUFFER_SIZE], usize) {
    ([0; BUFFER_SIZE], BUFFER_SIZE)
}

#[test]
fn dump_mocked_transaction() {
    let mock_tx: MockTransaction = {
        let tx_filename = std::env::var("CKB_TX_FILE").expect("environment variable");
        let tx_content = std::fs::read_to_string(tx_filename).expect("read tx file");
        let repr_mock_tx: ReprMockTransaction =
            serde_json::from_str(&tx_content).expect("parse tx file");
        repr_mock_tx.into()
    };
    dbg!(mock_tx.core_transaction());
}

#[test]
fn dump_tx() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_transaction(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_tx_hash() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_tx_hash(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_script_hash() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_script_hash(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_witness() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_witness(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_INPUT,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_cell_by_field() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_INPUT,
        CELL_FIELD_CAPACITY,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}
