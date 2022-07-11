use std::f32::consts::PI;
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait LightTime {
    fn time(&self) -> u8;
}

impl LightTime for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => return 10,
            TrafficLight::Green => return 15,
            TrafficLight::Yellow => return 20,
        }
    }
}

fn u32_sum(u32_slice: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;

    for item in u32_slice.iter() {
        sum += item;
        if sum == u32::MAX {
            return None;
        }
    }
    Some(sum)
}

struct Circle {
    radis: f32,
}
impl Caluarena for Circle {
    fn arena(&self) -> f32 {
        PI * self.radis * self.radis
    }
}
struct Ranc {
    width: f32,
    height: f32,
}
impl Caluarena for Ranc {
    fn arena(&self) -> f32 {
        self.height * self.width
    }
}

struct Tri {
    width: f32,
    height: f32,
}

impl Caluarena for Tri {
    fn arena(&self) -> f32 {
        self.height * self.width * 1.0 / 2.0
    }
}

trait Caluarena {
    fn arena(&self) -> f32;
}

fn carena<T: Caluarena>(element: &T) -> f32 {
    element.arena()
}

fn main() {
    println!("=================Q1=========================");
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    println!("  red   light time:{}", red.time());
    println!(" green  light time:{}", green.time());
    println!(" yellow light time:{}", yellow.time());

    println!("=================Q2=========================");
    let number_slice = [1, 2, 3, 4, 5];
    let reuslt = u32_sum(&number_slice);
    println!("{:?}", reuslt);

    println!("=================Q3=========================");
    let a = Circle { radis: 3.0 };
    let b = Ranc {
        width: 2.0,
        height: 3.0,
    };
    let c = Tri {
        width: 2.0,
        height: 3.0,
    };
    println!("半径为{}圆形面积为{}", a.radis, a.arena());
    println!(
        "长为{} 宽为{}的长方形或正方形面积为 {}",
        b.height,
        b.width,
        b.arena()
    );
    println!(
        "底为{}，高为{}的三角形面积为{}",
        c.width,
        c.height,
        c.arena()
    );
    println!("{:?}", carena(&a));
    println!("{:?}", carena(&b));
    println!("{:?}", carena(&c));
}
