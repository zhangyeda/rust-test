// 从命令行参数接收一个文件路径，例如 input.txt。如果没有提供路径或文件无法打开，给出相应的错误提示并退出。
// 读取该文件的所有文本内容，统计文本中一共包含的字符数（不含换行符）与行数，并将结果写入 output.txt。
// 若 output.txt 文件已存在，可以选择直接覆盖或者追加，任选其一，但需要在程序里明确注释或说明处理方式。

use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn get_input_file() -> Result<File, std::io::Error> {
    let file_path = env::args().nth(1).expect("请提供文件路径，例如：./input.txt");
    let file = File::open(&file_path);
    file
}

fn count(input_file: &mut File) -> (i32, i32) {
    let mut input_content = String::new();
    input_file.read_to_string(&mut input_content).expect("无法读取文件内容");

    let char_count = input_content
        .chars()
        .filter(|c| !c.is_control() && *c != '\n' && *c != '\r')
        .count();

    let line_count = input_content.lines().count();
    (char_count as i32, line_count as i32)
}

fn run() {
    let mut input_file = get_input_file().expect("无法打开文件");

    let (char_count, line_count) = count(&mut input_file);

    // 写入输出文件（使用覆盖模式）
    let mut output_file = File::create("output.txt").expect("无法创建输出文件");
    writeln!(&mut output_file, "字符数: {}", char_count).expect("写入失败");
    writeln!(&mut output_file, "行数: {}", line_count).expect("写入失败");
}

// fn main() {
//     run();
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_question4() { 
        // 执行统计逻辑
        let mut file = File::open("src/input.txt").unwrap();
        let (char_count, line_count) = count(&mut file);

        assert_eq!(char_count, 17);
        assert_eq!(line_count, 5);
    }
}
