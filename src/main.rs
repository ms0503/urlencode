use clap::Parser;

fn main() {
    let opts = Opts::parse();
    if opts.decode {
        println!("{}", decode_rfc3986(&opts.src.join(" ")));
    } else {
        println!("{}", encode_rfc3986(&opts.src.join(" ")));
    }
}

#[derive(Debug, Parser)]
#[command(about, version)]
struct Opts {
    #[arg(long, short)]
    pub decode: bool,
    #[arg()]
    pub src: Vec<String>
}

fn encode_rfc3986(src: &str) -> String {
    let mut dst = String::new();
    for c in src.chars() {
        let c = c as u32;
        if c < 0x80 {
            if c <= 0x2c
                || c == 0x2f
                || c == 0x40
                || (0x5b..=0x5e).contains(&c)
                || c == 0x60
                || (0x7b..=0x7d).contains(&c)
                || c == 0x7f
            {
                dst.push_str(&format!("%{:02X}", c));
            } else {
                dst.push(char::from_u32(c).unwrap());
            }
        } else if c < 0x800 {
            let fst = 0xc0 | c >> 6;
            let snd = 0x80 | c & 0x3f;
            dst.push_str(&format!("%{:02X}%{:02X}", fst, snd));
        } else if c < 0x10000 {
            let fst = 0xe0 | c >> 12;
            let snd = 0x80 | c >> 6 & 0x3f;
            let trd = 0x80 | c & 0x3f;
            dst.push_str(&format!("%{:02X}%{:02X}%{:02X}", fst, snd, trd));
        } else if c < 0x110000 {
            let fst = 0xf0 | c >> 18;
            let snd = 0x80 | c >> 12 & 0x3f;
            let trd = 0x80 | c >> 6 & 0x3f;
            let frt = 0x80 | c & 0x3f;
            dst.push_str(&format!("%{:02X}%{:02X}%{:02X}%{:02X}", fst, snd, trd, frt));
        } else {
            unreachable!();
        }
    }
    dst
}

fn decode_rfc3986(src: &str) -> String {
    let mut dst = String::new();
    let mut is_encoded = false;
    let mut num_buf = vec![0u8; 2];
    let mut num_ind = 0usize;
    let mut char_buf = vec![0u8; 4];
    let mut char_ind = 0usize;
    let mut char_len = 0usize;
    for c in src.chars() {
        if c == '%' {
            is_encoded = true;
            continue;
        }
        if is_encoded {
            if num_ind == 1 {
                num_buf[num_ind] = c as u8;
                num_ind = 0;
                let num = u8::from_str_radix(
                    std::str::from_utf8(&num_buf).expect("Error: unexpected token."),
                    16
                )
                .expect("Error: unexpected token.");
                println!("num: {:?}", &num);
                char_buf[char_ind] = num;
                if char_ind == 0 {
                    char_len = if num < 0x80 {
                        1
                    } else if num < 0xe0 {
                        2
                    } else if num < 0xf0 {
                        3
                    } else if num < 0xf4 {
                        4
                    } else {
                        unreachable!()
                    };
                }
                char_ind += 1;
                if char_ind == char_len {
                    dst.push_str(
                        std::str::from_utf8(&char_buf.clone()[0..char_len])
                            .expect("Error: invalid utf8 sequence.")
                    );
                    char_ind = 0;
                    char_len = 0;
                }
                is_encoded = false;
            } else {
                num_buf[num_ind] = c as u8;
                num_ind += 1;
            }
        } else {
            dst.push(c);
        }
    }
    dst
}
