use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Result};
use std::path::Path;

pub struct StlParser {
    pub header: [u8; 80],
    pub triangles: Vec<([f32; 3], [f32; 3], [f32; 3], [f32; 3])>,
    pub is_binary: bool,
}

impl StlParser {
    pub fn new() -> Self {
        Self {
            header: [0; 80],
            triangles: Vec::new(),
            is_binary: false,
        }
    }

    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let mut file = File::open(path)?;

        file.read_exact(&mut self.header)?;
        self.is_binary = Self::is_binary_stl(&self.header);

        if self.is_binary {
            let triangle_count = {
                let mut buf = [0; 4];
                file.read_exact(&mut buf)?;
                u32::from_le_bytes(buf)
            };

            for _ in 0..triangle_count {
                self.parse_binary_triangle(&mut file)?;
            }
        } else {
            self.parse_ascii(&mut file)?;
        }

        Ok(())
    }

    fn is_binary_stl(header: &[u8; 80]) -> bool {
        let solid_prefix = b"solid";
        !header.starts_with(solid_prefix) && !header.starts_with(&solid_prefix.to_ascii_uppercase())
    }

    fn parse_ascii(&mut self, file: &mut File) -> Result<()> {
        let reader = BufReader::new(file);
        let mut lines_iter = reader.lines();
    
        while let Some(line_result) = lines_iter.next() {
            let line = line_result?;
            let tokens: Vec<&str> = line.trim().split_whitespace().collect();
    
            if tokens.len() < 4 {
                continue;
            }
    
            if tokens[0] == "facet" && tokens[1] == "normal" {
                let normal = [
                    match tokens[2].parse::<f32>() {
                        Ok(value) => value,
                        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse normal")),
                    },
                    match tokens[3].parse::<f32>() {
                        Ok(value) => value,
                        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse normal")),
                    },
                    match tokens[4].parse::<f32>() {
                        Ok(value) => value,
                        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse normal")),
                    },
                ];
    
                let mut vertices = [(0.0_f32, 0.0_f32, 0.0_f32); 3];
    
                for i in 0..3 {
                    let line = match lines_iter.next() {
                        Some(line_result) => line_result?,
                        None => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of file")),
                    };
                    let tokens: Vec<&str> = line.trim().split_whitespace().collect();
                    if tokens[0] == "vertex" {
                        vertices[i] = (
                            match tokens[1].parse::<f32>() {
                                Ok(value) => value,
                                Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse vertex")),
                            },
                            match tokens[2].parse::<f32>() {
                                Ok(value) => value,
                                Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse vertex")),
                            },
                            match tokens[3].parse::<f32>() {
                                Ok(value) => value,
                                Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse vertex")),
                            },
                        );
                    }
                }
                self.triangles.push((
                    normal,
                    [vertices[0].0, vertices[0].1, vertices[0].2],
                    [vertices[1].0, vertices[1].1, vertices[1].2],
                    [vertices[2].0, vertices[2].1, vertices[2].2],
                ));
            }
        }
    
        Ok(())
    }
    


    fn parse_binary_triangle(&mut self, reader: &mut impl Read) -> Result<()> {
        let normal = Self::read_vector(reader)?;
        let v1 = Self::read_vector(reader)?;
        let v2 = Self::read_vector(reader)?;
        let v3 = Self::read_vector(reader)?;

        let mut buf = [0; 2];
        reader.read_exact(&mut buf)?;

        self.triangles.push((normal, v1, v2, v3));

       
        Ok(())
    }

    fn read_vector(reader: &mut impl Read) -> Result<[f32; 3]> {
        let mut buf = [0; 4 * 3];
        reader.read_exact(&mut buf)?;
        let vec = [
            f32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
            f32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]),
            f32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]),
        ];
        Ok(vec)
    }

    pub fn get_header(&self) -> &[u8; 80] {
        &self.header
    }

    pub fn get_triangles(&self) -> &Vec<([f32; 3], [f32; 3], [f32; 3], [f32; 3])> {
        &self.triangles
    }
}
