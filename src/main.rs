///TODO ::  Math Equation Parser
///TODO :: Parm Getter
///TODO :: Sqllite DB
///TODO :: Daemon service
// Date - 19/01/24 ToxicTrigger

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
        return EquationMap{ equations : BTreeMap::new() };
    }

    /// - return -> T = Success , F = Failed
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
    
    if maps.insert("Add".to_string(), Equation{value:"x+y".to_string()})
    {
        let mut equ = String::new();
        println!("Typing Equation Name!");
        std::io::stdin().read_line(&mut equ).unwrap();

        // Delete \n
        if let Some('\n') = equ.chars().next_back()
        {
            equ.pop();
        }

        match maps.equations.get(&equ)
        {
            Some(s) => println!("{:?}" , s),
            None => println!("{} is null",equ),
        }
    }
}
