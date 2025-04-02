// 实现一个命令行工具，对指定目录下的所有文本文件进行搜索，将匹配结果汇总后输出。
// 为增强可玩性和综合性，该工具需要支持：
// - 命令行参数（接收搜索关键词、目录路径、是否忽略大小写等）。
// - 并发搜索。
// - 消息通信。
// - 数据结构。
// - 错误处理。
// - 文件操作。
// - 迭代器与泛型（文本行迭代、搜索函数可考虑使用泛型或 trait 做一定延伸）。
// - 可选扩展：忽略大小写、正则匹配、统计行数或文件数等。

// cargo run --bin question6 src ./
// cargo build --bin question6

use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
};
use walkdir::WalkDir;

// 命令行参数结构
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
struct AppConfig {
    /// 要搜索的关键词
    keyword: String,
    /// 要搜索的目录路径
    directory: PathBuf,
    /// 是否忽略大小写
    #[arg(short, long)]
    ignore_case: bool,
    /// 使用正则表达式匹配
    #[arg(short, long)]
    regex: bool,
}

// 匹配结果结构
#[derive(Debug)]
struct MatchResult {
    file_path: PathBuf,
    line_number: usize,
    content: String,
}

fn main() -> Result<()> {
    let config = AppConfig::parse();
    let (tx, rx) = mpsc::channel();

    // 遍历目录并生成搜索线程
    for entry in WalkDir::new(&config.directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let tx = tx.clone();
        let config = config.clone();
        let path = entry.path().to_path_buf();

        thread::spawn(move || {
            if let Ok(results) = search_file(&path, &config) {
                for result in results {
                    tx.send(result).unwrap();
                }
            }
        });
    }

    drop(tx); // 关闭发送端

    // 收集并排序结果
    let mut matches: Vec<_> = rx.iter().collect();
    matches.sort_by(|a, b| a.file_path.cmp(&b.file_path));

    // 输出结果
    for m in matches {
        println!(
            "{}:{}: {}",
            m.file_path.display(),
            m.line_number,
            m.content.trim()
        );
    }

    Ok(())
}

fn search_file(path: &Path, config: &AppConfig) -> Result<Vec<MatchResult>> {
    let file = File::open(path).with_context(|| format!("无法打开文件: {}", path.display()))?;
    let reader = BufReader::new(file);
    let mut results = Vec::new();

    let pattern = if config.ignore_case {
        config.keyword.to_lowercase()
    } else {
        config.keyword.clone()
    };

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let content = if config.ignore_case {
            line.to_lowercase()
        } else {
            line.clone()
        };

        let matched = if config.regex {
            let re = regex::Regex::new(&config.keyword)?;
            re.is_match(&content)
        } else {
            content.contains(&pattern)
        };

        if matched {
            results.push(MatchResult {
                file_path: path.to_path_buf(),
                line_number: line_num + 1,
                content: line,
            });
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use predicates::prelude::*;
    use tempfile::tempdir;

    #[test]
    fn test_basic_search() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.txt");
        std::fs::write(&file_path, "hello\nworld\nhello world")?;

        let mut cmd = Command::cargo_bin("question6")?;
        cmd.args(&["hello", dir.path().to_str().unwrap()])
            .assert()
            .success()
            .stdout(predicate::str::contains("hello"))
            .stdout(predicate::str::contains("hello world"));

        Ok(())
    }

    #[test]
    fn test_case_insensitive() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.txt");
        std::fs::write(&file_path, "Hello\nWORLD")?;

        let mut cmd = Command::cargo_bin("question6")?;
        cmd.args(&["hello", dir.path().to_str().unwrap(), "-i"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Hello"));

        Ok(())
    }

    #[test]
    fn test_regex_search() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.txt");
        std::fs::write(&file_path, "123\n456\n789")?;

        let mut cmd = Command::cargo_bin("question6")?;
        cmd.args(&[r"\d{3}", dir.path().to_str().unwrap(), "-r"])
            .assert()
            .success()
            .stdout(predicate::str::contains("123"))
            .stdout(predicate::str::contains("456"))
            .stdout(predicate::str::contains("789"));

        Ok(())
    }
}
