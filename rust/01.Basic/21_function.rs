fn nama() {
	
	println!("Nama : Budi");
}

fn hobi(hobi: &str) {
	
	println!("Hobi : {}", hobi);
}


fn game(game: &str) -> &str {
	
	//return game;
	game
}


fn main() {
	
	let game = game("Ep Ep Mex");
		
	nama();
	umur();
	hobi("Rebahan");
	println!("Game kesukaan : {}", game);
}

fn umur() {

	println!("Umur : 10 Tahun");
}
