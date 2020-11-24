struct Command<'a> {
  text: &'a str

}

pub fn run(command: &str) {
  if validate(command) {
    execute(command);
  }
}

fn parse(command: &str) -> Result<Command, &str> {
  let parts = command.split(" ");
  println!("{:?}", parts);
  Ok(Command {
    text: command
  })
}

fn validate(command: &str) -> bool {
  return true;
}

fn execute(command: &str) {

}
