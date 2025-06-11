fn main() {
    let mut somevec = vec![1, 2, 3, 4];
    let mut vec_drain = somevec.drain(0..2);

    let mut nvec = vec![vec_drain];

    //println!("vec_drain: {:?}", vec_drain.next().unwrap());
    
    println!("nvec: {:?}", &nvec);
    drop(vec_drain);
    println!("somevec: {:?}", &somevec);
}
