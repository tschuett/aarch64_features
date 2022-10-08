use strum::EnumCount;
fn main() {
    let count = aarch64_features::Feature::COUNT;
    println!("number of features: {count}");

    #[cfg(not(target_arch = "aarch64"))]
    {
        println!("this not an aarch64 cpu!");
    }

    #[cfg(target_arch = "aarch64")]
    {
        use aarch64_features::check_features;
        use aarch64_features::midr::Midr;
        let features = check_features();

        println!();

        println!("let features = vec![");
        for feature in features {
            println!("  {:?},", feature);
        }
        println!("];");

        println!();

        let midr = Midr::new();

        let _core = aarch64_features::cpu_type::Core::try_from(midr).unwrap();
    }
}
