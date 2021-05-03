#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn it_works() {
        Command::new("./test.sh").assert().success();
    }
}
