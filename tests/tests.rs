use hash_finder::calculate;

#[test]
fn test_n_3_f_12() {
    use std::fs;

    let expected_lines = vec![
      "4611686018427388409, \"c1a92d314fe6b8f4268897393fbe9eb844fbdff8fccb847f76021df31f905000\"",
      "4611686018427391743, \"bec4dc72dbc7fff0c1598a2f6a1ee6f555fbacefe80b2d302d60d7ec2e814000\"",
      "4163, \"95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000\"",
      "1537228672809135581, \"04565f5e4d4b68df85e9469713960f8b679928ddf683b196e79893a46d9b9000\"",
      "1537228672809136056, \"307edd7342b0fbf2f05e871dce2ee1fdb8ac44a3b655fcb8beb7d717b5aca000\"",
      "4611686018427395130, \"d45580b09c082a03677434a2e297432cda4584dae55911e404f439360d7b8000\"",
      "6148914691236526231, \"e198e128053f18674f4baf084e496e4619a7f4cc079f97da57ce71e05e8eb000\"",
      "6148914691236526488, \"6e5205ba505a37e6ec87903cabe2e6836705ba5eac26c14af5062a7edf19e000\"",
      "11848, \"cb58074fd7620cd0ff471922fd9df8812f29f302904b15e389fc14570a66f000\"",
      "13835058055282163749, \"712e1723acbdee67ce8ed748ae1a7b982b543733b0b43a2105a4948d6a771000\"",
      "10760600709663905480, \"ae0e09f745179ebe26e38997361c8aa7150c55e39cb191ff61a2744b9fc39000\"",
      "12843, \"bb90ff93a3ee9e93c123ebfcd2ca1894e8994fef147ad81f7989eccf83f64000\"",
   ];

    let test_file = "tests/output_3_zeros.txt";

    calculate(&3, &12, Some(test_file));

    let contents = fs::read_to_string(test_file).expect("Failed to read the test output file");

    let actual_lines: Vec<_> = contents.lines().collect();

    let matches = actual_lines
        .iter()
        .filter(|line| expected_lines.contains(line))
        .count();
    assert!(
        matches >= 3,
        "Expected at least 3 matching lines, but found {}",
        matches
    );
}

#[test]
fn test_n_4_f_12() {
    use std::fs;

    let expected_lines = vec![
      "7686143364045666223, \"d630d8e8975e0a3b68e722866a84dab6212f3658d5540f1a03e2007856110000\"",
      "31214, \"16b024b09ebcb9d66f6a9968858d7e01081e51a14a4922edf3c8e3c2009c0000\"",
      "12297829382473058451, \"eea97fcc5e324cfc0c27c1cfbbc4efa6b119c61e98804ed320a89c73aec90000\"",
      "15372286728091331711, \"6b21d62ddaabbb5fcecb79c336cc6dd20cb8b8d01599229e72d1dbf578ee0000\"",
      "10760600709663941953, \"026a013828ebee0470ddb79aab755965a91cac6177e8fffbf0e10f1535e90000\"",
      "1537228672809179669, \"de06f618c86219afb3232b10bc09f3b3dd9f0922748e6db86ef42cfcd4620000\"",
      "12297829382473089863, \"d42e5123322c955886fd497e569b3c38d99225bf2f3f494a891a67bb35390000\"",
      "12297829382473095182, \"2b7a249bef93846c7e95e56ab2be523c80e0a908a022d388e4cdf753c7ab0000\"",
      "88183, \"1b8af662814e2b7c794ae26d5624f23886b6915fb3e9e2c0d4d62099731e0000\"",
      "12297829382473095923, \"f24358b886a3e7296bb81fa7072fd8d2507e141cbb1f60b4a7e483eab8070000\"",
      "10760600709663983110, \"156115a3d7b234cc0748aa089d2ec0a4b40a0898fcf290df96b92e2929ef0000\"",
      "3074457345618346296, \"54505603b788212c92f6b9934cc1b533393115421cb23ea3fcc925c9d5600000\"",
   ];

    let test_file = "tests/output_4_zeros.txt";

    calculate(&4, &12, Some(test_file));

    let contents = fs::read_to_string(test_file).expect("Failed to read the test output file");

    let actual_lines: Vec<_> = contents.lines().collect();

    let matches = actual_lines
        .iter()
        .filter(|line| expected_lines.contains(line))
        .count();
    assert!(
        matches >= 4,
        "Expected at least 4 matching lines, but found {}",
        matches
    );
}

#[test]
fn test_n_5_f_12() {
   use std::fs;

   let expected_lines = vec![
      "3074457345618346296, \"54505603b788212c92f6b9934cc1b533393115421cb23ea3fcc925c9d5600000\"",
      "7686143364045797670, \"8025d6cf07470c0f1d4ad947a3e04242fee03ba4a853d2703c370db14d000000\"",
      "16909515400900557825, \"d02347f5824eddea3021dc7ed924df15536452ac3a1e29c94da98d66cb600000\"",
      "9223372036854961814, \"762ba2a4968989f561cb6f0d657e6657b215a2727c4af533e68659aa8f600000\"",
      "7686143364045974998, \"5910b9c4d5118e467b8aacaff91c8ecb84001854560c28ff88f3110930000000\"",
      "6148914691236956037, \"a261e213c2fcedceaebecbb48d9dfae6d409a49314f8d88055a208d573800000\"",
      "9223372036855356662, \"ec0250ffaf710c788f9dca2e90c16c264f161c6482dd91f27a76ee2b64600000\"",
      "828028, \"d95f19b5269418c0d4479fa61b8e7696aa8df197082b431a65ff37595c100000\"",
      "15372286728092323972, \"6293e3c6533d2410001aca4c379b292704ef36ba5db3f6c7fa4e6fc03fa00000\"",
      "1537228672810104080, \"3cb12ec4d9c6af9f75695f7b946c5c8811ecb53952b9a0ffd44f9bdb54d00000\"",
      "4611686018428308185, \"dd9d4eb2d2885164f36e862d145bd7e350a9225205a3e5e6c5432f908d800000\"",
      "10760600709665058594, \"3fce8bfc6f0a0e9792de9f1ac776d62733e8a332d8027c10b20bc53938c00000\"",
   ];

   let test_file = "tests/output_5_zeros.txt";

   // Вызываем calculate
   calculate(&5, &12, Some(test_file));

   // Читаем содержимое файла
   let contents = fs::read_to_string(test_file).expect("Failed to read the test output file");

   // Разбиваем содержимое на строки
   let actual_lines: Vec<_> = contents.lines().collect();

   // Проверяем, что как минимум 12 строк совпадают с ожидаемыми
   let matches = actual_lines.iter().filter(|line| expected_lines.contains(line)).count();
   assert!(
      matches >= 7,
      "Expected at least 7 matching lines, but found {}",
      matches
   );
}

#[test]
fn test_n_6_f_12() {
    use std::fs;

    let expected_lines = vec![
        "7686143364045797670, \"8025d6cf07470c0f1d4ad947a3e04242fee03ba4a853d2703c370db14d000000\"",
        "7686143364045974998, \"5910b9c4d5118e467b8aacaff91c8ecb84001854560c28ff88f3110930000000\"",
        "9223372036857194586, \"0c8ff13cab6cfa8ac1de5c10caae54f077368596f0ef664c088d417504000000\"",
        "16909515400902939482, \"f2aafd10d28026c5e6e745d7746dbd6a46a60067e19f8d6cf0ef276142000000\"",
        "6148914691239985947, \"5dd091c17060ac9c71ec31023c6b6a503ff6dde266c4745c900f11c030000000\"",
        "10760600709668568446, \"7523b4af2d8e3bbd5bdb7d8f5ba42c6f908fa549a3f02a9e9559e2b053000000\"",
        "3074457345629967438, \"c9b22dbb70192724f0cd67011b3b5a06c6be58c27f25e1928db6a7d7c9000000\"",
        "15372286728104418151, \"a923647abb966242763a583b7be704a0c42414d6e6ab79050e836be12f000000\"",
        "12297829382486476487, \"8b1516ceba7f05a82bb73474f3ea64f47ffc383a47c5ea62d04efd006f000000\"",
        "1537228672823564072, \"316246630373827a7dba8b14742efe1413d4395fa0782cfc63ccbba284000000\"",
        "12297829382491449245, \"8f70fb2655de576d57cac9e1bddba0572c05788696a3d1bfccec7761b7000000\"",
        "15372286728112152773, \"0d84689e2e2262304bb7d0bc829e5192088f3ecbe8b084fd73e56c4ac3000000\"",
    ];

    let test_file = "tests/output_6_zeros.txt";

    calculate(&6, &12, Some(test_file));

    let contents = fs::read_to_string(test_file).expect("Failed to read the test output file");

    let actual_lines: Vec<_> = contents.lines().collect();

    let matches = actual_lines.iter().filter(|line| expected_lines.contains(line)).count();
    assert!(
        matches >= 6,
        "Expected at least 6 matching lines, but found {}",
        matches
    );
}

