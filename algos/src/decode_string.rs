pub fn decode_string(mut s: String) -> String {
    s.push(']');
    decrypt(s.trim_end().as_bytes(), 1).1
}

fn decrypt(str: &[u8], times: usize) -> (usize, String) {
    let mut times_counter: usize = 0;
    let mut index: usize = 0;
    let mut result = String::new();

    loop {
        let c = str[index];
        //check for int
        if (b'0'..=b'9').contains(&c) {
            times_counter = (times_counter * 10) + ((c - b'0') as usize);
            index += 1;
            continue;
        }

        if c == b'[' {
            index += 1;
            {
                let (i, r) = decrypt(&str[index..], times_counter);
                index += i;
                result.push_str(&r[..]);
            }
            times_counter = 0;
            continue;
        }

        if c == b']' {
            result.push_str(&result.repeat(times - 1)[..]);
            return (index + 1, result);
        }

        result.push(c as char);
        index += 1;
    }
}

#[test]
fn decode_string_test() {
    assert_eq!(decode_string("2[3[a]b]".to_string()), "aaabaaab");
}
