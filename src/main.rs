use std::collections::BTreeMap;

/// # EquationMap
/// ---
/// Save Equations using BTreeMap<String, Equation>
struct EquationMap
{
    equations : BTreeMap<String, Equation>
}

impl EquationMap
{
    pub fn new() -> EquationMap
    {
        return EquationMap{equations : BTreeMap::new() };
    }

    pub fn insert(&mut self, equation_name : String, equation : Equation)->bool
    {
        let op = self.equations.insert(equation_name, equation);
        match op
        {
            None => return true,
            _ => return false,
        }
    }
}

/// # Readable Equation
/// ---
///  value : String -> Readable Equations Text
#[derive(Debug)]
struct Equation
{
    value : String
}


fn main()
{
    let mut maps = EquationMap::new();
    maps.insert("Test001".to_string(), Equation{value:"x+y".to_string()});
    println!("value = {:?}" , maps.equations.get("Test001").unwrap());
}
