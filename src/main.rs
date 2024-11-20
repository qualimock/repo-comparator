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

	let branch_a_name = &args[1];
	let branch_b_name = &args[2];

	if !available_repos.contains(&branch_a_name.as_str()) {
		panic!("No such repository {branch_a_name}.\nAvailable repos: {available_repos:?}");
	}

	if !available_repos.contains(&branch_b_name.as_str()) {
		panic!("No such repository {branch_b_name}.\nAvailable repos: {available_repos:?}");
	}
}
