// 请从命令行读取一行字符串（例如 "apple banana pear banana apple banana"）。
// 使用空格进行拆分，统计每个单词出现的次数，并以从高到底的顺序输出。
// 如果出现次数相同，按单词本身的字典序从小到大排序输出。

use std::collections::HashMap;
use std::env;


fn process_input(input: &str) -> Vec<(String, usize)> {
    let mut counts = HashMap::new();
    
    for word in input.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
    });

    sorted
}


fn run() {
    let input = env::args()
        .nth(1)
        .unwrap_or_else(|| "".to_string());

    let results = process_input(&input);
    
    for (word, count) in results {
        println!("{}: {}", word, count);
    }
}

// fn main() {
//     run();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let result = process_input("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_basic_counting() {
        let input = "apple banana pear banana apple banana";
        let result = process_input(input);
        assert_eq!(result, vec![
            ("banana".into(), 3),
            ("apple".into(), 2),
            ("pear".into(), 1)
        ]);
    }
}
