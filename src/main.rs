use std::{
    io::{self, stdout, Write},
    time::{Duration, SystemTime},
};

use colored::Colorize;
use rand::{seq::SliceRandom, Rng};
use std::env;

fn main() {
    let user_path = if let Ok(path) = env::current_dir() {
        path
    } else {
        std::path::PathBuf::from("")
    };

    let mut warning_message: [[String; 3]; 3] = [
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

    println!("Enter number of packages to install (default: 10) and expected time in seconds (default: 5): (Using space to separate)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let num_packages: usize = parts.next().unwrap_or("10").parse().unwrap_or(10);
    let duration = Duration::new(parts.next().unwrap_or("5").parse().unwrap_or(5), 0);
    let start_time = SystemTime::now();
    let section_time = Duration::from_secs_f64(duration.as_secs_f64() / num_packages as f64);

    let mut rng = rand::rng();

    println!(
        "{}> npm {} test-package",
        user_path.display(),
        "i".bright_yellow()
    );
    println!("");

    let mut finished_packages = 0;

    loop {
        let elapsed = start_time.elapsed().unwrap_or(Duration::from_secs(0));

        if elapsed.as_secs_f64() >= duration.as_secs_f64() {
            break;
        }

        if finished_packages < num_packages {
            let package_start_time = SystemTime::now();

            let mut finish_idel_tree = false;
            loop {
                let package_elapsed = SystemTime::now()
                    .duration_since(package_start_time)
                    .unwrap_or(Duration::from_secs(0));

                let stage_one_time = section_time / 2;

                if package_elapsed.as_secs_f64() < stage_one_time.as_secs_f64() {
                    let stage_one_precentage =
                        package_elapsed.as_secs_f64() / stage_one_time.as_secs_f64() * 10.0;

                    print!(
                        "{} - idealTree:@humanfs/node   \r",
                        progress_view(stage_one_precentage.floor() as i32)
                    );
                } else {
                    if finish_idel_tree == false {
                        print!("\x1B[1A\x1B[2K");
                        print!("\r");

                        if rng.random_range(0..10) % 2 == 0 {
                            let random_number = rng.random_range(0..(warning_message.len()-1));

                            warning_message.shuffle(&mut rng);
                            let slice = &warning_message[0..random_number];

                            for message in slice {
                                display_warning_message(message);
                            }

                            // for message in &warning_message {
                            //     display_warning_message(message);
                            // }
                        }

                        println!("");

                        finish_idel_tree = true;
                    }

                    let stage_two_precentage = ((package_elapsed - stage_one_time).as_secs_f64()
                        / stage_one_time.as_secs_f64()
                        * 10.0)
                        .floor();

                    print!(
                        "{} - reify: <testing-package>: fetching tarball\r",
                        progress_view(stage_two_precentage as i32)
                    );
                }

                if package_elapsed >= section_time {
                    finished_packages += 1;
                    break;
                }
            }
        }
    }

    println!("");

    print!("\x1B[1A\x1B[2K");
    stdout().flush().unwrap();

    println!(
        "up to date, audited {} packages in {}ms",
        num_packages,
        duration.as_secs()
    );
    println!("");

    println!("{} packages are looking for funding", num_packages);
    println!("  run `npm fund` for details");

    println!("");
    println!(
        "found {} vulnerabilities",
        rng.random_range(0..30).to_string().green()
    );

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
