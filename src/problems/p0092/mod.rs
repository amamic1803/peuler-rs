use crate::Problem;
use crate::math::digits;

problem!(Problem0092, 92, "Square Digit Chains");

impl Problem for Problem0092 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        // index 0 is not used
        // None = not yet calculated
        // Some(true) = ends at 89
        // Some(false) = ends at 1
        let mut cache: Vec<Option<bool>> = vec![None; LIMIT];
        cache[1] = Some(false);
        cache[89] = Some(true);

        // process all numbers in the cache
        for i in 2..LIMIT {
            if cache[i].is_none() {
                cache[i] = Some(process_number(i as u64, &mut cache));
            }
        }

        // count the number of true values in the cache
        cache
            .iter()
            .filter(|&&x| x == Some(true))
            .count()
            .to_string()
    }
}

const LIMIT: usize = 10_000_000;

fn process_number(num: u64, cache: &mut Vec<Option<bool>>) -> bool {
    // if num is 1 return false
    // if num is 89 return true
    // otherwise, calculate the next number
    // if the next number is in the cache, return the result
    // otherwise, calculate the result, store it in the cache, and return it
    // if the next number is greater than the cache, calculate the result and return it (don't store it in the cache)
    match num {
        1 => false,
        89 => true,
        _ => {
            let next = digits(num, 10).map(|x| (x as u64).pow(2)).sum();

            if next as usize >= cache.len() {
                process_number(next, cache)
            } else {
                match cache[next as usize] {
                    Some(true) => true,
                    Some(false) => false,
                    None => {
                        let result = process_number(next, cache);
                        cache[next as usize] = Some(result);
                        result
                    }
                }
            }
        }
    }
}
