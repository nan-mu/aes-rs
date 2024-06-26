mod bit_shift;
mod sbox;
mod test;

// use log::debug;

// 列混合使用的矩阵
const MIX_MATIX: [u8; 16] = [2, 3, 1, 1, 1, 2, 3, 1, 1, 1, 2, 3, 3, 1, 1, 2];

// 密钥拓展使用的偏移常量
const RCON: [u8; 11] = [
    0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36,
];

/// 标准流程
pub fn encrypt(input: [u8; 16], key: [u8; 16]) -> [u8; 16] {
    let key = key_expansion(key);
    let mut input = input;

    add_round_key(&mut input, &key[0]);
    for round in 1..10 {
        sbox::subcode(&mut input);
        shift_rows(&mut input);
        mix_columns(&mut input);
        add_round_key(&mut input, &key[round]);
    }

    sbox::subcode(&mut input);
    shift_rows(&mut input);
    add_round_key(&mut input, &key[10]);

    input
}

/// 行位移，ShiftRows
fn shift_rows(input: &mut [u8; 16]) {
    // println!("ShiftRows: {:02x?}", input);
    // 替换规则
    //     0[0],  1[5],  2[10], 3[15],
    //     4[4],  5[9],  6[14], 7[3],
    //     8[8],  9[13], 10[2], 11[7],
    //     12[12],13[1], 14[6], 15[11],
    input.swap(1, 5); // 5 is 1
    input.swap(2, 10); // 这个正巧
    input.swap(3, 15); // 15 is 3
    input.swap(5, 9); //9 is 1
    input.swap(6, 14); //14 is 6
    input.swap(7, 15); //15 is 7
    input.swap(9, 13); // 13 is 1
    input.swap(11, 15); // 15 is 11
                        // println!("ShiftRows: {:02x?}", input);
}

/// 列混合，MixColumns
fn mix_columns(input: &mut [u8]) {
    // println!("MixColumns: {:02x?}", input);
    assert_eq!(input.len(), MIX_MATIX.len());
    let mut col = [0; 16];
    col.copy_from_slice(input);
    for i in 0..4 {
        for ii in 0..4 {
            input[i * 4 + ii] = bit_shift::gmul_times(
                &col[(i * 4)..(i * 4 + 4)],
                &MIX_MATIX[(ii * 4)..(ii * 4 + 4)],
            );
        }
    }
    //期望04 66 81 e5 e0 cb 19 9a 48 f8 d3 7a 28 06 26 4c
    // println!("MixColumns: {:02x?}", input);
}

/// 轮密钥加
fn add_round_key(input: &mut [u8], key: &[u8]) {
    // println!("AddRoundKey: {:x?}", input);
    assert!(input.len() == key.len());
    for index in 0..input.len() {
        input[index] ^= key[index];
    }
    // println!("AddRoundKey: {:02x?}", input);
}

/// 密钥扩展
fn key_expansion(key: [u8; 16]) -> [[u8; 16]; 11] {
    let mut round_key: [[u8; 16]; 11] = [[0; 16]; 11];
    // 第一组密钥
    round_key[0] = key;
    for i in 1..11 {
        let key = &round_key[i - 1]; //上一组字
        let mut last_word = &key[12..16]; //上一个字 W[i-1]
        let mut new_key = [0u8; 16];
        for ii in 0..4 {
            let orgin_word = &key[(ii * 4)..(ii * 4 + 4)]; // W[i - 4]
            let mut pre_word = [0u8; 4];
            if ii == 0 {
                pre_word = bit_shift::rot_word(last_word); // T(W[i-1])
                sbox::subcode(&mut pre_word);
            } else {
                pre_word.copy_from_slice(last_word);
            }
            for iii in 0..4 {
                new_key[ii * 4 + iii] = pre_word[iii] ^ orgin_word[iii]; // W[i]
            }
            if ii == 0 {
                new_key[ii * 4] ^= RCON[i];
            }
            last_word = &new_key[(ii * 4)..(ii * 4 + 4)];
        }
        round_key[i] = new_key;
    }
    // println!("KeyExpansion: ");
    // for index in 0..11 {
    //     let key = round_key[index];
    //     for i in 0..4 {
    //         print!("{:02} ", index * 4 + i);
    //         for ii in 0..4 {
    //             print!("{:02x}", key[i * 4 + ii]);
    //         }
    //         println!("");
    //     }
    // }
    round_key
}
