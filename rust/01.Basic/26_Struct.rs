struct Profile {
	
	nama: String,
	hobi: Vec<String>
}

fn main() {
	
	let data: Profile = Profile {
		nama: String::from("Budi"),
		hobi: vec![
			String::from("Rebahan"),
			String::from("Main game ep ep mex")
		],
	};
	
	println!("Nama ->> {}", data.nama);
	println!("Hobi ->> {} dan {}", data.hobi[0], data.hobi[1]);
}
