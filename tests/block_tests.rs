// Copyright (c) IxMilia.  All Rights Reserved.  Licensed under the Apache License, Version 2.0.  See License.txt in the project root for license information.

extern crate dxf;
use self::dxf::*;
use self::dxf::entities::*;

mod test_helpers;
use test_helpers::helpers::*;

fn read_blocks_section(content: Vec<&str>) -> Drawing {
    let mut file = String::new();
    file.push_str(vec![
        "0", "SECTION",
        "2", "BLOCKS",
    ].join("\n").as_str());
    file.push('\n');
    for line in content {
        file.push_str(line);
        file.push('\n');
    }
    file.push_str(vec![
        "0", "ENDSEC",
        "0", "EOF",
    ].join("\n").as_str());
    parse_drawing(file.as_str())
}

fn read_single_block(content: Vec<&str>) -> Block {
    let mut full_block = vec![];
    full_block.push("0");
    full_block.push("BLOCK");
    for line in content {
        full_block.push(line);
    }
    full_block.push("0");
    full_block.push("ENDBLK");
    let drawing = read_blocks_section(full_block);
    assert_eq!(1, drawing.blocks.len());
    drawing.blocks[0].to_owned()
}

#[test]
fn read_empty_blocks_section() {
    let drawing = read_blocks_section(vec![]);
    assert_eq!(0, drawing.blocks.len());
}

#[test]
fn read_empty_block() {
    let _block = read_single_block(vec![]);
}

#[test]
fn read_block_specific_values() {
    let block = read_single_block(vec![
        "2", "block-name",
        "10", "1.1",
        "20", "2.2",
        "30", "3.3",
    ]);
    assert_eq!("block-name", block.name);
    assert_eq!(0, block.entities.len());
    assert_eq!(Point::new(1.1, 2.2, 3.3), block.base_point);
}

#[test]
fn read_with_end_block_values() {
    // these values should be ignored
    let drawing = read_blocks_section(vec![
        "0", "BLOCK",
        "0", "ENDBLK",
        "5", "1", // handle
        "330", "2", // owner handle
        "100", "AcDbEntity",
        "8", "layer-name",
        "100", "AcDbBlockEnd",
    ]);
    assert_eq!(1, drawing.blocks.len());
}

#[test]
fn read_multiple_blocks() {
    let drawing = read_blocks_section(vec![
        "0", "BLOCK",
        "0", "ENDBLK",
        "0", "BLOCK",
        "0", "ENDBLK",
    ]);
    assert_eq!(2, drawing.blocks.len())
}

#[test]
fn read_block_with_single_entity() {
    let block = read_single_block(vec![
        "0", "LINE",
        "10", "1.1",
        "20", "2.2",
        "30", "3.3",
        "11", "4.4",
        "21", "5.5",
        "31", "6.6",
    ]);
    assert_eq!(1, block.entities.len());
    match block.entities[0].specific {
        EntityType::Line(ref line) => {
            assert_eq!(Point::new(1.1, 2.2, 3.3), line.p1);
            assert_eq!(Point::new(4.4, 5.5, 6.6), line.p2);
        },
        _ => panic!("expected a line"),
    }
}

#[test]
fn read_block_with_multiple_entities() {
    let block = read_single_block(vec![
        "0", "LINE",
        "0", "CIRCLE",
    ]);
    assert_eq!(2, block.entities.len());
    match block.entities[0].specific {
        EntityType::Line(_) => (),
        _ => panic!("expected a line"),
    }
    match block.entities[1].specific {
        EntityType::Circle(_) => (),
        _ => panic!("expected a circle"),
    }
}

#[test]
fn read_block_with_unsupported_entity_first() {
    let block = read_single_block(vec![
        "0", "UNSUPPORTED_ENTITY",
        "0", "LINE",
    ]);
    assert_eq!(1, block.entities.len());
    match block.entities[0].specific {
        EntityType::Line(_) => (),
        _ => panic!("expected a line"),
    }
}

#[test]
fn read_block_with_unsupported_entity_last() {
    let block = read_single_block(vec![
        "0", "LINE",
        "0", "UNSUPPORTED_ENTITY",
    ]);
    assert_eq!(1, block.entities.len());
    match block.entities[0].specific {
        EntityType::Line(_) => (),
        _ => panic!("expected a line"),
    }
}

#[test]
fn read_block_with_unsupported_entity_in_the_middle() {
    let block = read_single_block(vec![
        "0", "LINE",
        "0", "UNSUPPORTED_ENTITY",
        "0", "CIRCLE",
    ]);
    assert_eq!(2, block.entities.len());
    match block.entities[0].specific {
        EntityType::Line(_) => (),
        _ => panic!("expected a line"),
    }
    match block.entities[1].specific {
        EntityType::Circle(_) => (),
        _ => panic!("expected a circle"),
    }
}

#[test]
fn dont_write_blocks_section_if_no_blocks() {
    let drawing = Drawing::new();
    let contents = to_test_string(&drawing);
    assert!(!contents.contains("BLOCKS"));
}

#[test]
fn round_trip_blocks() {
    let mut drawing = Drawing::new();
    let mut b1 = Block::default();
    b1.entities.push(
        Entity {
            common: EntityCommon::new(),
            specific: EntityType::Line(Default::default()),
        }
    );
    drawing.blocks.push(b1);
    let mut b2 = Block::default();
    b2.entities.push(
        Entity {
            common: EntityCommon::new(),
            specific: EntityType::Circle(Default::default()),
        }
    );
    drawing.blocks.push(b2);
    let written = to_test_string(&drawing);
    let reparsed = unwrap_drawing(Drawing::load(written.as_bytes()));
    assert_eq!(2, reparsed.blocks.len());
    assert_eq!(1, reparsed.blocks[0].entities.len());
    match reparsed.blocks[0].entities[0].specific {
        EntityType::Line(_) => (),
        _ => panic!("expected a line"),
    }
    assert_eq!(1, reparsed.blocks[1].entities.len());
    match reparsed.blocks[1].entities[0].specific {
        EntityType::Circle(_) => (),
        _ => panic!("expected a circle"),
    }
}
