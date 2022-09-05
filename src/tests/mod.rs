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
fn dump_script() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_script(
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
fn dump_input_cell() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell(
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
fn dump_input_cell_data() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_data(
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
fn dump_input_cell_data_hash() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_INPUT,
        CELL_FIELD_DATA_HASH,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_input_cell_capacity() {
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

#[test]
fn dump_input_cell_lock() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_INPUT,
        CELL_FIELD_LOCK,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_input_cell_lock_hash() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_INPUT,
        CELL_FIELD_LOCK_HASH,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_input_cell_type() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_INPUT,
        CELL_FIELD_TYPE,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_input_cell_type_hash() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_INPUT,
        CELL_FIELD_TYPE_HASH,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_input_cell_occupied_capacity() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_INPUT,
        CELL_FIELD_OCCUPIED_CAPACITY,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_output_cell() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_OUTPUT,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_output_cell_data() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_data(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_OUTPUT,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_output_cell_capacity() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_OUTPUT,
        CELL_FIELD_CAPACITY,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_output_cell_data_hash() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_OUTPUT,
        CELL_FIELD_DATA_HASH,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_output_cell_lock() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_OUTPUT,
        CELL_FIELD_LOCK,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_output_cell_lock_hash() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_OUTPUT,
        CELL_FIELD_LOCK_HASH,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_output_cell_type() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_OUTPUT,
        CELL_FIELD_TYPE,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_output_cell_type_hash() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_OUTPUT,
        CELL_FIELD_TYPE_HASH,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_output_cell_occupied_capacity() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_cell_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_OUTPUT,
        CELL_FIELD_OCCUPIED_CAPACITY,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_input() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_input(
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
fn dump_input_out_point() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_input_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_INPUT,
        INPUT_FIELD_OUT_POINT,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_input_since() {
    let (mut v, mut len) = get_buffer_with_length();
    let result = ckb_load_input_by_field(
        v.as_mut_ptr() as *mut c_void,
        &mut len as *mut usize as *mut u64,
        0,
        0,
        SOURCE_INPUT,
        INPUT_FIELD_SINCE,
    );
    dbg!(&result, &len);
    dbg_hex!(&v[..len]);
}

#[test]
fn dump_header() {
    let (mut v, mut len) = get_buffer_with_length();
    let sources = [
        SOURCE_INPUT,
        SOURCE_GROUP_INPUT,
        SOURCE_CELL_DEP,
        SOURCE_HEADER_DEP,
    ];
    for source in sources {
        let result = ckb_load_header(
            v.as_mut_ptr() as *mut c_void,
            &mut len as *mut usize as *mut u64,
            0,
            0,
            source,
        );
        dbg!(&result, &len);
        if result == 0 {
            dbg_hex!(&v[..len]);
        }
    }
}

#[test]
fn dump_header_by_field() {
    let (mut v, mut len) = get_buffer_with_length();
    let sources = [
        SOURCE_INPUT,
        SOURCE_GROUP_INPUT,
        SOURCE_CELL_DEP,
        SOURCE_HEADER_DEP,
    ];
    let header_fields = [
        HEADER_FIELD_EPOCH_NUMBER,
        HEADER_FIELD_EPOCH_START_BLOCK_NUMBER,
        HEADER_FIELD_EPOCH_LENGTH,
    ];
    for source in sources {
        for field in header_fields {
            let result = ckb_load_header_by_field(
                v.as_mut_ptr() as *mut c_void,
                &mut len as *mut usize as *mut u64,
                0,
                0,
                source,
                field,
            );
            dbg!(&result, &len);
            if result == 0 {
                dbg_hex!(&v[..len]);
            }
        }
    }
}
