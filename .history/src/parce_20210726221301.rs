use std::fmt;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use crate::graph::Vector3D;

pub struct Object {
    pub filename: String,
    pub facets: Vec<[i32; 3]>,
    pub vert: Vec<Vector3D>
}


impl Object {
    pub fn new(filename: &str) -> Object {
        Object {
            filename: filename.to_string(),
            facets:Vec::new(),
            vert: Vec::new(),
        }
    }

    pub fn read(&mut self){
        let file = read_file(&self.filename);
        let mut v_line: i32 = 0;
        let mut f_line: i32 = 0;
        for line in file.lines() {
            let line = line.unwrap();
            if line.starts_with("v ") {
                v_line += 1;
                let words: Vec<&str> = line.split_whitespace().collect();
                self.vert.push(Vector3D::new(
                    words[1].parse().unwrap(),
                    words[2].parse().unwrap(),
                    words[3].parse().unwrap(),
                ));
                // println!("{}", self.vert.last().unwrap());
            }
            else if line.starts_with("f ") {
                f_line += 1;
                let mut face: [i32; 3] = [-1, -1, -1];
                let words: Vec<&str> = line.split_whitespace().collect();
                for i in 0..3 {
                    face[i] = words[i+1].split("/").next().unwrap().parse().unwrap();
                    face[i] -= 1;
                }
                self.facets.push(face);
            }
        }
        println!("f: {}  v: {}", f_line, v_line)
    }
}

fn read_file<P>(filename: P) -> std::io::BufReader<std::fs::File> where P:  AsRef<Path> { 
        let file = io::BufReader::new(File::open(filename).unwrap());
        file
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.filename)
    }
}


// if line.starts_with("f ") {
//     f_line += 1;
//     let mut face: [i32; 3] = [-1, -1, -1];
//     let words: Vec<&str> = line.split_whitespace().collect();
//     for i in 0..3 {
//         face[i] = words[i+1].split("/").next().unwrap().parse().unwrap();
//         face[i] -= 1;
//     }
//     self.facets.push(face);
// }



// if line.starts_with("f ") {
//     f_line += 1;
//     let words: Vec<&str> = line.split_whitespace().collect();
//     for word in 0..3 {
//         if word != 0 {
//             let word: Vec<&str> = words[word].split("/").collect();
//             // println!("{:?}", word);
//             let x: [i32; 3] = [
//                 word[0].parse::<i32>().unwrap() - 1,
//                 word[1].parse::<i32>().unwrap() - 1,
//                 word[2].parse::<i32>().unwrap() - 1,
//             ];
//             self.facets.push(x);
//         }
//     }
// }