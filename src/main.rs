use std::{
    io::{self, stdout, Write},
    time::{Duration, SystemTime},
};

use colored::Colorize;
use rand::Rng;
use std::env;

fn main() {
    let user_path = if let Ok(path) = env::current_dir() {
        path
    } else {
        std::path::PathBuf::from("")
    };

    let warning_message: [[String; 3]; 3] = [
        [
            String::from("package: '@eslint/config-array@0.20.1',"),
            String::from("required: { node: '^18.18.0 || ^20.9.0 || >=21.1.0' },"),
            String::from("current: { node: 'v20.4.0', npm: '9.7.2' }"),
        ],
        [
            String::from("package: '@eslint/config-helpers@0.2.3',"),
            String::from("required: { node: '^18.18.0 || ^20.9.0 || >=21.1.0' },"),
            String::from("current: { node: 'v20.4.0', npm: '9.7.2' }"),
        ],
        [
            String::from("package: '@eslint/core@0.14.0',"),
            String::from("required: { node: '^18.18.0 || ^20.9.0 || >=21.1.0' },"),
            String::from("current: { node: 'v20.4.0', npm: '9.7.2' }"),
        ],
    ];

    let duration = Duration::new(3, 0);
    let start_time = SystemTime::now();

    let mut rng = rand::rng();

    let mut finish_idel_tree = false;

    println!(
        "{}> npm {} test-package",
        user_path.display(),
        "i".bright_yellow()
    );
    println!("");

    loop {
        let elapsed = start_time.elapsed().unwrap_or(Duration::from_secs(0));
        let remaining = duration.saturating_sub(elapsed);
        let stage_one_time = duration / 2;

        if elapsed.as_secs_f64() < stage_one_time.as_secs_f64() {
            let stage_one_precentage = elapsed.as_secs_f64() / stage_one_time.as_secs_f64() * 10.0;

            print!(
                "{} - idealTree:@humanfs/node   \r",
                progress_view(stage_one_precentage.floor() as i32)
            );
        } else {
            if finish_idel_tree == false {
                print!("\x1B[1A\x1B[2K");
                print!("\r");

                for message in &warning_message {
                    display_warning_message(message);
                }
                finish_idel_tree = true;
                println!("");
            }

            let stage_two_precentage =
                ((elapsed - stage_one_time).as_secs_f64() / stage_one_time.as_secs_f64() * 10.0)
                    .floor();

            print!(
                "{} - reify: <testing-package>: fetching tarball\r",
                progress_view(stage_two_precentage as i32)
            );
        }

        if remaining <= Duration::from_secs(0) {
            break;
        }
    }
    println!("");

    print!("\x1B[1A\x1B[2K");
    stdout().flush().unwrap();

    println!(
        "up to date, audited {} packages in {}ms",
        rng.random_range(0..100),
        rng.random_range(0..100)
    );
    println!("");

    println!(
        "{} packages are looking for funding",
        rng.random_range(0..30)
    );
    println!("  run `npm fund` for details");

    println!("");
    println!("found {} vulnerabilities", "0".green());

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn progress_view(_percentage: i32) -> String {
    let mut message = String::from("[");

    for x in 1..11 {
        if _percentage < x {
            message.push_str("=");
        } else {
            message.push_str("█");
        }
    }
    message.push_str("]");
    message
}

fn display_warning_message(infos: &[String]) {
    println!("");

    println!(
        "npm {} {} Unsupported engine {{",
        "WARN".black().on_yellow(),
        "EBADENGINE".purple()
    );

    for info in infos {
        println!(
            "npm {} {}   {}",
            "WARN".black().on_yellow(),
            "EBADENGINE".purple(),
            info
        );
    }

    println!(
        "npm {} {} }}",
        "WARN".black().on_yellow(),
        "EBADENGINE".purple()
    );
}
