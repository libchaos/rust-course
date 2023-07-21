struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

struct Duck {
    name: String,
}
impl Animal for Dog {
    fn say(&self) {
        println!("{} say woof", self.name);
    }
}

impl Animal for Cat {
    fn say(&self) {
        println!("{} say meow", self.name);
    }
}


impl Animal for Duck {
    fn say(&self) {
        println!("{} say quack", self.name);
    }
}

enum AnimalEnum {
    Dog(Dog),
    Cat(Cat),
    Duck(Duck),
}

impl AnimalEnum {
    fn say(&self) {
        match self {
            AnimalEnum::Dog(dog) => dog.say(),
            AnimalEnum::Cat(cat) => cat.say(),
            AnimalEnum::Duck(duck) => duck.say(),
        }
    }
}

trait Animal{
    fn say(&self);
}

use std::ops::Add;
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self,  other: Point) -> Point {  
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
        
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_enum_vec() {
        let animal_enum_vec = vec![AnimalEnum::Dog(Dog {
            name: String::from("dog"),
        }), AnimalEnum::Cat(Cat {
            name: String::from("cat"),
        }), AnimalEnum::Duck(Duck{
            name: String::from("duck"),
        })];

        for animal in animal_enum_vec {
            animal.say();
        }
    }


    #[test]
    fn test_trait_object_vec() {
        let animal_trait_vec: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog{name: String::from("dog")}),
            Box::new(Cat{name: String::from("cat")}),
            Box::new(Duck{name: String::from("duck")})
        ];

        for animal in animal_trait_vec {
            animal.say();
        }
    }


    #[test]
    fn test_add_point() {
        let point = Point{x: 1, y: 2};
        let point2 = Point{x: 3, y: 4};
        let point3 = point + point2;
        assert_eq!(point3.x, 4);
        assert_eq!(point3.y, 6);
    }
}
