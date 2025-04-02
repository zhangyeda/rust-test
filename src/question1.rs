// 从命令行读取一个整数 n（若读取失败或没有输入则默认 n = 5）。
// 打印从 1 到 n 的所有整数，每行一个。
// 若该整数可以被 3 整除，则在数字后面附加输出 "Fizz"；若可以被 5 整除，则附加输出 "Buzz"；若同时满足可以被 3 和 5 整除的情况，则输出 "FizzBuzz"。

use std::env;
use std::io::Write;

fn parse_input() -> Option<i32> {
    env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
}

fn question1<W: Write>(n: i32, writer: &mut W) {
    for i in 1..=n {
        let result = match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => format!("{}Fizz", i),
            (_, 0) => format!("{}Buzz", i),
            _ => i.to_string(),
        };
        writeln!(writer, "{}", result).unwrap();
    }
}

fn run() {
    let n = parse_input().unwrap_or(5);
    let mut stdout = std::io::stdout();
    question1(n, &mut stdout);
}

// fn main() {
//     run();
// }

#[test]
fn test_question1() {
    let test_cases = vec![
        (1, vec!["1"]),
        (3, vec!["1", "2", "3Fizz"]),
        (5, vec!["1", "2", "3Fizz", "4", "5Buzz"]),
        (15, vec![
            "1", "2", "3Fizz", "4", "5Buzz",
            "6Fizz", "7", "8", "9Fizz", "10Buzz",
            "11", "12Fizz", "13", "14", "FizzBuzz"
        ]),
    ];

    for (input, expected) in test_cases {
        let mut output = Vec::new();
        question1(input, &mut output);  // 直接传递缓冲区
        
        let output_str = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output_str.trim().split('\n').collect();
        assert_eq!(lines, expected);
    }
}
