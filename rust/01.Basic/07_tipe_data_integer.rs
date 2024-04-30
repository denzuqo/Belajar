fn main() {
	
	
	/*
		8    bit : -128 hingga 127
		16   bit : -32,768 hingga 32,767
		32   bit : -2,147,483,648 hingga 2,147,483,647
		64   bit : -9,223,372,036,854,775,808 hingga 9,223,372,036,854,775,807
		128  bit : 2^127
		i/u size : Mengikuti ukuran pada arsitektur sistem
		
		Prefiks i (Signed) yang ber-nilai deret positif & negatif - ... 0 ... +
		Prefiks u (Unsigned) yang ber-nilai berawalan dari nilai deret 0 ... +
	*/
	
	let _i8bit: i8    = -128;
	let _u8bit: u8    = 127;
	let _usize: usize = 1;

	println!("Nilai : \nSigned ->> {} \nUnsigned ->> {}", _i8bit, _u8bit);
}
