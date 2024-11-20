use std::env;

fn main() {
	let available_repos = vec![
		"p9",
		"p10",
		"p11",
		"sisyphus"
	];

	let args: Vec<String> = env::args().collect();

	if args.len() == 1  {
		panic!("No repository provided");
	}
}
