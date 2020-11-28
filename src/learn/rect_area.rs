pub fn area_primitive() {
    let width1 = 30;
    let height1 = 30;

    let area = find_area_primitive(width1, height1);

    println!("The area of a {}x{} rect is {}", width1, height1, area);
}

fn find_area_primitive(width: i32, height: i32) -> i32 {
    width * height
}

pub fn area_tuple() {
    let rect1 = (30, 30);
    let area = find_area_tuple(rect1);
    println!("The area of a {}x{} rect is {}", rect1.0, rect1.1, area);
}

fn find_area_tuple(rect: (i32, i32)) -> i32 {
    rect.0 * rect.1
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        // width and height of other rectange must be smaller in order to fit
        other_rectangle.width < self.width && other_rectangle.height < self.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn area_struct() {
    let rect1 = Rectangle {
        width: 21,
        height: 21,
    };
    let rect2 = Rectangle {
        width: 400,
        height: 12,
    };
    let rect3 = Rectangle {
        width: 5,
        height: 5,
    };
    // let area = find_area_struct(&rect1);
    println!(
        "The area of a {}x{} rect is {}",
        rect1.width,
        rect1.height,
        rect1.area()
    );
    println!(
        "{:#?} can hold {:#?}? {}",
        rect1,
        rect2,
        rect1.can_hold(&rect2)
    );
    println!(
        "{:#?} can hold {:#?}? {}",
        rect1,
        rect3,
        rect1.can_hold(&rect3)
    );
    let square = Rectangle::square(50);
    println!("{:#?}", square);
}

fn find_area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
