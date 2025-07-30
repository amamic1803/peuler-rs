use crate::Problem;

problem!(Problem0082, 82, "Path Sum: Three Ways");

impl Problem for Problem0082 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let input = include_str!("0082_matrix.txt");
        let input_matrix = input
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .split(',')
                    .map(|num_str| num_str.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        let mut work_matrix = input_matrix.clone();

        // we start from the second to last column and dynamically calculate minimum path sums
        // i - row, j - column
        for j in (0..(work_matrix[0].len() - 1)).rev() {
            for i in 0..work_matrix.len() {
                let mut min_sum = work_matrix[i][j + 1]; // right

                // try to go up
                let mut up_sum = 0;
                for k in (0..i).rev() {
                    up_sum += input_matrix[k][j];
                    if up_sum > min_sum {
                        break;
                    }
                    let candidate_min_sum = up_sum + work_matrix[k][j + 1];
                    if candidate_min_sum < min_sum {
                        min_sum = candidate_min_sum;
                    }
                }

                // try to go down
                let mut down_sum = 0;
                for k in (i + 1)..work_matrix.len() {
                    down_sum += input_matrix[k][j];
                    if down_sum > min_sum {
                        break;
                    }
                    let candidate_min_sum = down_sum + work_matrix[k][j + 1];
                    if candidate_min_sum < min_sum {
                        min_sum = candidate_min_sum;
                    }
                }

                // add the minimum sum from the current cell to the value in the cell
                work_matrix[i][j] += min_sum;
            }
        }

        // get the minimum path sum from the first column
        work_matrix
            .into_iter()
            .map(|row| row[0]) // take the first column
            .min()
            .unwrap()
            .to_string()
    }
}
