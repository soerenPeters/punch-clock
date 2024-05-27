use app::*;

#[test]
fn test_clock_in_and_out_worktime_is_difference() {
    let mut work_day = Workday::new();
    work_day.clock_in(Time(10,0));
    work_day.clock_out(Time(11,5));

    let WorkTime(hours, minutes) = work_day.work_time();

    assert_eq!(hours, 1);
    assert_eq!(minutes, 5);
}