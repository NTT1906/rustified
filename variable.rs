const _SIGNED_INT: i32 = 1i32;
const _UNSIGNED_INT: u32 = 3u32;

fn main() {
    // https://rust-tieng-viet.github.io/basic/variables/index.html
    let _tmp = _SIGNED_INT;
    let mut changeable: i32 = _tmp + 1;
    println!("{}, {}", _tmp, changeable);
    changeable = changeable + 1;
    println!("{}, {}", _tmp, changeable);
    {
        let _tmp = _UNSIGNED_INT;
        println!("{}, {}", _tmp, changeable);
    }
    println!("{}, {}", _tmp, changeable);

    let a = {
        let y = 10;
        let z = 100;

        y + z
    };

    println!("a = {}", a);

    let mut b = 1;
    b = 3;
    let b = 2;

    println!("b = {}", b);
}
