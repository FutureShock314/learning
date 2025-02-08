fn main() {
    let num = 3;

    for up in 1..=1_000_000 {
        if up % num == 0 {
            let divis = up / num;
            println!( "{divis} * {num} = {up}" );
        }
    }
}
