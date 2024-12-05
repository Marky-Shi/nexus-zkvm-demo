use nexus_sdk::{
    compile::CompileOpts,
    nova::seq::{Generate, Nova, PP},
    Local, Prover, Verifiable,
};
 

const PACKAGE: &str = "guest";
 
fn main() {
    println!("Setting up Nova public parameters...");
    let pp: PP = PP::generate().expect("failed to generate parameters");
 
    let mut opts = CompileOpts::new(PACKAGE);
    opts.set_memlimit(8); // use an 8mb memory
 
    println!("Compiling guest program...");
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");
 
    let input = (3, 5);
 
    print!("Proving execution of vm...");
    let proof = prover
        .prove_with_input::<(i32,i32)>(&pp, &input)
        .expect("failed to prove program");
 
    println!(
        " output is {}!",
        proof
            .output::<i32>()
            .expect("failed to deserialize output")
    );
 
    println!(">>>>> Logging\n{}<<<<<", proof.logs().join("\n"));
 
    print!("Verifying execution...");
    proof.verify(&pp).expect("failed to verify proof");
 
    println!("  Succeeded!");
}