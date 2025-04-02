// 使用多线程并行计算某个函数的值或模拟并发任务。
// 需要创建 3 个线程同时进行下载，并在下载完成后将结果（例如“URL + 下载完成”）
// 通过消息通道（std::sync::mpsc）发送回主线程。主线程依次接收并打印结果。

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn download_task(url: &str, sender: mpsc::Sender<String>) {
    thread::sleep(Duration::from_secs(1));
    
    let result = format!("{} 下载完成", url);
    sender.send(result).unwrap();
}


fn run(urls: Vec<String>) -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    
    for url in urls { 
        let tx_clone = tx.clone();
        thread::spawn(move || {
            download_task(&url, tx_clone);
        });
    }
    
    drop(tx);

    let mut results: Vec<String> = rx.iter().collect();
    results.sort();
    
    for received in &results {
        println!("{}", received);
    }

    results
}

// fn main() {
//     let urls = [
//         "https://example.com/file1".to_string(),
//         "https://example.com/file2".to_string(),
//         "https://example.com/file3".to_string(),
//     ];
//     let _results= run(urls.into());
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_tasks() {
        let test_urls = ["url1".to_string(), "url2".to_string(), "url3".to_string()];

        let test_result = run(test_urls.into());
        
        let expected = vec![
            "url1 下载完成".to_string(),
            "url2 下载完成".to_string(),
            "url3 下载完成".to_string()
        ];
        assert_eq!(test_result, expected);
    }
}
