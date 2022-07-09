struct Area {
    length: f32,
    width: f32,
}

struct AreaT {
    length: f32,
    width: f32,
    height: f32,
}
// Implements functions to AreaT struct
impl AreaT {
    fn multiply(&self) -> f32 {
        return self.length * self.width * self.height;
    }
}
// Implements functions to Area struct
impl Area {
    fn multiply(&self) -> f32 {
        return self.length * self.width;
    }
    fn circle(&self) -> f32 {
        return self.length * 3.14;
    }
    fn is_square(&self) {
        if self.width == self.length {
            println!("is square");
        } else {
            println!("Not a square")
        }
    }
}

fn main() {
    // calling test function
    test();
}

// test function
fn test() {
    // test for Area
    let t1 = Area {
        length: 10.,
        width: 10.,
    };
    println!("test 1: {}", t1.multiply());

    // test for AreaT
    let t2 = AreaT {
        length: 10.,
        width: 10.,
        height: 10.,
    };
    println!("test 2: {}", t2.multiply());

    // test for circle function impl
    let t3 = Area {
        length: 10.,
        width: 10.,
    };
    println!("test 3: {}", t3.circle());

    // test for is_square function
    let t4 = Area {
        length: 10.,
        width: 9.,
    };
    println!("test 4: {:?}", t4.is_square());
}

