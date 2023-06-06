

fn main() {
    
    let mut a:u32=2;
    //let mut count:usize = 1;
    //let mut array :[u32;10000] = [0;10000];
    //let mut v0: Vec<u32> = Vec::new();
    
    
    loop {
        let mut t:u32;
        if a>2 {t=3;}
        else {t=2;} 
        
        loop {
            if a%t==0 {
                break;
            }
            if a>2 {t+=2}
            else {t+=1} 
        };
        //if result==t { println!("{}",result) };
        if a==t { 
            println!("{}",a);
        };
        
        if a>2 {a+=2}
        else {a+=1}
        
        
        if a>u32::max_value() { break }
    }
    
    /*let arr_str = v0.iter()
                     .map(|&x| x.to_string())
                     .collect::<Vec<String>>()
                     .join("\n");
    println!("{}", arr_str);*/
    
    
}
