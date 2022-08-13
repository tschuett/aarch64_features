fn main() {
    #[cfg(not(target_arch = "aarch64"))]
    {
        println!("this not an aarch64 cpu!");
    }

    #[cfg(target_arch = "aarch64")]
    {
        use aarch64_features::check_features;

        let features = check_features();

        println!();

        println!("let features = vec![");
        for feature in features {
            println!("  {},", feature.to_string());
        }
        println!("];");

        println!();
    }
}
