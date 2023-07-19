
// ugly hack just meant to make it a bit easier to read serialization outputs

pub fn fmt(text:&str) -> String {

    let mut depth = 1;
    let mut is_inside_strlit = false;
    let mut s = String::new();
    let mut previous_char : Option<char> = None;
    let mut previous_non_space_char : Option<char> = None;
    for x in text.chars() {
        match x {
            '\r'|'\n'|'\t' => {},
            '"' => {
                is_inside_strlit = !is_inside_strlit;
                s.push(x)
            },            
            '[' if !is_inside_strlit => {
                s.push('\n');
                s.push_str("\t".repeat(depth).as_str());
                depth += 1;
                s.push(x);  
                
            },
            '(' if !is_inside_strlit && let Some('[') = previous_non_space_char => {
                s.push(x);
                depth += 1;
            }
            '(' if !is_inside_strlit => {
                s.push('\n');
                s.push_str("\t".repeat(depth).as_str());
                s.push(x);
                depth += 1;
            },
            x if let Some(')') = previous_non_space_char && x != ')' && x != '(' && x != ']' && x != '[' && x != ' ' && x != '\t' => {
             
                s.push('\n');
                s.push_str("\t".repeat(depth).as_str());
                s.push(x);
            },
            ')' if !is_inside_strlit && let Some(')') = previous_non_space_char => {
                depth -= 1;
                s.push('\n');
                s.push_str("\t".repeat(depth).as_str());
                s.push(x);
                
            },
            ')' if !is_inside_strlit => {
                depth -= 1;
                s.push(x)
            },
            ' ' if !is_inside_strlit && let Some(' ') = previous_char => { 

            },
            ']' if !is_inside_strlit => {
                depth -= 1;
                s.push(x);
            }
            _ => s.push(x)
            
        }
        if x != ' ' && x != '\n' && x != '\t' {
            previous_non_space_char = Some(x);
        }
        previous_char = Some(x);
    }

    s.replace('\t', "    ")
    
}

#[test]
pub fn test_fmt() {
    
    let s = "When [Case (Deposit (Role \"role\") (Role \"role\") (Token \"\" \"\") (Constant 1000000) ) (Pay (Role \"role3\") (Party (Role \"role2\")) (Token \"\" \"\") (Constant 5000000) (When [Case (Deposit (Role \"role\") (Role \"role\") (Token \"\" \"\") (Constant 1000000) ) (Pay (Role \"role3\") (Party (Role \"role2\")) (Token \"\" \"\") (Constant 5000000) Close  )] 1668781080000 Close) )] 1668781080000 Close";
    format!("{}",fmt(s));
}