fn main(){
    //type annotation, i => integer type
    let x : i32 = 5;
    // let y : i32; // warning not used
    // we can place an _ to prevent this warning
    let _y : i32;

    //assert equality ( vars are equal)
    assert_eq!(x,5);
    println!("Success!");

}