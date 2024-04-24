use rand::Rng;

mod sbox;
mod test;

/// 标准流程
pub fn encode(input: [u8; 16], key: Option<[[u8; 4]; 4]>) -> [u8; 16] {
    let input = [
        [input[0], input[1], input[2], input[3]],
        [input[4], input[5], input[6], input[7]],
        [input[8], input[9], input[10], input[11]],
        [input[12], input[13], input[14], input[15]],
    ];

    let key = match key {
        None => generate_aes_128_key(),
        Some(key) => key,
    };
    let key = key_expansion(key);

    let mut input = add_round_key(input, take_key(0, &key));

    for round in 0..9 {
        for index in 0..4 {
            input[index] = sbox::encode(input[index]);
        }
        input = shift_rows(input);
        input = mix_columns(input);
        input = add_round_key(input, take_key((round + 1) * 4, &key));
    }

    for index in 0..4 {
        input[index] = sbox::encode(input[index]);
    }
    input = shift_rows(input);
    input = add_round_key(input, take_key(40, &key));

    let mut output = [0u8; 16];
    for (i, item) in input.iter().flatten().enumerate() {
        output[i] = *item;
    }
    output
}

/// 生成密钥
fn generate_aes_128_key() -> [[u8; 4]; 4] {
    //6162636465666768696a6b6c6d6e6f70
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 16];
    rng.fill(&mut key);
    [
        [key[0], key[1], key[2], key[3]],
        [key[4], key[5], key[6], key[7]],
        [key[8], key[9], key[10], key[11]],
        [key[12], key[13], key[14], key[15]],
    ]
}

/// 行位移，ShiftRows
fn shift_rows(input: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    [
        input[0],
        [input[1][1], input[1][2], input[1][3], input[1][0]],
        [input[2][2], input[2][3], input[2][0], input[2][1]],
        [input[3][3], input[3][0], input[3][1], input[3][2]],
    ]
}

/// 列混合，MixColumns
fn mix_columns(input: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output = [[0; 4]; 4];
    for index in 0..4 {
        output[0][index] = gmul(2, input[0][index])
            ^ gmul(3, input[1][index])
            ^ gmul(1, input[2][index])
            ^ gmul(1, input[3][index]);
        output[1][index] = gmul(1, input[0][index])
            ^ gmul(2, input[1][index])
            ^ gmul(3, input[2][index])
            ^ gmul(1, input[3][index]);
        output[2][index] = gmul(1, input[0][index])
            ^ gmul(1, input[1][index])
            ^ gmul(2, input[2][index])
            ^ gmul(3, input[3][index]);
        output[3][index] = gmul(3, input[0][index])
            ^ gmul(1, input[1][index])
            ^ gmul(1, input[2][index])
            ^ gmul(2, input[3][index]);
    }
    output
}

/// 执行有限域GF(2^8)乘法
fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    let mut hi_bit_set;
    for _counter in 0..8 {
        if (b & 1) == 1 {
            p ^= a;
        }
        hi_bit_set = a & 0x80;
        a <<= 1;
        if hi_bit_set == 0x80 {
            a ^= 0x1b; /* x^8 + x^4 + x^3 + x + 1 */
        }
        b >>= 1;
    }
    p
}

/// 列混合的逆运算
fn _de_mix_columns(input: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output = [[0; 4]; 4];
    for index in 0..4 {
        output[0][index] = gmul(0x0e, input[0][index])
            ^ gmul(0x0b, input[1][index])
            ^ gmul(0x0d, input[2][index])
            ^ gmul(0x09, input[3][index]);
        output[1][index] = gmul(0x09, input[0][index])
            ^ gmul(0x0e, input[1][index])
            ^ gmul(0x0b, input[2][index])
            ^ gmul(0x0d, input[3][index]);
        output[2][index] = gmul(0x0d, input[0][index])
            ^ gmul(0x09, input[1][index])
            ^ gmul(0x0e, input[2][index])
            ^ gmul(0x0b, input[3][index]);
        output[3][index] = (0x0b * input[0][index])
            ^ gmul(0x0d, input[1][index])
            ^ gmul(0x09, input[2][index])
            ^ gmul(0x0e, input[3][index]);
    }
    output
}

/// 轮密钥加
fn add_round_key(input: [[u8; 4]; 4], key: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            output[i][j] = input[i][j] ^ key[i][j];
        }
    }
    output
}

const RCON: [u8; 11] = [
    0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36,
];

/// 密钥扩展
fn key_expansion(key: [[u8; 4]; 4]) -> [[u8; 4]; 44] {
    let mut round_key = [[0; 4]; 44];
    for i in 0..4 {
        round_key[i] = key[i];
    }
    for i in 4..44 {
        let mut temp = round_key[i - 1];
        if i % 4 == 0 {
            temp = sbox::encode(rot_word(temp)) as [u8; 4];
            temp[0] ^= RCON[i / 4];
        }
        for j in 0..4 {
            round_key[i][j] = round_key[i - 4][j] ^ temp[j];
        }
    }
    round_key
}

fn rot_word(word: [u8; 4]) -> [u8; 4] {
    [word[1], word[2], word[3], word[0]]
}

fn take_key(index: usize, key: &[[u8; 4]; 44]) -> [[u8; 4]; 4] {
    let mut output = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            output[j][i] = key[index + i][j];
        }
    }
    output
}
