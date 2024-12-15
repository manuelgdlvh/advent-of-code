use std::str::FromStr;

pub struct Header {
    values: &'static [char],
    validated: bool,
    idx: usize,
}

impl Header {
    pub fn new(values: &'static [char]) -> Self {
        Self { values, validated: false, idx: 0 }
    }

    pub fn clear(&mut self) {
        self.idx = 0;
        self.validated = false;
    }
    fn validate(&mut self, c: char) -> bool {
        let header_val = if let Some(val) = self.values.get(self.idx) {
            val
        } else {
            self.idx = 0;
            return false;
        };

        if !c.eq(header_val) {
            self.idx = 0;
            return false;
        }

        self.idx += 1;
        if self.idx == self.values.len() {
            self.validated = true;
        }

        return true;
    }

    fn is_validated(&self) -> bool {
        self.validated
    }
}


pub struct ComposeHeader {
    static_: Header,
    dyn_: Header,
}

impl ComposeHeader {
    const STATIC_HEADER_PART: &'static [char] = &['m', 'u', 'l', '('];
    const DYN_HEADER_PART: &'static [char] = &[',', ')'];
    pub fn new() -> Self {
        Self {
            static_: Header::new(Self::STATIC_HEADER_PART),
            dyn_: Header::new(Self::DYN_HEADER_PART),
        }
    }

    pub fn static_(&mut self) -> &mut Header {
        &mut self.static_
    }

    pub fn dyn_(&mut self) -> &mut Header {
        &mut self.dyn_
    }
    pub fn clear(&mut self) {
        self.static_.clear();
        self.dyn_.clear();
    }
}

pub struct InstructionMul<'a> {
    value: &'a str,
    idx: usize,
    result: Option<u64>,
    header: ComposeHeader,
}

impl<'a> InstructionMul<'a> {
    pub fn new(value: &'a str) -> Self {
        Self { value, idx: 0, result: None, header: ComposeHeader::new() }
    }


    pub fn process(&mut self) -> anyhow::Result<u64> {
        if let Some(result) = self.result {
            return Ok(result);
        }

        let mut result = 0;
        loop {
            let (left_buffer, right_buffer) = self.next();
            if left_buffer.is_empty() || right_buffer.is_empty() {
                break;
            }

            let left_number = u64::from_str(&left_buffer)?;
            let right_number = u64::from_str(&right_buffer)?;
            result += left_number * right_number;
            println!("{left_number} {right_number}");
        }

        self.result = Some(result);
        Ok(result)
    }

    fn next(&mut self) -> (String, String) {
        let mut clear = false;
        let mut found = false;

        let mut left_buffer = String::new();
        let mut right_buffer = String::new();
        let mut current_buffer = &mut left_buffer;
        for c in self.value.chars().skip(self.idx) {
            self.idx += 1;

            if !self.header.static_().is_validated() {
                self.header.static_().validate(c);
                continue;
            }

            if c.is_numeric() {
                current_buffer.push(c);
                continue;
            }

            if !self.header.dyn_().validate(c) {
                clear = true;
            } else {
                match c {
                    ',' => {
                        current_buffer = &mut right_buffer;
                    }
                    ')' => {
                        found = true;
                    }
                    _ => {
                        clear = true;
                    }
                }
            }

            if found {
                self.header.clear();
                break;
            }

            if clear {
                self.header.clear();
                left_buffer.clear();
                right_buffer.clear();
                current_buffer = &mut left_buffer;
                clear = false;
            }
        }

        if !found {
            self.header.clear();
            left_buffer.clear();
            right_buffer.clear();
            current_buffer = &mut left_buffer;
        }


        (left_buffer, right_buffer)
    }
}

fn main() -> anyhow::Result<()> {
    let input: &str = "mul(1,2)%mul(3,4)&mul(5,6)!@^mul(6,7)%mul(8,9)&mul(10,11)!@^do_not_mul(5,5)%mul(12,13)&mul(14,15)!@^mul(15,16)%mul(17,18)&mul(19,20)!\
    @^mul(20,21)%mul(22,23)&mul(24,25)!@^mul(25,26)%mul(27,28)&mul(29,30)!@^mul(30,31)%mul(32,33)&mul(34,35)!@^mul(35,36)%mul(37,38)&mul(39,40)!@^mul(40,41)\
    %mul(42,43)&mul(44,45)!@^mul(46,47)%mul(48,49)&mul(50,51)!@^mul(51,52)%mul(53,54)&mul(55,56)!@^mul(56,57)%mul(58,59)&mul(60,61)!@^mul(61,62)%mul(63,64)&mul(65,66)!\
    @^mul(66,67)%mul(68,69)&mul(70,71)!@^mul(71,72)%mul(73,74)&mul(75,76)!@^mul(76,77)%mul(78,79)&mul(80,81)!@^mul(81,82)%mul(83,84)&mul(85,86)!@^mul(86,87)%\
    mul(88,89)&mul(90,91)!@^mul(91,92)%mul(93,94)&mul(95,96)!@^mul(96,97)%mul(98,99)&mul(100,101)!@^mul(100,200)thenmul(101,202)mul(102,204)";

    let mut instruction_mul = InstructionMul::new(input);

    println!("{:?}", instruction_mul.process()?);
    Ok(())
}



