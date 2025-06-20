use crate::Problem;

problem!(Problem0019, 19, "Counting Sundays");

impl Problem for Problem0019 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        // Days in month (index 0 is not used)
        const DAYS_IN_MONTH: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        // how many sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
        // january, march, may, july, august, october, december = 31 days
        // april, june, september, november = 30 days
        // february = 28 or 29 days (leap year, divisible by 4 or 400 if century)
        // 1.1.1900 was Monday

        let mut year: u32 = 1900;
        let mut month: u32 = 1;
        let mut day: u32 = 1;

        let mut sundays: u32 = 0;

        while year <= 2000 {
            // every iteration we skip to the 1st of the next month

            // if day is divisible by 7, it's a sunday (we don't count 1900 since it's not in the 20th century)
            if (day % 7 == 0) && (year != 1900) {
                sundays += 1;
            }

            day += DAYS_IN_MONTH[month as usize];

            if month == 2 && (((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0)) {
                day += 1;
            } else if month == 12 {
                month = 0;
                year += 1;
            }
            month += 1;
        }

        sundays.to_string()
    }
}
