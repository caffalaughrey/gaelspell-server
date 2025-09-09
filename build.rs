fn main() {
    // Ensure libperl-config runs so libperl-rs picks up system Perl
    #[cfg(any(perlapi_ver30, perl_useithreads))]
    {
        println!("cargo:rerun-if-changed=build.rs");
    }
}




