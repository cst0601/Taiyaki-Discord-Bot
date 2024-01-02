//
// command.rs
//
use std::collections::HashMap;
use log::{warn};

pub struct Param<T> {
    name: String,
    description: String,
    value: T,
}

#[derive(PartialEq)]
pub enum ParamType{
    STRING,
    INTEGER,
    BOOLEAN,
}

pub struct Command {
    name: String,
    description: String,
    str_params: HashMap<String, Param<String>>,
    int_params: HashMap<String, Param<i32>>,
    bool_params: HashMap<String, Param<bool>>,
    is_priviledged_command: bool,
}

impl Command {

    fn new(name: &str, description: &str, is_priviledged: bool) -> Command {
        Command {
            name: name.to_string(),
            description: description.to_string(),
            str_params: HashMap::new(),
            int_params: HashMap::new(),
            bool_params: HashMap::new(),
            is_priviledged_command: is_priviledged,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_option(mut self, param_type: ParamType, option_name: & str, description: & str) -> Command {
        if param_type == ParamType::STRING {
            self.str_params.insert(
                option_name.to_string(),
                Param {
                    name: option_name.to_string(),
                    description: description.to_string(),
                    value: "".to_string(),
                }
            );
        }
        else if param_type == ParamType::INTEGER {
            self.int_params.insert(
                option_name.to_string(),
                Param {
                    name: option_name.to_string(),
                    description: description.to_string(),
                    value: 0,
                }
            );
        }
        else if param_type == ParamType::BOOLEAN {
            self.bool_params.insert(
                option_name.to_string(),
                Param {
                    name: option_name.to_string(),
                    description: description.to_string(),
                    value: false,
                }
            );
        }
        else {
            warn!("Invalid option type.");
        }
        self
    }

    fn parse(&self, command_str: &str) -> Result<(), String> {
        let collection = command_str.split(" ").collect::<Vec<&str>>();
        for part in 1..collection.len() {

        }
        Ok(())
    }

    fn get_option_str(&self, option_name: &str) -> &str {
        "test"
    }

    fn get_option_int(&self, option_name: &str) -> i32 {
        1
    }

    fn get_option_bool(&self, option_name: &str) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::command::Command;
    use crate::command::ParamType;

    #[test]
    fn create_command() {
        let command = Command::new("test", "Test Command", false);
        assert_eq!(command.get_name(), "test");
    }

    #[test]
    fn add_option() {
        let command = Command::new("test", "Test Command", false)
            .add_option(ParamType::STRING, "option1", "option description");
        assert_eq!(command.get_name(), "test");
    }

    #[test]
    fn parse() {
        let command = Command::new("create", "Create new character", false)
            .add_option(ParamType::STRING, "name", "Character name")
            .add_option(ParamType::INTEGER, "point", "Some sort of point")
            .add_option(ParamType::BOOLEAN, "flag", "Parameter without value");
        let input = ">create --name chikuma --point 10 --flag";
        match command.parse(input) {
            Ok(()) => assert_eq!(1, 1),
            Err(e) => assert_eq!(1, 2),
        }

        assert_eq!(command.get_option_str("name"), "chikuma");
        assert_eq!(command.get_option_int("point"), 10);
        assert_eq!(command.get_option_bool("flag"), true);
    }
}