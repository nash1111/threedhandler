pub mod stl_parser;
pub mod obj_parser;

pub fn read_stl(path: &str) -> Result<(), std::io::Error> {
    let mut stl = stl_parser::stl::StlParser::new();
    stl.parse_file(path)?;
    Ok(())
}

pub fn read_obj(path: &str) -> Result<(), std::io::Error> {
    let mut obj = obj_parser::obj::ObjParser::new();
    obj.parse_file(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_binary_stl_test() {
        let result = read_stl("samples/ball.stl");
        assert!(result.is_ok());
    }

    #[test]
    fn read_ascii_stl_test() {
        let result = read_stl("samples/ballascii.stl");
        assert!(result.is_ok());
    }

    #[test]
    fn read_obj_test() {
        let result = read_obj("samples/ball.obj");
        assert!(result.is_ok());
    }
}
