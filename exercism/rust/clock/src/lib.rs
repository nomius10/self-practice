#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    hrs : i32,
    min : i32
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{:02}:{:02}", self.hrs, self.min)
    }
}

/*  -21 % 4
 *      python => 3     (% is modulo)
 *      rust   => -1    (% is remainder)
 */
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hrs : (hours + minutes.div_euclid(60)).rem_euclid(24),
            min : minutes.rem_euclid(60)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hrs, self.min + minutes)
    }
}
