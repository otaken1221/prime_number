fn main() {
    let mut a:u32=3;
    println!("2");
    loop {
        let mut t:u32=3;
        let result:u32=loop {
            if a%t==0 {
                break a;
            }
            t+=2;   
        };
        if result==t { println!("{}",result) };
        a+=2;

        if a>u32::pow(2,31) { break }
    }
}
