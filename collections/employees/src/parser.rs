#[derive(Debug, PartialEq, Eq)]
pub enum Command<'a> {
    Update {
        employee: &'a str,
        department: &'a str,
    },
    ListDept(&'a str),
    ListAll,
    Quit,
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    let mut words = input.split_whitespace();
    let first = match words.next() {
        Some(first) => first.to_ascii_lowercase(),
        None => return Err("No command given.".into()),
    };

    use Command::*;
    match &first[..] {
        "add" => {
            let empl = words.next();
            let to = words.next();
            let dept = words.next();
            let none = words.next();
            if dept.is_none() {
                Err("Not enough arguments to Add. Expected 3.".into())
            } else if none.is_some() {
                Err("Too many arguments to Add. Expected 3.".into())
            } else if to != Some("to") {
                Err("Add usage: Add X to Y".into())
            } else {
                Ok(Update {
                    employee: empl.unwrap(),
                    department: dept.unwrap(),
                })
            }
        }
        "list" => {
            if let Some(dept) = words.next() {
                if let Some(_) = words.next() {
                    Err("List expects at most one argument.".into())
                } else {
                    Ok(ListDept(dept))
                }
            } else {
                Ok(ListAll)
            }
        }
        "quit" => {
            if let None = words.next() {
                Ok(Quit)
            } else {
                Err("Quit expects no arguments.".into())
            }
        }
        _ => Err(format!("Invalid command: {}", first)),
    }
}

#[cfg(test)]
mod test {
    use super::Command::*;
    use super::*;

    fn helper(input: &str, expected: Command) {
        assert_eq!(Ok(expected), parse_command(input));
    }

    fn helper_invalid(input: &str) {
        assert!(parse_command(input).is_err());
    }

    #[test]
    fn test_update() {
        helper(
            "Add Sally to Engineering",
            Update {
                employee: "Sally",
                department: "Engineering",
            },
        );
    }

    #[test]
    fn test_list_dept() {
        helper("List Engineering", ListDept("Engineering"));
    }

    #[test]
    fn test_list_all() {
        helper("List", ListAll);
    }

    #[test]
    fn test_quit() {
        helper("Quit", Quit);
    }

    #[test]
    fn test_case_insensitive() {
        helper("qUiT", Quit);
    }

    #[test]
    fn test_whitespace() {
        helper("   list   Engineering \n", ListDept("Engineering"));
    }

    #[test]
    fn test_invalid() {
        helper_invalid("   list a    Engineering ");
        helper_invalid("Lista Engineering");
        helper_invalid("");
        helper_invalid("asdf");
        helper_invalid(" ");
        helper_invalid("q");
    }
}
