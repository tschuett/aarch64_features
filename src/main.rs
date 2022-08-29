fn main() {
    #[cfg(not(target_arch = "aarch64"))]
    {
        println!("this not an aarch64 cpu!");
    }

    #[cfg(target_arch = "aarch64")]
    {
        use aarch64_features::check_features;
        use aarch64_features::cpu_type::detect_core;
        let features = check_features();

        println!();

        println!("let features = vec![");
        for feature in features {
            println!("  {:?},", feature);
        }
        println!("];");

        println!();

        detect_core();
    }
}
