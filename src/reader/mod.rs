extern crate cgmath;

use self::cgmath::*;

use std::io::{self, BufReader, ErrorKind};
use std::io::prelude::*;
use std::fs::File;

pub struct Reader {
    vertices: Vec<Vector3<f64>>,
    normals: Vec<Vector3<f64>>,
    pub objects: Vec<Box<::object::Object>>,
}

enum FaceIndex {
    V(usize),
    VN(usize, usize),
}

fn parse_float(f: &str) -> Result<f64, ::std::io::Error> {
    match f.parse() {
        Ok(n) => Ok(n),
        Err(msg) => Err(::std::io::Error::new(
            ErrorKind::Other,
            format!("Error parsing float: {} : {}", f, msg),
        )),
    }
}

fn parse_index(i: &str) -> Result<usize, ::std::io::Error> {
    match i.parse() {
        Ok(n) => {
            let index: usize = n;
            Ok(n - 1)
        }
        Err(msg) => Err(::std::io::Error::new(
            ErrorKind::Other,
            format!("Error parsing integer: {} : {}", i, msg),
        )),
    }
}

fn parse_face_indices(f: &str) -> Result<usize, ::std::io::Error> {
    if let Some(index) = f.split("//").next() {
        // TODO: Normals
        parse_index(index)
    } else {
        Err(::std::io::Error::new(
            ErrorKind::Other,
            format!("Unknown face specification: {}", f),
        ))
    }
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            vertices: Vec::new(),
            normals: Vec::new(),
            objects: Vec::new(),
        }
    }

    fn eval(&mut self, statement: &str, args: Vec<&str>) -> Result<(), ::std::io::Error> {
        match statement {
            "v" => {
                self.vertices.push(Vector3::new(
                    parse_float(args.get(0).unwrap())?,
                    parse_float(args.get(1).unwrap())?,
                    parse_float(args.get(2).unwrap())?,
                ))
            }
            "vn" => {
                self.normals.push(Vector3::new(
                    parse_float(args.get(0).unwrap())?,
                    parse_float(args.get(1).unwrap())?,
                    parse_float(args.get(2).unwrap())?,
                ))
            }
            "f" => {
                let v1: usize = parse_face_indices(args.get(0).unwrap())?;
                let v2: usize = parse_face_indices(args.get(1).unwrap())?;
                let v3: usize = parse_face_indices(args.get(2).unwrap())?;

                self.objects.push(
                    Box::new(::object::triangle::Triangle::new(
                        self.vertices.get(v1).unwrap().clone(),
                        self.vertices.get(v2).unwrap().clone(),
                        self.vertices.get(v3).unwrap().clone(),
                    )),
                )
            }
            _ => (),
        }

        Ok(())
    }

    fn parse(&mut self, file_contents: BufReader<File>) -> Result<(), ::std::io::Error> {
        // This is a buffer of the "arguments" for each line, it uses raw pointers
        // in order to allow it to be re-used across iterations.
        for line in file_contents.lines() {
            let line = line?;
            let mut tokens = line.split_whitespace();

            if let Some(statement) = tokens.next() {
                let mut args: Vec<&str> = Vec::new();

                for t in tokens {
                    args.push(t);
                }

                self.eval(statement, args)?
            }
        }

        Ok(())
    }

    pub fn read_file(&mut self, filename: &str) -> Result<(), io::Error> {
        let triangles: Vec<Box<::object::Object>> = Vec::new();

        let file_contents = BufReader::new(File::open(filename)?);

        self.parse(file_contents)
    }
}
