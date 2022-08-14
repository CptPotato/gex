use crossterm::{
    cursor,
    style::{Attribute, Color, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::{
    fmt,
    io::{stdin, stdout, BufRead, Write},
    process::Command,
};

pub struct BranchList {
    pub branches: Vec<String>,
    pub cursor: usize,
}

impl fmt::Display for BranchList {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use fmt::Write;
        for (i, branch) in self.branches.iter().enumerate() {
            if branch.starts_with('*') {
                write!(f, "{}", SetForegroundColor(Color::Yellow))?;
            }
            if i == self.cursor {
                let mut branch = branch.to_string();
                branch.insert_str(2, &format!("{}", Attribute::Reverse));
                write!(&mut branch, "{}", Attribute::Reset)?;
                writeln!(f, "{}{}", cursor::MoveToColumn(0), branch,)?;
            } else {
                writeln!(f, "{}{}", cursor::MoveToColumn(0), branch)?;
            }
            if branch.starts_with('*') {
                write!(f, "{}", SetForegroundColor(Color::Reset))?;
            }
        }
        Ok(())
    }
}

impl BranchList {
    pub fn new() -> Self {
        let mut branch_list = Self {
            branches: Vec::new(),
            cursor: 0,
        };
        branch_list.fetch();
        branch_list
    }

    pub fn fetch(&mut self) {
        let branches = Command::new("git")
            .arg("branch")
            .output()
            .expect("failed to run `git branch`");

        self.branches = std::str::from_utf8(&branches.stdout)
            .expect("broken stdout from `git branch`")
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();
    }

    pub fn checkout(&self) {
        Command::new("git")
            .args(["checkout", &self.branches[self.cursor][2..]])
            .output()
            .expect("failed to run `git checkout`");
    }

    pub fn checkout_new() {
        terminal::disable_raw_mode().expect("failed to exit raw mode");
        print!(
            "{}{}{}Name for the new branch: ",
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::All),
            cursor::Show
        );
        let _ = stdout().flush();

        // TODO: error reporting when the branch name is invalid rather than
        // silently ignore
        let input = stdin()
            .lock()
            .lines()
            .next()
            .expect("no stdin")
            .expect("malformed stdin");
        Command::new("git")
            .args(["checkout", "-b", &input])
            .output()
            .expect("failed to checkout new branch");

        terminal::enable_raw_mode().expect("failed to enter raw mode");
        print!("{}", cursor::Hide);
    }
}