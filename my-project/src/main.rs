use std::time::{Instant};
use async_std::task;

async fn r#loop() {
    let mut a:u32=3;
    let mut v0: Vec<u32> = Vec::new();
    let mut v1: Vec<u32> = Vec::new();
    v0.push(2);
    let first_loop = task::spawn(async move {
        loop {
            let mut t:u32=3;
            loop {
                if a%t==0 {
                    break;
                }
                t+=2;   
            };
            //if result==t { println!("{}",result) };
            if a==t { 
                v0.push(a);
            };
            
            a+=2;
    
            if a>u32::pow(2,10) { break }
        }
    });
    let final_loop = task::spawn(async move {
        loop {
            let mut t:u32=u32::pow(2,10)+1;
            loop {
                if a%t==0 {
                    break;
                }
                t+=2;   
            };
            //if result==t { println!("{}",result) };
            if a==t { 
                v1.push(a);
            };
            
            a+=2;
    
            if a>u32::pow(2,20) { break }
        }
    });
    /*loop {
        let mut t:u32=3;
        loop {
            if a%t==0 {
                break;
            }
            t+=2;   
        };
        //if result==t { println!("{}",result) };
        if a==t { 
            v0.push(a);
        };
        
        a+=2;

        if a>u32::pow(2,20) { break }
    }
    */
    first_loop.await;
    final_loop.await;
    v0.append(&mut v1);
    let arr_str = v0.iter()
                     .map(|&x| x.to_string())
                     .collect::<Vec<String>>()
                     .join("\n");
    println!("{}", arr_str);
}


fn main() {
    //let start =Instant::now();
    let mut a:u32=3;
    //let mut count:usize = 1;
    //let mut array :[u32;10000] = [0;10000];
    let mut v0: Vec<u32> = Vec::new();
    v0.push(2);
    loop {
        let mut t:u32=3;
        loop {
            if a%t==0 {
                break;
            }
            t+=2;   
        };
        //if result==t { println!("{}",result) };
        if a==t { 
            v0.push(a);
        };
        
        a+=2;

        if a>u32::max_value() { break }
    }
    let arr_str = v0.iter()
                     .map(|&x| x.to_string())
                     .collect::<Vec<String>>()
                     .join("\n");
    println!("{}", arr_str);
    //let end =start.elapsed();
    //println!("{}.{:000000005}s", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
