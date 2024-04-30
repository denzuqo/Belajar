fn contoh_1(umur: i32) {

	if umur < 10 {
		
		println!("Masih Bocah");
	} else if umur > 10 && umur <= 17 {
		
		println!("Mulai Puber");
	} else {
		
		println!("Siap-siap menempuh perjuangan berat");
	}
}

fn contoh_2(nama: &str) -> bool {
	
	let dia = if nama == "budi" {
	
		true
	} else {
		
		false
	};

	return dia;
}

fn main() {
	
	println!("Contoh 1");
	contoh_1(18);
	
	println!("\nContoh2 \n{}", contoh_2("budi"));
}
