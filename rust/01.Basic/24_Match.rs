fn main() {
	
	let hari: &str = "sabtu";
	
	match hari {

		"senin"  => println!("Menggunakan baju Putih Merah"),
		"selasa" => println!("Menggunakan baju olahraga"),
		"rabu"   => println!("Menggunakan baju adat"),
		"kamis"  => println!("Menggunakan baju pramuka"),
		"jumat"  => println!("Menggunakan baju batik"),
		"sabtu"  => println!("Menggunakan baju cosplay!"),
		_	 => println!("Libur"),
	}
}
