#[allow(dead_code)]
enum Buah {
	Apel,
   	Pisang,
   	Jeruk,
    	Anggur,
}

fn main() { 
	let buah_pilihan = Buah::Pisang;
	
	match buah_pilihan {
		
		Buah::Apel	=> println!("Memilih apel."),
        	Buah::Pisang 	=> println!("Memilih pisang."),
    	 	Buah::Jeruk 	=> println!("Memilih jeruk."),
        	Buah::Anggur 	=> println!("Memilih anggur."),
	}
}
