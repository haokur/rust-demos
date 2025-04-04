use crate::utils::process;
use crate::utils::process::Program;
use crate::utils::text;
use inquire;
use inquire::list_option::ListOption;

pub fn run(port_or_names: Vec<&str>) {
    let matched_programs = process::fetch_all_matched_program(port_or_names);

    if matched_programs.len() > 0 {
        // 多选要kill的进程
        let programs_options: Vec<_> = matched_programs
            .iter()
            .enumerate()
            .map(|(i, x)| {
                format!(
                    "{}. [{}] - ({}) - {}",
                    text::pad_left(&(i + 1).to_string(), 3, '0'),
                    text::pad_left(&*x.pid.to_string(), 5, '_'),
                    text::pad_left(&*x.port.to_string(), 5, '_'),
                    x.name.clone(),
                )
            })
            .collect();

        let formatted_output = |selected: &[ListOption<&String>]| {
            let mut output_str = String::from("\n\nYou Selected:\n");
            let options_str = selected
                .iter()
                .map(|opt| format!("* {}", opt.value)) // 获取每个选项的 `item` 字段
                .collect::<Vec<String>>()
                .join("\n");
            output_str.push_str(&options_str);

            output_str
        };

        let user_checked_kill_programs = inquire::MultiSelect::new(
            "please choose what program you want to kill",
            programs_options,
        )
        .with_page_size(100)
        .with_formatter(&formatted_output)
        .prompt()
        .unwrap();

        let choose_programs: Vec<Program> = user_checked_kill_programs
            .iter()
            .filter_map(|choose| {
                let index_str = choose.split(".").next()?;
                let index: usize = index_str.parse().ok()?;
                matched_programs.get(index - 1).cloned()
            })
            .collect();

        process::kill_programs(choose_programs, true);
    } else {
        println!("No match program found");
    }
}
