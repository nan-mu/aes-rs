#[cfg(test)]
mod tests {
    use crate::encrypt;

    #[test]
    fn test_example() {
        let input = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];
        let key = [
            0x2b, 0x28, 0xab, 0x09, 0x7e, 0xae, 0xf7, 0xcf, 0x15, 0xd2, 0x15, 0x4f, 0x16, 0xa6,
            0x88, 0x3c,
        ];
        println!("加密结果：{:x?}", encrypt(input, key));
    }

    // /// 生成密钥
    // fn _generate_aes_128_key() -> [u8; 16] {
    //     use rand::Rng;
    //     //6162636465666768696a6b6c6d6e6f70
    //     let mut rng = rand::thread_rng();
    //     let mut key = [0u8; 16];
    //     rng.fill(&mut key);
    //     key
    // }
}
