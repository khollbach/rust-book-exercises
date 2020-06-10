use std::collections::HashMap;
use std::fmt;

type Dept = String;
type Empl = String;

pub struct DB {
    depts: HashMap<Dept, Vec<Empl>>,
}

/// Sorted by name.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct DeptSummary(Vec<Empl>);

impl fmt::Display for DeptSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for name in &self.0 {
            writeln!(f, "{}", name)?;
        }
        Ok(())
    }
}

/// Sorted by department name.
pub struct CompanySummary(Vec<(Dept, DeptSummary)>);

impl fmt::Display for CompanySummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (name, dept) in &self.0 {
            write!(f, "  {}:\n{}", name, dept)?;
        }
        Ok(())
    }
}

impl DB {
    pub fn new() -> Self {
        Self {
            depts: HashMap::new(),
        }
    }

    pub fn update(&mut self, department: &str, employee: &str) {
        let dept = String::from(department);
        let empl = String::from(employee);
        self.depts.entry(dept).or_insert(vec![]).push(empl);
    }

    pub fn employees(&self, department: &str) -> DeptSummary {
        let mut empls = self.depts.get(department).unwrap_or(&vec![]).clone();
        empls.sort_unstable();
        DeptSummary(empls)
    }

    pub fn all_employees(&self) -> CompanySummary {
        let mut result = Vec::new();
        for dept in self.depts.keys() {
            result.push((dept.to_string(), self.employees(dept)));
        }
        // Sort by dep't name.
        result.sort_unstable();
        CompanySummary(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Insert some employees and list them.
    #[test]
    fn smoke() {
        let mut db = DB::new();
        db.update("sales", "amir");
        assert_eq!(vec!["amir".to_string()], db.employees("sales"));
        assert_eq!(
            vec![("sales".into(), vec!["amir".into()])],
            db.all_employees()
        );
    }

    /// Check that the resulting lists are sorted.
    #[test]
    fn lists_are_sorted() {
        let mut db = DB::new();
        db.update("sales", "joseph");
        db.update("sales", "amir");
        db.update("marketing", "amir");
        assert_eq!(
            vec!["amir".to_string(), "joseph".to_string()],
            db.employees("sales")
        );
        assert_eq!(
            vec![
                ("marketing".into(), vec!["amir".into()]),
                ("sales".into(), vec!["amir".into(), "joseph".into()])
            ],
            db.all_employees()
        );
    }
}