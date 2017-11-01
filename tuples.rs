fn printtup(x: (i32, bool)) -> (bool, i32){

println!("{}",x.0);
println!("{}",x.1);
let (integer, boolean) = x;

(boolean, integer)
}


fn main(){
let tuple = (42, true);

println!("{:?}", printtup(tuple));


}
