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
                    parse_float(args[0])?,
                    parse_float(args[1])?,
                    parse_float(args[2])?,
                ))
            }
            "vn" => {
                self.normals.push(Vector3::new(
                    parse_float(args[0])?,
                    parse_float(args[1])?,
                    parse_float(args[2])?,
                ))
            }
            "f" => {
                // TODO: If there are more than 3 vertices, make triangles out of them
                let v1: usize = parse_face_indices(args[0])?;
                let v2: usize = parse_face_indices(args[1])?;
                let v3: usize = parse_face_indices(args[2])?;

                self.objects.push(
                    Box::new(::object::triangle::Triangle::new(
                        self.vertices[v1],
                        self.vertices[v2],
                        self.vertices[v3],
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
