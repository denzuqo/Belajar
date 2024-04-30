fn main() {
	
	//let tuple = (12, [1, 2, 3], vec![4, 5, 6], "Tuple");
	let tuple: (i32, [i32; 3], Vec<i32>, &str) = (12, [1, 2, 3], vec![4, 5, 6], "Tuple");
	
	println!("Tuple ->> {:?}", tuple);
	println!("Tuple posisi 0 ->> {}", tuple.0);
	println!("Tuple posisi 1 ->> Data Array ->> {:?} ->> Key 1 ->> {}", tuple.1, tuple.1[1]);
}
