use godot::engine::INode;
use godot::engine::Node;
use godot::prelude::*;

use fountain_parser_rs::fountain_enums;
use fountain_parser_rs::static_fountain_parser;

#[derive(GodotClass)]
#[class(base = Node)]
struct FountainParser {
    base: Base<Node>,
}
#[godot_api]
impl INode for FountainParser {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl FountainParser {
    #[func]
    fn get_parsed_lines_from_raw_string(raw_string: GString) -> Dictionary {
        let parsed_lines =
            static_fountain_parser::get_parsed_lines_from_raw_string(raw_string.to_string());
        let mut ln_dict = Dictionary::new();
        for fnln in parsed_lines {
            let ln_str = fnln.string.to_godot();
            let ln_type = fnln.fn_type.to_string().to_godot();
            let ln_pos = fnln.position;
            let ln_isforced: bool = fnln.is_forced;
            let mut lines_arr = Array::new();
            lines_arr.push(ln_str);
            lines_arr.push(ln_type);
            lines_arr.push(ln_isforced.to_string().into_godot());
            ln_dict.insert(ln_pos, lines_arr);
        }

        ln_dict
    }
    // TODO: Fix problems with fountain-parser-rs
    // - Expose a way to get a Dictionary or HashMap of FNLineTypes
    // -- KEY: FNLineType NUMBER
    // -- VAL: FNLineType type name as STRING
    #[func]
    fn print_all_fnline_types() {
        let lt_vec = fountain_enums::FNLineType::vec_of_line_types();
        for lt in lt_vec {
            godot_print!("{}\t{}", lt.clone() as i32, &lt.to_string())
        }
    }
}

// FNLine struct for godot

#[derive(GodotClass)]
#[class(base=Node)]
struct FNLineGD {
    #[var]
    string: GString,
    #[var]
    pos: i32,
    #[var]
    fn_type: GString,
    #[var]
    is_type_forced: GString,
    base: Base<Node>,
}

#[godot_api]
impl INode for FNLineGD {
    fn init(base: Base<Node>) -> Self {
        Self {
            string: "".to_godot(),
            pos: 0.to_godot(),
            fn_type: "Unparsed".to_godot(),
            is_type_forced: "false".to_string().to_godot(),
            base: base,
        }
    }
}

// Extension tag
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
