use serde::Deserialize;
use std::collections::VecDeque;
use std::io;

#[derive(Debug, Deserialize)]
enum Instruction {
    Set(i32),
    Left,
    Right,
    Reset,
}

#[derive(Debug)]
struct Light {
    // TODO: change me!
    left: Option<Box<Light>>,
    right: Option<Box<Light>>,
    brightness: i32,
}

fn get_instructions_from_stdin() -> VecDeque<Instruction> {
    let mut instructions = String::new();
    io::stdin().read_line(&mut instructions).unwrap();
    ron::from_str(&instructions).unwrap()
}

fn main() {
    let instructions = get_instructions_from_stdin();
    let light = Light { left: None, right: None, brightness: 0};
    // println!("{instructions:?}");
    // println!("{light:?}");
    // TODO: your implementation here

    let mut root = Box::new(light);

  {  
    let mut node = &mut root;

    for ins in instructions.into_iter() {
        match ins {
            Instruction::Set(brightness) => {
                node.brightness = brightness;
            }
            Instruction::Left => {
                node.left = Some(Box::new(Light {
                    left: None,
                    right: None,
                    brightness: 0,
                }));
                node = node.left.as_mut().unwrap();
            }
            Instruction::Right => {
                node.right = Some(Box::new(Light {
                    left: None,
                    right: None,
                    brightness: 0,
                }));
                node = node.right.as_mut().unwrap();
            }
            Instruction::Reset => {
                node = &mut root;
            }
        }
}

        let mut data = (0, 0);
        traversal(&Some(&mut root), &mut data);

        println!("{}", if data.1 == 0 { "0".to_string() } else { (data.0 / data.1).to_string() });


    }

    // instructions.iter().for_each(|ins|{
    //   match ins {
    //     Instruction::Set(brightness) => {
    //       node.brightness = *brightness;
    //     },
    //     Instruction::Left =>{
    //       node.left =Some(Box::new(Light { left: None, right: None, brightness: 0}));
    //       node = node.left.as_mut().unwrap();
    //   },
    //     Instruction::Right => todo!(),
    //     Instruction::Reset => todo!(),
    // }
    // });


}

fn traversal(node: &Option<&Box<Light>>, num:&mut(i32,  i32)){
    if let Some(nd) = node {
        traversal(&nd.left.as_ref(), num);
        traversal(&nd.right.as_ref(),num);
        num.0 += nd.brightness;
        num.1 += 1;
    }
}
