trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn draw(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

#[derive(Debug)]
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw Button {:?}", self)
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("draw Button {:?}", self)
    }
}

fn main() {
    let s = Screen {
        components: vec![
            Box::new(Button {
                width: 3,
                height: 4,
                label: String::from("test"),
            }),
            Box::new(SelectBox {
                width: 3,
                height: 4,
                options: vec![String::from("Ok")],
            }),
        ],
    };
    s.draw();
}
