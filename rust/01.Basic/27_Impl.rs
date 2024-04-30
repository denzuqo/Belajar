#[derive(Debug)]
#[allow(dead_code)]
struct Data{
	kelas: i32,
}

impl Data {

	fn new(kelas: i32) -> Data {
		
		Data{ kelas: kelas  }
	}

	fn nama() {
		println!("Nama ->> Budi");
	}
}

fn main() {
	
	Data::nama();
	
	let data = Data::new(10);
	
	println!("Kelas ->> {}", data.kelas);

	println!("Raw ->> {:#?}", data);
}
