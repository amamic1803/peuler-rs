use crate::Problem;
use crate::math::newtons_method;
use std::sync::LazyLock;

problem!(Problem0034, 34, "Digit Factorials");

impl Problem for Problem0034 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        // first we want to find some upper bound for the problem
        // let n be the number of digits in the number
        // then the maximum sum of factorials of digits for numbers of size n is
        // 9! * n
        // we want that to be less than
        // 10^n
        // now we have inequality like this
        // 9! * n < 10^n
        // we solve for n using newton's method

        let n = newtons_method(
            20.0,
            0.01,
            |n| 10.0_f64.powf(n) - FACTORIALS[9] as f64 * n,
            |n| 10.0_f64.powf(n) * 10.0_f64.ln() - FACTORIALS[9] as f64,
        )
        .ceil() as u64;

        // we can calculate the upper bound now
        let upper_bound = FACTORIALS[9] * n;

        // now that we know the upper bound, we just start checking numbers
        let mut sum = 0;
        for num in 10..(upper_bound + 1) {
            let mut temp = num;
            let mut digits_sum = 0;
            while temp > 0 {
                digits_sum += FACTORIALS[(temp % 10) as usize];
                temp /= 10;
            }
            if digits_sum == num {
                sum += num;
            }
        }

        sum.to_string()
    }
}

// pre-calculated factorials
static FACTORIALS: LazyLock<[u64; 10]> = LazyLock::new(|| {
    let mut array = [1; 10];

    for i in 2..10 {
        array[i] = array[i - 1] * i as u64;
    }

    array
});
