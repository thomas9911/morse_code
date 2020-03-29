use std::convert::TryFrom;

const DASH: bool = true;
const DOT: bool = false;

const PRINT_CHAR: char = '∎';
const DASH_CHAR: char = '-';
const DOT_CHAR: char = '.';


#[derive(Debug, PartialEq)]
pub enum Char {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TryFrom<char> for Char {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'a' => Ok(Char::A),
            'b' => Ok(Char::B),
            'c' => Ok(Char::C),
            'd' => Ok(Char::D),
            'e' => Ok(Char::E),
            'f' => Ok(Char::F),
            'g' => Ok(Char::G),
            'h' => Ok(Char::H),
            'i' => Ok(Char::I),
            'j' => Ok(Char::J),
            'k' => Ok(Char::K),
            'l' => Ok(Char::L),
            'm' => Ok(Char::M),
            'n' => Ok(Char::N),
            'o' => Ok(Char::O),
            'p' => Ok(Char::P),
            'q' => Ok(Char::Q),
            'r' => Ok(Char::R),
            's' => Ok(Char::S),
            't' => Ok(Char::T),
            'u' => Ok(Char::U),
            'v' => Ok(Char::V),
            'w' => Ok(Char::W),
            'x' => Ok(Char::X),
            'y' => Ok(Char::Y),
            'z' => Ok(Char::Z),
            '0' => Ok(Char::Zero),
            '1' => Ok(Char::One),
            '2' => Ok(Char::Two),
            '3' => Ok(Char::Three),
            '4' => Ok(Char::Four),
            '5' => Ok(Char::Five),
            '6' => Ok(Char::Six),
            '7' => Ok(Char::Seven),
            '8' => Ok(Char::Eight),
            '9' => Ok(Char::Nine),
            _ => Err("Character cannot to be converted to Morse Char"),
        }
    }
}

impl From<Char> for char {
    fn from(value: Char) -> char {
        match value {
            Char::A => 'a',
            Char::B => 'b',
            Char::C => 'c',
            Char::D => 'd',
            Char::E => 'e',
            Char::F => 'f',
            Char::G => 'g',
            Char::H => 'h',
            Char::I => 'i',
            Char::J => 'j',
            Char::K => 'k',
            Char::L => 'l',
            Char::M => 'm',
            Char::N => 'n',
            Char::O => 'o',
            Char::P => 'p',
            Char::Q => 'q',
            Char::R => 'r',
            Char::S => 's',
            Char::T => 't',
            Char::U => 'u',
            Char::V => 'v',
            Char::W => 'w',
            Char::X => 'x',
            Char::Y => 'y',
            Char::Z => 'z',
            Char::Zero => '0',
            Char::One => '1',
            Char::Two => '2',
            Char::Three => '3',
            Char::Four => '4',
            Char::Five => '5',
            Char::Six => '6',
            Char::Seven => '7',
            Char::Eight => '8',
            Char::Nine => '9',
        }
    }
}

impl Char {
    pub fn to_morse(&self) -> &'static [bool] {
        match self {
            &Char::A => &[DOT, DASH],
            &Char::B => &[DASH, DOT, DOT, DOT],
            &Char::C => &[DASH, DOT, DASH, DOT],
            &Char::D => &[DASH, DOT, DOT],
            &Char::E => &[DOT],
            &Char::F => &[DOT, DOT, DASH, DOT],
            &Char::G => &[DASH, DASH, DOT],
            &Char::H => &[DOT, DOT, DOT, DOT],
            &Char::I => &[DOT, DOT],
            &Char::J => &[DOT, DASH, DASH, DASH],
            &Char::K => &[DASH, DOT, DASH],
            &Char::L => &[DOT, DASH, DOT, DOT],
            &Char::M => &[DASH, DASH],
            &Char::N => &[DASH, DOT],
            &Char::O => &[DASH, DASH, DASH],
            &Char::P => &[DOT, DASH, DASH, DOT],
            &Char::Q => &[DASH, DASH, DOT, DASH],
            &Char::R => &[DOT, DASH, DOT],
            &Char::S => &[DOT, DOT, DOT],
            &Char::T => &[DASH],
            &Char::U => &[DOT, DOT, DASH],
            &Char::V => &[DOT, DOT, DOT, DASH],
            &Char::W => &[DOT, DASH, DASH],
            &Char::X => &[DASH, DOT, DOT, DASH],
            &Char::Y => &[DASH, DOT, DASH, DASH],
            &Char::Z => &[DASH, DASH, DOT, DOT],
            &Char::One => &[DOT, DASH, DASH, DASH, DASH],
            &Char::Two => &[DOT, DOT, DASH, DASH, DASH],
            &Char::Three => &[DOT, DOT, DOT, DASH, DASH],
            &Char::Four => &[DOT, DOT, DOT, DOT, DASH],
            &Char::Five => &[DOT, DOT, DOT, DOT, DOT],
            &Char::Six => &[DASH, DOT, DOT, DOT, DOT],
            &Char::Seven => &[DASH, DASH, DOT, DOT, DOT],
            &Char::Eight => &[DASH, DASH, DASH, DOT, DOT],
            &Char::Nine => &[DASH, DASH, DASH, DASH, DOT],
            &Char::Zero => &[DASH, DASH, DASH, DASH, DASH],
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MorseString {
    data: Vec<Option<Char>>,
}

impl TryFrom<&str> for MorseString {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut data = Vec::new();
        for item in value.chars() {
            if item == ' ' {
                data.push(None);
            } else {
                let ch = Char::try_from(item)?;
                data.push(Some(ch));
            }
        }

        Ok(MorseString { data: data })
    }
}

impl MorseString {
    pub fn print_morse(&self) -> String {
        let mut morse_string = self
            .bits()
            .iter()
            .map(|x| match x {
                true => PRINT_CHAR,
                false => ' ',
            })
            .collect::<String>()
            .trim_end()
            .to_string();

        morse_string.push_str(" ");
        morse_string
    }

    pub fn print_morse_short(&self) -> String {
        self.data.iter().fold(String::new(), |mut acc, x| {
            match x {
                None => acc.push(' '),
                Some(ch) => {
                    acc.push_str(
                        ch.to_morse()
                            .iter()
                            .map(|y| match y {
                                &DASH => DASH_CHAR,
                                &DOT => DOT_CHAR,
                            })
                            .collect::<String>()
                            .as_ref(),
                    );
                }
            }
            acc.push(' ');
            acc
        }).trim_end().to_string()
    }

    pub fn bits(&self) -> Vec<bool> {
        self.data.iter().fold(Vec::new(), |mut acc, x| {
            acc.extend(bits_char(x));
            acc.extend_from_slice(&[false, false]);
            acc
        })
    }
}

fn bits_char(ch: &Option<Char>) -> Vec<bool> {
    match ch {
        None => vec![false, false],
        Some(x) => x.to_morse().iter().fold(Vec::new(), |mut acc, x| {
            acc.extend_from_slice(bits_morse_char(x));
            acc
        }),
    }
}

fn bits_morse_char(d: &bool) -> &[bool] {
    match d {
        &DASH => &[true, true, true, false],
        &DOT => &[true, false],
    }
}

#[test]
fn parse_string() {
    use std::convert::TryInto;

    let mut expected_data = Vec::new();

    expected_data.push(Some(Char::T));
    expected_data.push(Some(Char::E));
    expected_data.push(Some(Char::S));
    expected_data.push(Some(Char::T));
    expected_data.push(None);
    expected_data.push(Some(Char::T));
    expected_data.push(Some(Char::E));
    expected_data.push(Some(Char::S));
    expected_data.push(Some(Char::T));

    assert_eq!(
        MorseString {
            data: expected_data
        },
        "test test".try_into().unwrap()
    );
}

#[test]
fn print_morse_test_test() {
    use std::convert::TryInto;

    let t: MorseString = "test test".try_into().unwrap();

    assert_eq!(
        "∎∎∎   ∎   ∎ ∎ ∎   ∎∎∎       ∎∎∎   ∎   ∎ ∎ ∎   ∎∎∎ ",
        t.print_morse()
    );
}

#[test]
fn print_short_morse_test_test() {
    use std::convert::TryInto;

    let t: MorseString = "test test".try_into().unwrap();

    assert_eq!(
        "- . ... -   - . ... -",
        t.print_morse_short()
    );
}

#[test]
fn print_morse_1234() {
    use std::convert::TryInto;

    let t: MorseString = "1234 5678 90".try_into().unwrap();

    assert_eq!(
        "∎ ∎∎∎ ∎∎∎ ∎∎∎ ∎∎∎   ∎ ∎ ∎∎∎ ∎∎∎ ∎∎∎   \
        ∎ ∎ ∎ ∎∎∎ ∎∎∎   ∎ ∎ ∎ ∎ ∎∎∎       \
        ∎ ∎ ∎ ∎ ∎   ∎∎∎ ∎ ∎ ∎ ∎   ∎∎∎ ∎∎∎ ∎ ∎ ∎   \
        ∎∎∎ ∎∎∎ ∎∎∎ ∎ ∎       ∎∎∎ ∎∎∎ ∎∎∎ ∎∎∎ ∎   \
        ∎∎∎ ∎∎∎ ∎∎∎ ∎∎∎ ∎∎∎ ",
        t.print_morse()
    );
}

#[test]
fn bits_morse_test_test() {
    use std::convert::TryInto;

    let t: MorseString = "test test".try_into().unwrap();

    assert_eq!(
        vec![
            true, true, true, false, false, false, true, false, false, false, true, false, true,
            false, true, false, false, false, true, true, true, false, false, false, false, false,
            false, false, true, true, true, false, false, false, true, false, false, false, true,
            false, true, false, true, false, false, false, true, true, true, false, false, false
        ],
        t.bits()
    );
}
