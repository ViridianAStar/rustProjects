fn main(){

    let bob: [i32; 4] = [1, 2, 3, 4];
    let mut i = 0;

    while i <= 3{
        println!("{}", bob[i]);
        i = i + 1;
    }
    printsmth()
    
}

fn printsmth(){
    println!("something");
    print!("{}", ' ');
}