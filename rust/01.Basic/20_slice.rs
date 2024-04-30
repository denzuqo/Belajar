fn main(){
	
	let data = [1, 2, 3, 4, 5];
	
	println!("Data ->> {:?}", data);
	println!("Slice 1.. ->> {:?}", &data[1..]);
	println!("Slice ..3 ->> {:?}", &data[..3]);
	println!("Slice 1..3 ->> {:?}", &data[1..3]);
	println!("Slice 1..=3 ->> {:?}", &data[1..=3]);
}
