fn greet_world(){
	println!("Hello, world!");
	let southern_germany = "GriiB Gott!";
	let japan = "こんにちは世界";
	let regions = [southern_germany, japan];
	for region in regions.iter() {
		println!("{}", &region);
	}
}
fn main() {
	greet_world();
}
