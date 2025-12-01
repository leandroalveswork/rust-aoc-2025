use crate::mass_parser::read_lines;

pub struct Dial {
    pub position: i16,
    pub times_at_0: u16,
}

impl Dial {
    fn rotate(self, difference: i16) -> Dial {
        let new_position: i16 = (self.position + difference) % 100;
        Dial { position: new_position, times_at_0: self.times_at_0 + (if new_position == 0 { 1 } else { 0 }) }
    }
    fn new() -> Dial {
        Dial { position: 50, times_at_0: 0 }
    }
}

fn read_difference(text: String) -> Option<i16> {
    let (first, last) = text.split_at(1);
    if first == "L" {
        return i16::from_str_radix(last, 10).ok().map(|x| -x);
    }

    i16::from_str_radix(last, 10).ok()
}

pub async fn answer() -> Option<()> {
    let differences = read_lines("day-01.txt").await?;
    // let differences = vec!["L40", "R15", "L25", "R60", "R40"]
    //     .iter()
    //     .map(|x| x.to_string())
    //     .collect::<Vec<String>>();
    
    let end_dial = differences
        .iter()
        .map(|differ| read_difference(differ.to_owned()))
        .filter(|differ| differ.is_some())
        .map(|differ| differ.unwrap())
        .fold(Dial::new(), |dial, differ| dial.rotate(differ));

    println!("{0}", end_dial.times_at_0);
    Some(())
}

pub struct SensibleDial {
    pub position: i16,
    pub times_at_0: i16,
}

fn abs_modulo(x: i16, modu: i16) -> i16 {
    x % modu + (if x % modu < 0 { modu } else { 0 })
}

impl SensibleDial {
    fn rotate(self, difference: i16) -> SensibleDial {
        let clicks: i16 =
            if self.position + difference <= 0 {
                let abs_value = -(self.position + difference);
                let already_first_ringed = self.position == 0;
                abs_value / 100 + (if already_first_ringed { 0 } else { 1 })
            } else {
                (self.position + difference) / 100
            };

        let new_position: i16 = abs_modulo(self.position + difference, 100);
        SensibleDial { position: new_position, times_at_0: self.times_at_0 + clicks }
    }
    fn new() -> SensibleDial {
        SensibleDial { position: 50, times_at_0: 0 }
    }
}

pub async fn answer2() -> Option<()> {
    let differences = read_lines("day-01.txt").await?;
    // let differences = vec!["L40", "L15", "R30", "L25", "R60", "R40"]
    // let differences = vec!["L40", "L15", "R30", "L25", "R60", "R40", "L300"]
    //     .iter()
    //     .map(|x| x.to_string())
    //     .collect::<Vec<String>>();

    let end_dial = differences
        .iter()
        .map(|differ| read_difference(differ.to_owned()))
        .filter(|differ| differ.is_some())
        .map(|differ| differ.unwrap())
        .fold(SensibleDial::new(), |dial, differ| dial.rotate(differ) );

    println!("{0}", end_dial.times_at_0);
    Some(())
}
