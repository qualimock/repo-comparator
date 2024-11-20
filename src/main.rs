use std::env;
use repo_comparator::repo_comparator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

	let branch_a = repo_comparator::fetch_branch(&branch_a_name).await;
	let branch_b = repo_comparator::fetch_branch(&branch_b_name).await;

	let packages_a = repo_comparator::collect_packages(branch_a.clone());
	let packages_b = repo_comparator::collect_packages(branch_b.clone());

	let (in_a_not_in_b, in_b_not_in_a) = repo_comparator::compare_branches(&packages_a, &packages_b);
	let newer_packages = repo_comparator::compare_versions(&packages_a, &packages_b);

	let in_a_not_in_b_json = repo_comparator::packages_to_json(&in_a_not_in_b);
	let in_b_not_in_a_json = repo_comparator::packages_to_json(&in_b_not_in_a);
	let newer_packages_json = repo_comparator::packages_to_json(&newer_packages);

	Ok(())
}
