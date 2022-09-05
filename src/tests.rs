use crate::*;

#[test]
fn dump_witness() {
    let mock_tx: MockTransaction = {
        let tx_filename = std::env::var("CKB_TX_FILE").expect("environment variable");
        let tx_content = std::fs::read_to_string(tx_filename).expect("read tx file");
        let repr_mock_tx: ReprMockTransaction =
            serde_json::from_str(&tx_content).expect("parse tx file");
        repr_mock_tx.into()
    };
    dbg!(mock_tx.core_transaction());
    const SIZE: usize = 256;
    let mut v: [u8; SIZE] = [0; SIZE];
    let mut len: u64 = SIZE as u64;
    let result = ckb_load_witness(v.as_mut_ptr() as *mut c_void, &mut len as *mut u64, 0, 0, 1);
    dbg!(&result, &v, &len);
}
