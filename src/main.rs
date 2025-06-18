use rand::Rng;
use regex::Regex;
use std::{
    io::{Result, Write, stdin, stdout},
    thread,
    time::{Duration, Instant},
};

/// 从 [a–z0–9] 中随机选 6 个字符
fn generate_word() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..6)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

// 用户输入流处理
fn input_code() -> Result<String> {
    let mut res = String::new();
    let re = Regex::new(r"\s+").unwrap();
    loop {
        res.clear();
        print!("> ");
        let _ = stdout().flush();

        stdin().read_line(&mut res)?;
        let formatted = re.replace_all(&res, "").into_owned();
        if formatted.len() == 6 {
            return Ok(formatted);
        }
    }
}

fn main() {
    println!("=== Florr Code Practice ===");

    loop {
        // 1) 随机 2–10 秒
        let delay_ms = rand::thread_rng().gen_range(2_000..=10_000);

        // 2) 提示用户“即将出现code”
        print!("\nNext code will appear soon… ");
        let _ = stdout().flush();

        // 3) 等待
        thread::sleep(Duration::from_millis(delay_ms));

        // 4) 产生并显示单词
        let word = generate_word();
        println!("\nCode: {}", word);
        let _ = stdout().flush();

        // 5) 计时并读取用户输入
        let start = Instant::now();
        let input = input_code().unwrap();
        let elapsed = Instant::now().duration_since(start).as_secs_f64();

        // 6) 判断并反馈
        if input == word {
            println!("Success: {:.3} s", elapsed);
        } else {
            println!("Failure: {:.3} s (you typed `{}`)", elapsed, input);
        }
    }
}
