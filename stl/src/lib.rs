const EOF_KEYWORD_OF_ASCII_STL: &str = "endsolid";
const KEYWORD_OF_VERTEX: &str = "vertex";
const KEYWORD_START_STL_NAME: &str = "solid ";
const KEYWORD_OF_FACETS_NORMAL_FACET: &str = "facet";
const KEYWORD_OF_FACETS_NORMAL_NORMAL: &str = "normal";

#[cfg(windows)]
const EOL: &'static str = "\r\n";
#[cfg(not(windows))]
const EOL: &'static str = "\n";

pub struct ParsedSTL {
    name: String,
    is_binary: bool,
    is_ascii: bool,
    is_valid: bool,
    normal_vectors: Vec<NormalVector>,
    vertex_coordinates: Vec<VertexCoordinates>,
}

struct NormalVector {
    x: String,
    y: String,
    z: String,
}

struct VertexCoordinates {
    x: String,
    y: String,
    z: String,
}

fn get_stl_name(line: &str) -> String {
    if !line.contains(KEYWORD_START_STL_NAME) {
        return "".to_string();
    }
    let stl_name = line.replace(KEYWORD_START_STL_NAME, "");

    return stl_name;
}

pub fn read_ascii_stl(texts: String) -> ParsedSTL {
    let lines: Vec<&str> = texts.split(EOL).collect();
    let name = get_stl_name(lines[0]);
    let end_line_index: usize = get_keyword_line(&lines, EOF_KEYWORD_OF_ASCII_STL);
    let normal_vectors: Vec<NormalVector> = get_normal_vectors(&lines, end_line_index);
    let vertex_coordinates: Vec<VertexCoordinates> = get_vertex_coordinates(&lines, end_line_index);

    let parsed_stl: ParsedSTL = ParsedSTL {
        name: name,
        is_binary: false,
        is_ascii: true,
        is_valid: true,
        normal_vectors: normal_vectors,
        vertex_coordinates: vertex_coordinates,
    };

    return parsed_stl;
}

fn get_keyword_line(texts: &Vec<&str>, key_word: &str) -> usize {
    let mut index: usize = 0;
    for ith in 0..texts.len() {
        if texts[ith] == key_word {
            index = ith;
        }
    }
    return index;
}

fn get_normal_vectors(texts: &Vec<&str>, num_eof: usize) -> Vec<NormalVector> {
    let mut normal_vectors: Vec<NormalVector> = Vec::new();
    let mut keywords_to_remove = Vec::new();
    keywords_to_remove.push(KEYWORD_OF_FACETS_NORMAL_FACET);
    keywords_to_remove.push(KEYWORD_OF_FACETS_NORMAL_NORMAL);
    for ith in 1..num_eof {
        if
            texts[ith].contains(KEYWORD_OF_FACETS_NORMAL_FACET) &&
            texts[ith].contains(KEYWORD_OF_FACETS_NORMAL_NORMAL)
        {
            let mut line = remove_keywords(texts[ith], &keywords_to_remove);
            normal_vectors.push(NormalVector {
                x: line[0].as_mut().to_string(),
                y: line[1].as_mut().to_string(),
                z: line[2].as_mut().to_string(),
            });
        }
    }
    return normal_vectors;
}

fn remove_keywords(text: &str, keywords: &Vec<&str>) -> Vec<String> {
    let mut parsed_text: Vec<String> = text
        .split(" ")
        .map(|s| s.to_string())
        .collect();
    let mut keywords_removed_text: Vec<String> = Vec::new();
    for keyword in keywords {
        for ith in 0..parsed_text.len() - 1 {
            if &parsed_text[ith] == keyword {
                parsed_text.remove(ith);
            }
        }
    }
    for word in parsed_text {
        if word != "" {
            keywords_removed_text.push(word);
        }
    }
    println!("keywords_removed_text: {:?}", keywords_removed_text);
    return keywords_removed_text;
}

fn get_vertex_coordinates(texts: &Vec<&str>, num_eof: usize) -> Vec<VertexCoordinates> {
    let mut vertex_coordinates: Vec<VertexCoordinates> = Vec::new();
    let mut keywords_to_remove = Vec::new();
    keywords_to_remove.push(KEYWORD_OF_VERTEX);
    for ith in 1..num_eof {
        if texts[ith].contains(KEYWORD_OF_VERTEX) {
            let mut line = remove_keywords(&texts[ith], &keywords_to_remove);
            vertex_coordinates.push(VertexCoordinates {
                x: line[0].as_mut().to_string(),
                y: line[1].as_mut().to_string(),
                z: line[2].as_mut().to_string(),
            });
        }
    }
    return vertex_coordinates;
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}