
pub fn get_indices_of_set_bits(value: u64) -> Vec<u8> {
    let mut number = value;
    let mut indices = Vec::new();
    let mut index = 0;
    
    while number > 0 {
        if number & 1 == 1 {
            indices.push(index);
        }
        number >>= 1;
        index += 1;
    }
    
    indices
}
