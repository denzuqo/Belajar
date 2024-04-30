fn main() {

	println!("Logika AND 1 | 1 ->> {}", (true && true));
	println!("Logika AND 0 | 0 ->> {}", (false && false));
	println!("Logika AND 1 | 0 ->> {}", (true && false));
	println!("Logika AND 0 | 1 ->> {}\n\n", (false && true));
	
	println!("Logika OR 1 | 1 ->> {}", (true || true));
	println!("Logika OR 0 | 0 ->> {}", (false || false));
	println!("Logika OR 1 | 0 ->> {}", (true || false));
	println!("Logika OR 0 | 1 ->> {}\n\n", (false || true));
	
	
	println!("Logika NOT 1  ->> {}", !(true));
	println!("Logika NOT 0  ->> {}\n\n", !(false));

	println!("Logika NAND 1 | 1  ->> {}", !(true && true));
	println!("Logika NAND 0 | 0  ->> {}", !(false && false));
	println!("Logika NAND 1 | 0  ->> {}", !(true && false));
	println!("Logika NAND 0 | 1  ->> {}\n\n", !(false && true));
	
	println!("Logika NOR 1 | 1  ->> {}", !(true || true));
	println!("Logika NOR 0 | 0  ->> {}", !(false || false));
	println!("Logika NOR 1 | 0  ->> {}", !(true || false));
	println!("Logika NOR 0 | 1  ->> {}\n\n", !(true || false));

	println!("XOR & XNOR ->> Implementasikan secara mandiri");
	println!(r"Dokumentasi ->> https://doc.rust-lang.org/book/appendix-02-operators.html");
}
