pub fn main() {
    slice();
    from_array();
    from_string();
    from_vector();
}

fn slice() {
    let scores_s: &[i32] = &[90, 80, 85, 60, 75];
    println!("FROM SLICE: {:?}", scores_s);
}

fn from_array() {
    let scores = [90, 80, 85, 60, 75];
    let scores_s = &scores[..scores.len()];
    println!("FROM ARRAY: {:?}", scores_s);
}

fn from_string() {
    let name = String::from("Alex Under");
    if name.len() <= 0 {
        return
    }

    let name_chars = &name[..name.len()];
    println!("FROM STRING: {:?}", name_chars);
}

fn from_vector() {
    let scores = vec![90, 80, 85, 60, 75];
    let scores_s: &[i32] = &scores[..scores.len()];
    println!("FROM VECTOR: {:?}", scores_s);
}

