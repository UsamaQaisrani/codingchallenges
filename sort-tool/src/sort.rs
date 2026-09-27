use clap::ValueEnum;
use common::input_reader::read_string;

#[derive(Debug, Clone, ValueEnum)]
pub enum SortType {
    Standard,
    Merge,
    Radix,
}

pub fn sort(
    path: Option<&str>,
    unique: bool,
    sort_type: SortType,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut words: Vec<String> = read_string(path)?
        .lines()
        .map(|line| line.to_string())
        .collect();

    match sort_type {
        SortType::Radix => {
            words = radix_sort(&words);
        }
        SortType::Merge => {
            words = merge_sort(&words);
        }
        _ => {
            words.sort();
        }
    }
    if unique {
        words.dedup();
    }

    Ok(words)
}

fn radix_sort(list: &[String]) -> Vec<String> {
    let mut count_array = vec![0usize; 257];

    let max_word_length = list.iter().map(|s| s.len()).max().unwrap_or(0);

    let mut input = list.to_vec();
    let mut output = vec![String::new(); list.len()];

    for i in (0..max_word_length).rev() {
        count_array.fill(0);

        // Count
        for word in &input {
            let index = if i < word.len() {
                word.as_bytes()[i] as usize + 1
            } else {
                0
            };

            count_array[index] += 1;
        }

        // Prefix sums
        for j in 1..count_array.len() {
            count_array[j] += count_array[j - 1];
        }

        // Remap
        for word in input.iter().rev() {
            let index = if i < word.len() {
                word.as_bytes()[i] as usize + 1
            } else {
                0
            };

            count_array[index] -= 1;
            let updated_index = count_array[index];

            output[updated_index] = word.clone();
        }

        std::mem::swap(&mut input, &mut output);
    }

    input
}

fn merge_sort(values: &[String]) -> Vec<String> {
    if values.len() <= 1 {
        return values.to_vec();
    }

    let mid = values.len() / 2;
    let left = merge_sort(&values[..mid]);
    let right = merge_sort(&values[mid..]);

    merge(&left, &right)
}

fn merge(left: &[String], right: &[String]) -> Vec<String> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);

    result
}
