mod windows_commands {

    trait WindowsCommand {
        fn is_command(cmd: &str) -> bool;
        fn generate_bash(cmd: &str) -> String;
    }

    fn is_filepath(string: &str) -> bool {
        return string.contains('/') || string.contains('\\');
    }

    struct Dir;
    impl WindowsCommand for Dir {
        fn is_command(cmd: &str) -> bool {
            return cmd.to_lowercase().starts_with("dir");
        }

        fn generate_bash(cmd: &str) -> String {
            let mut out_string = String::new();
            for word in cmd.split_whitespace() {
                // If the word matches a option use its replacement
                // otherwise if it's not a filepath just remove it
                out_string.push_str(match word {
                    "dir"   => "ls",
                    "/p"    => "",
                    "/q"    => "-l",
                    "/w"    => "--width=5",
                    "/d"    => "--width=5",
                    "/a"    => "-A",
                    "/ad"   => "-A",
                    "/ah"   => "-A",
                    "/as"   => "-A",
                    "/al"   => "-A",
                    "/ar"   => "-A",
                    "/aa"   => "-A",
                    "/ai"   => "-A",
                    "/o"    => "",
                    "/on"   => "",
                    "/oe"   => "--sort=extension",
                    "/og"   => "",
                    "/os"   => "--sort=size -r",
                    "/od"   => "--sort=time -r",
                    "/tc"   => "--time=creation",
                    "/ta"   => "--time=access",
                    "/tw"   => "--time=modification",
                    "/s"    => "",
                    "/b"    => "",
                    "/l"    => "--sort=none",
                    "/n"    => "-l",
                    "/x"    => "",
                    "/c"    => "",
                    "/4"    => "",
                    "/r"    => "",
                    "/?"    => "--help",
                    _ => if is_filepath(word) {&word} else {""}
                });
                out_string.push_str(" ");
            }

            // Remove trailing whitespace
            out_string.truncate(out_string.len() - 1);

            return out_string;
        }
    }

    #[cfg(test)]
    mod dir_tests {
        use crate::windows_commands::WindowsCommand;
        use crate::windows_commands::Dir;

        #[test]
        fn simple() {
            assert_eq!(
                Dir::generate_bash("dir").as_str(),
                "ls"
            );
        }

        #[test]
        fn help() {
            assert_eq!(
                Dir::generate_bash("dir /?").as_str(),
                "ls --help"
            );
        }

        #[test]
        fn complicated() {
            assert_eq!(
                Dir::generate_bash("dir /q /a /od /tc").as_str(),
                "ls -l -A --sort=time -r --time=creation"
            );
        }

        #[test]
        fn path_specified() {
            assert_eq!(
                Dir::generate_bash("dir \"C:\\Users\"").as_str(),
                "ls \"C:\\Users\""
            );
        }
    }

    struct UnknownCommand;
    impl WindowsCommand for UnknownCommand {
        fn is_command(_cmd: &str) -> bool {
            // Everything can be an unknown command!
            return true;
        }

        fn generate_bash(cmd: &str) -> String {
            // Comment out the string and add a comment saying
            // it couldn't be translated
            return format!("# bat-to-sh COULD NOT TRANSLATE!!!!\n# {}", cmd);
        }
    }

    #[cfg(test)]
    mod unknown_command_tests {
        use crate::windows_commands::{UnknownCommand, WindowsCommand};

        #[test]
        fn simple() {
            assert_eq!(
                UnknownCommand::generate_bash("mycustomcommand /r /? \"C:\\Users\""),
                "# bat-to-sh COULD NOT TRANSLATE!!!!\n\
                # mycustomcommand /r /? \"C:\\Users\""
            );
        }
    }
}

