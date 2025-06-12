use rand::Rng;
use std::{
    io::{Write, stdin, stdout},
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

fn main() {
    println!("=== Florr Code Practice ===");
    let mut buf = String::new();

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
        print!("> ");
        let _ = stdout().flush();

        // 5) 计时并读取用户输入
        buf.clear();
        let start = Instant::now();
        if stdin().read_line(&mut buf).is_err() {
            break;
        }
        let elapsed = Instant::now().duration_since(start).as_secs_f64();

        // 6) 判断并反馈
        let input = buf.trim_end();
        if input == word {
            println!("Success: {:.3} s", elapsed);
        } else {
            println!("Failure: {:.3} s (you typed `{}`)", elapsed, input);
        }
    }
}
