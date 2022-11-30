fn main(){

    let bob: [i32; 4] = [1, 2, 3, 4];
    let mut i = 0;
    let n = i + 1;
    let mult_by_next = format!("{}", bob[i] * bob[n]);
    let mult_by_next_bin = format!("{:b}", bob[i] * bob[n]);

    while i <= 3{
        println!("{}", bob[i]);
        let binary_rep = format!("{:b}", bob[i]);
        println!("{}", binary_rep);
       
        if i <= 2 {
            println!("{}", mult_by_next);
            println!("{}", mult_by_next_bin)
        } else if i == 3 {
           let mult_by_next = format!("{}", bob[i] * bob[i]);
           let mult_by_next_bin = format!("{:b}", bob[i] * bob[i]);
           println!("{}", mult_by_next);
           println!("{}", mult_by_next_bin)
        }
        i = i + 1;
    }
    printsmth()
    
}

fn printsmth(){
    println!("something");
    print!("{}", ' ');
}