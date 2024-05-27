pub mod greet;

pub struct WorkTime(pub i8, pub i8);
pub struct Time(pub i8, pub i8);

pub struct Workday {
    clock_in_time: Time,
    clock_out_time: Time,
}

impl Workday {
    pub fn new() -> Workday {
        Workday {
            clock_in_time: Time(0, 0),
            clock_out_time: Time(0, 0),
        }
    }
    pub fn work_time(&self) -> WorkTime {
        let Time(clock_in_hour, clock_in_min) = self.clock_in_time;
        let Time(clock_out_hour, clock_out_min) = self.clock_out_time;
        WorkTime(clock_out_hour - clock_in_hour, clock_out_min - clock_in_min)
    }

    pub fn clock_in(&mut self, time: Time) {
        self.clock_in_time = time;
    }

    pub fn clock_out(&mut self, time: Time) {
        self.clock_out_time = time;
    }
}
