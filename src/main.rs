mod string;
use std::io;

fn run_main(writer: &mut dyn io::Write) {
    match writeln!(writer, "Hello, world!") {
        Ok(rc) => rc,
        Err(e) => println!("error: {}", e.to_string())
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use string::u8_to_str;

    #[test]
    fn test_main() {
        let mut cursor = io::Cursor::new(vec![0; 24]);
        run_main(&mut cursor);
        let s = u8_to_str(cursor.get_ref());
        assert_eq!(&s[0..13], "Hello, world!");
    }
}

fn main() {
    run_main(&mut io::stdout());
}
