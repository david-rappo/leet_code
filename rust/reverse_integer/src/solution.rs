const BIT_COUNT: usize = 32;

#[allow(dead_code)]
pub fn reverse(x: i32) -> i32 {
    let is_signed = x < 0;
    let mut y = x.unsigned_abs();
    let mut i = find_begin(y);
    let mut j = 0;
    while i > j {
        let bit_at_i = get_bit(i, y);
        println!("bit_at_i {:0b}", bit_at_i);
        let bit_at_j = get_bit(j, y);
        println!("bit_at_j {:0b}", bit_at_j);
        set_bit(i, bit_at_j, &mut y);
        set_bit(j, bit_at_i, &mut y);
        i -= 1;
        j += 1;
    }

    if is_signed {
        -(y as i32)
    } else {
        y as i32
    }
}

fn find_begin(x: u32) -> usize {
    let mut index = BIT_COUNT - 1;
    loop {
        let bit = get_bit(index, x);
        if bit != 0 {
            return index;
        }

        if index > 0 {
            index -= 1;
        } else {
            break;
        }
    }
    
    0
}

// index: 0 represents the rightmost bit.
fn set_bit(index: usize, bit: u32, x: &mut u32) {
    debug_assert!((bit == 0) || (bit == 1));
    let shifted = bit << index;
    *x |= shifted
}

// index: 0 represents the rightmost bit.
fn get_bit(index: usize, x: u32) -> u32 {
    let shifted = 1 << index;
    (x & shifted) >> index
}