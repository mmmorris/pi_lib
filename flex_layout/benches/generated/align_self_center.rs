fn print(count: &mut usize, id: usize, layout: &layout::tree::LayoutR) {
    *count += 1;
    println!("result: {:?} {:?} {:?}", *count, id, layout);
}
pub fn compute() {
    let mut layout_tree = layout::tree::LayoutTree::default();
    layout_tree.insert(
        1,
        0,
        0,
        layout::idtree::InsertType::Back,
        layout::style::Style {
            position_type: layout::style::PositionType::Absolute,
            size: layout::geometry::Size {
                width: layout::style::Dimension::Points(1920.0),
                height: layout::style::Dimension::Points(1024.0),
            },
            ..Default::default()
        },
    );
    layout_tree.insert(
        2,
        1,
        0,
        layout::idtree::InsertType::Back,
        layout::style::Style {
            size: layout::geometry::Size {
                width: layout::style::Dimension::Points(100f32),
                height: layout::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
    );
    layout_tree.insert(
        3,
        2,
        0,
        layout::idtree::InsertType::Back,
        layout::style::Style {
            align_self: layout::style::AlignSelf::Center,
            size: layout::geometry::Size {
                width: layout::style::Dimension::Points(10f32),
                height: layout::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
    );
    layout_tree.compute(print, &mut 0);
}