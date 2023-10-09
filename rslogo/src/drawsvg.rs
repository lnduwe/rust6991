use unsvg::{Image, COLORS};

fn test() -> Result<(), String> {
    let mut img: Image = Image::new(200, 200);
    let second_point = img.draw_simple_line(10.0, 10.0, 120, 100.0, COLORS[1]).expect("msg");
    let third_point = img.draw_simple_line(second_point.0, second_point.1, 240, 100.0, COLORS[2]).expect("jfiejwof");
    let _ = img.draw_simple_line(third_point.0, third_point.1, 0, 100.0, COLORS[3]).expect("fejfie");

    img.save_png("path_to.png").expect("error");

    Ok(())
}

#[test]
fn test_draw_simple_line() {
    assert!(test().is_ok());
}
   
