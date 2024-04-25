/// 向量叉乘，使用有限域GF(2^8)乘法，将乘法结果放到第一个参数中
pub fn gmul_times(input1: &[u8], input2: &[u8]) -> [u8; 4] {
    assert_eq!(4 as usize, input1.len());
    assert_eq!(4 as usize, input2.len());
    let mut output = [0; 4];
    for index in 0..4 {
        output[index] = gmul(input1[index], input2[index]);
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
