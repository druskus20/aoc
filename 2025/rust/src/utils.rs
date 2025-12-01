/// Cleans input by removing empty lines and trimming whitespaces.
pub fn clean_input(input: &str) -> impl Iterator<Item = &str> + '_ {
    input.lines().map(|l| l.trim()).filter(|l| !l.is_empty())
}
