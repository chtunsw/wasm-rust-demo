// export function from the produced library
#[no_mangle]
pub fn wasm_add(left: i32, right: i32) -> i32 {
    left + right
}
