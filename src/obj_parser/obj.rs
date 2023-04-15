use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub struct ObjParser {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub tex_coords: Vec<[f32; 2]>,
    pub faces: Vec<Vec<(usize, Option<usize>, Option<usize>)>>,
}

impl ObjParser {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            tex_coords: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let tokens: Vec<&str> = line.trim().split_whitespace().collect();

            if tokens.is_empty() {
                continue;
            }

            match tokens[0] {
                "v" => {
                    self.vertices.push([
                        tokens[1].parse::<f32>().unwrap_or(0.0),
                        tokens[2].parse::<f32>().unwrap_or(0.0),
                        tokens[3].parse::<f32>().unwrap_or(0.0),
                    ]);
                }
                "vn" => {
                    self.normals.push([
                        tokens[1].parse::<f32>().unwrap_or(0.0),
                        tokens[2].parse::<f32>().unwrap_or(0.0),
                        tokens[3].parse::<f32>().unwrap_or(0.0),
                    ]);
                }
                "vt" => {
                    self.tex_coords.push([
                        tokens[1].parse::<f32>().unwrap_or(0.0),
                        tokens[2].parse::<f32>().unwrap_or(0.0),
                    ]);
                }
                "f" => {
                    let mut face = Vec::new();

                    for &face_vertex_data in tokens[1..].iter() {
                        let indices: Vec<Option<usize>> = face_vertex_data
                            .split('/')
                            .map(|s| s.parse::<usize>().ok())
                            .collect();
                        let vertex = indices[0];
                        let tex_coord = indices.get(1).and_then(|&opt| opt);
                        let normal = indices.get(2).and_then(|&opt| opt);

                        if let Some(vertex_index) = vertex {
                            face.push((vertex_index - 1, tex_coord.map(|i| i - 1), normal.map(|i| i - 1)));
                        }
                    }

                    self.faces.push(face);
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn get_vertices(&self) -> &Vec<[f32; 3]> {
        &self.vertices
    }

    pub fn get_normals(&self) -> &Vec<[f32; 3]> {
        &self.normals
    }

    pub fn get_tex_coords(&self) -> &Vec<[f32; 2]> {
        &self.tex_coords
    }

    pub fn get_faces(&self) -> &Vec<Vec<(usize, Option<usize>, Option<usize>)>> {
        &self.faces
    }
}
