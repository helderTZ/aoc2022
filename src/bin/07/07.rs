// TODO: WIP

// const COMMANDS: &'static str = include_str!("input.txt");

// #[derive(Debug)]
// struct Command<'a> {
//     cmd: &'a str,
//     out: &'a str,
// }

// impl Command<'_> {
//     fn new(s: &'_ str) -> Command<'_> {
//         let it = s.split_once('\n');
//         if it.is_some() {
//             Command::<'_>  { cmd: it.unwrap().0, out: it.unwrap().1 }
//         } else {
//             Command::<'_>  { cmd: s, out: "" }
//         }
//     }
// }

// #[derive(Debug)]
// struct Dir<'a> {
//     name: &'a str,
//     level: usize,
//     parent: Option<&'a mut Dir<'a>>,
//     dirs: Vec<Dir<'a>>,
//     files: Vec<File<'a>>,
// }

// #[derive(Debug)]
// struct File<'a> {
//     name: &'a str,
//     size: usize,
// }

// impl<'a> File<'a> {
//     fn new(name: &'a str, size: usize) -> Self {
//         Self { name: name.clone(), size }
//     }
// }

// impl<'a> Dir<'a> {
//     fn new(name: &'a str, level: usize, parent: Option<&'a mut Dir<'a>>) -> Self {
//         Self { name, level, parent, dirs: vec![], files: vec![] }
//     }
// }

// fn create_fs(commands: &str) {
//     let mut file_system: Dir;
//     let mut curr_cmd: &str;
//     let mut past_cmd: &str;
//     let mut curr_cmd_args: &str;
//     let mut past_cmd_args: &str;
//     let mut curr_level = 0;
//     let mut curr_dir: Option<&mut Dir> = None;
//     for cmd in commands.lines() {
//         let words: Vec<&str> = cmd.split_ascii_whitespace().collect();
//         match words[0] {
//             // lines starting with  '$' are commands
//             // they are succeeded by either 'cd <...>' or 'ls'
//             "$" => match words[1] {
//                 "cd" => {
//                     curr_cmd = words[1];
//                     curr_cmd_args = words[2];
//                     if curr_cmd_args == "/" {
//                         file_system = Dir::new(curr_cmd_args, curr_level, None);
//                         curr_level += 1;
//                     }
//                     else if curr_cmd_args == ".." {
//                         curr_level -= 1;
//                         curr_dir = curr_dir.unwrap().parent;
//                     }
//                 },
//                 "ls" => {
//                     curr_cmd = words[1];
//                     curr_cmd_args = "";
//                 }
//                 _ => unreachable!(),
//             }
//             // lines starting with 'dir' are the results of a previous 'ls'
//             "dir" => {
//                 let dir_name = words[1];
//                 let this_parent = Some(curr_dir.unwrap());
//                 let mut dir_tree: &mut Vec<Dir> = curr_dir.unwrap().dirs.as_mut();
//                 dir_tree.push(Dir::new(dir_name, curr_level, this_parent));
//             }
//             // lines starting with a number are the results of a previous 'ls'
//             _ => {
//                 let file_size = words[0].parse::<usize>().unwrap();
//                 let file_name = words[1];
//                 curr_dir.unwrap().files.push(File::new(file_name, file_size));
//             }
//         }
//     }
// }

// fn main() {
//     let mut commands: Vec<Command> = vec![];
//     let it: Vec<&str> = COMMANDS.split('$').collect();
//     for cmd in it.into_iter() {
//         commands.push(Command::new(cmd));
//     }

//     let mut curr_path: &str;
//     for cmd in commands {
//         println!("{:?}", cmd);
//         let mut it = cmd.cmd.split_ascii_whitespace();
//         let command_str = it.next().unwrap();
//         let command_arg = it.next().unwrap();
//         match command_str {
//             "cd" => curr_path = command_arg.clone(),
//             "ls" => {
//                 // let mut it = command_arg
//             },
//             _ => panic!("Got {}, not 'ls' or 'cd'", command_str),
//         }
//     }

// }