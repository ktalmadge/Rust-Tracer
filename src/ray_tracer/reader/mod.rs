#![cfg_attr(feature = "cargo-clippy", allow(needless_range_loop))]

extern crate cgmath;

use self::cgmath::*;

use std::io::{self, BufReader, ErrorKind};
use std::io::prelude::*;
use std::fs::File;

use super::object::triangle::Triangle;
use super::object::sphere::Sphere;
use super::object::material::Material;
use super::object::Shape;

pub struct Reader {
    vertices: Vec<Vector3<f64>>,
    normals: Vec<Vector3<f64>>,
    pub shapes: Vec<Shape>,
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
            shapes: Vec::new(),
        }
    }

    fn eval(
        &mut self,
        statement: &str,
        args: Vec<&str>,
        material: Material,
    ) -> Result<(), ::std::io::Error> {
        match statement {
            "v" => {
                self.vertices.push(Vector3::new(
                    parse_float(args[0])?,
                    parse_float(args[1])?,
                    parse_float(args[2])?,
                ));

                Ok(())
            }
            "vn" => {
                self.normals.push(Vector3::new(
                    parse_float(args[0])?,
                    parse_float(args[1])?,
                    parse_float(args[2])?,
                ));

                Ok(())
            }
            "f" => {
                if args.len() > 4 {
                    Err(::std::io::Error::new(
                        ErrorKind::Other,
                        format!(
                            "Face has too many vertices: {} {:?}",
                            statement,
                            args
                        ),
                    ))
                } else {
                    self.shapes.push(Shape::Triangle(Triangle::new(
                        self.vertices[parse_face_indices(args[0])?],
                        self.vertices[parse_face_indices(args[1])?],
                        self.vertices[parse_face_indices(args[2])?],
                        material,
                    )));

                    if args.len() == 4 {
                        // Object is a rectangle - Make two triangles
                        self.shapes.push(Shape::Triangle(Triangle::new(
                            self.vertices[parse_face_indices(args[2])?],
                            self.vertices[parse_face_indices(args[3])?],
                            self.vertices[parse_face_indices(args[0])?],
                            material,
                        )));
                    }

                    Ok(())
                }
            }
            "sphere" => {
                let sphere_origin: Vector3<f64> = Vector3::new(
                    parse_float(args[0])?,
                    parse_float(args[1])?,
                    parse_float(args[2])?,
                );

                self.shapes.push(Shape::Sphere(
                    Sphere::new(sphere_origin, parse_float(args[3])?, material),
                ));

                Ok(())
            }
            _ => Ok(()),
            /*  Error handling for unknown statements

                _ => Err(::std::io::Error::new(
                    ErrorKind::Other,
                    format!("Unknown statement specification: {}", statement),
                )),
            */
        }
    }

    fn parse(
        &mut self,
        file_contents: BufReader<File>,
        material: Material,
    ) -> Result<(), ::std::io::Error> {
        for line in file_contents.lines() {
            let line = line?;
            let mut tokens = line.split_whitespace();

            if let Some(statement) = tokens.next() {
                let mut args: Vec<&str> = Vec::new();

                for t in tokens {
                    args.push(t);
                }

                match self.eval(statement, args, material) {
                    Err(error) => panic!("{}", error),
                    Ok(_) => (),
                }
            }
        }

        Ok(())
    }

    pub fn read_file(&mut self, filename: &str, material: Material) -> Result<(), io::Error> {
        let file_contents = BufReader::new(File::open(filename)?);
        self.parse(file_contents, material)
    }
}
