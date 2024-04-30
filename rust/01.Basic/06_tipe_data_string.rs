fn main() {
	
	let nama   	 = "Budi";
	let alamat: &str = "
	->> Jl. Suka Kaya,
	->> Desa Tidak Suka Miskin, 
	->> Kecamatan Aamiin
";	
	let hobi	= String::from("Rebahan");

	println!("Nama aku : {nama}");
	println!("Alamat : \n{}", alamat);
	println!("Hobi : {}", hobi);
}
