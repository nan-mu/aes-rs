/// 向量叉乘，使用有限域GF(2^8)乘法，将乘法结果放到第一个参数中
pub fn gmul_times(input1: &[u8], input2: &[u8]) -> u8 {
    assert_eq!(4 as usize, input1.len());
    assert_eq!(4 as usize, input2.len());
    let mut output = 0;
    for index in 0..4 {
        output ^= gmul(input1[index], input2[index]);
    }
    output
}

/// 位移一个字
pub fn rot_word(word: &[u8]) -> [u8; 4] {
    //断言这是一个字
    assert!(word.len() == 4);
    [word[1], word[2], word[3], word[0]]
}

/// 执行有限域GF(2^8)乘法
fn gmul(a: u8, b: u8) -> u8 {
    let mut result = 0;
    let mut b = b;

    for i in 0..8 {
        if a & (1 << i) != 0 {
            result ^= b;
        }
        let high_bit_set = b & 0x80 != 0;
        b <<= 1;
        if high_bit_set {
            b ^= 0x1B; // 0x1B 是 AES 中用于乘法的固定值
        }
    }

    result
}

#[test]
fn test_gmul() {
    assert_eq!(gmul(0x57, 0x13), 0xfe);
    assert_eq!(gmul(0x13, 0x57), 0xfe);
    assert_eq!(gmul(0x57, 0x83), 0xc1);
    assert_eq!(gmul(0x83, 0x57), 0xc1);
}
