trait TT {
    fn make() -> Z;
}

struct AS {}
struct BS ()

impl TT for AS {
    fn make() -> Z {
        Z::AS()
    }
}

impl TT for BS {
    fn make() -> Z {
        Z::BS()
    }
}

enum Z { AS(), BS() }

impl Z {
    fn new<T: TT>() -> Self {
        T::make()
    }

    fn newa(self) -> Self {
        println!("yes");
        Self::new::<AS>()
    }
}

fn main() {
    let a = Z::new::<AS>();
    let b = Z::new::<BS>();
    let a1 = Z::newa(BS());
    let mut v = Vec::new();
    v.push(a);
    v.push(b);
    for a in v.iter() {
        match a {
            Z::AS() => print!("AS"),
            Z::BS() => print!("BS"),
        }
    }
}
