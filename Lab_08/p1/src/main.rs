use std::{fs, collections::HashMap};

fn p1(path: String) {
    let mut cuvinte_map:HashMap<String, i32> = HashMap :: new();
    let file = fs::read_to_string(path).expect("Path Gresit");
    let mut max_length = 0;
    for key in file.split([' ','.','\r', '\n','!','?','\t']) {
        if key == "" {
            continue;
        }
        let keyy= key.to_ascii_lowercase();
        if max_length < keyy.len() {
            max_length = keyy.len();
        }
        cuvinte_map.entry(keyy).and_modify(|x| *x +=1).or_insert(1);   
    }
    let mut sort_vec: Vec<_> = cuvinte_map.iter().collect();
    sort_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (key,value) in sort_vec {
        print!("{key}");
        let mut i =0;
        while i<(max_length-key.len()) {
            print!(" ");
            i+=1;
        }
        print!(" =>{value}\n");
    }
}

fn main() {
    p1("imput.txt".to_string());
}
