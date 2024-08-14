


fn main() {
    // println!("{:?}",is_prime(1200099999998989988));
    println!("{:?}",find_primes(100));
    println!("{:?}",check_if_composite(12))
}
// time complexity for this =O(sqrt(n))
fn is_prime(n:u128)->bool{
    // assume every number is prime at first
    let mut flag=true;
    let somelimit=(n as f64).sqrt() as u128;
    
    for i in 2..=somelimit{
        if n%i==0{
            flag=false;
        }
    }
    flag
}
// time complexity
// O(n*sqrt(n))
fn find_primes(n:u128)->Vec<u128>{
    let mut primes=Vec::new();
    for i in 2..n{
        if is_prime(i){
            primes.push(i)
        }
    }
    primes
}

fn check_if_composite(n:u128)->String{
    if is_prime(n){
       format!("{} is a prime number",&n)
    }
    else{
        format!("{} is a composite number",&n)
    }
}

