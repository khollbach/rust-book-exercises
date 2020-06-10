use std::error::Error;
use std::io;

use db::DB;

mod db;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let mut db = DB::new();
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        use parser::Command::*;
        match parser::parse_command(buf.trim())? {
            Update {
                department,
                employee,
            } => db.update(department, employee),
            ListDept(dept) => {
                println!("{}", db.employees(dept));
            }
            ListAll => {
                println!("\n{}", db.all_employees());
            }
            Quit => break,
        }
    }
    Ok(())
}
