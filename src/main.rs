use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use structopt::StructOpt;
use image::{io::Reader as ImgReader, Pixel, RgbaImage, imageops::crop};
use std::{fs::File, io::Read, path::PathBuf};
use std::env::current_dir;
use ff::{Field, PrimeField};
use bellpepper_core::{test_cs::TestConstraintSystem, ConstraintSystem};
use arecibo::{
    provider::{Bn256EngineZM, GrumpkinEngine},
    traits::{
      circuit::{StepCircuit, TrivialCircuit},
      snark::RelaxedR1CSSNARKTrait,
      Engine, Group,
    },
    CompressedSNARK, PublicParams, RecursiveSNARK,
  };


use circom_scotia::{calculate_witness, r1cs::CircomConfig};
use bellpepper_core::{num::AllocatedNum, SynthesisError};
use pasta_curves::vesta::Base as vesta_base;

type E1 = Bn256EngineZM;
type E2 = GrumpkinEngine;

/// structure for parsing circuit's inputs with serde
#[derive(Deserialize, Serialize)]
struct ProofInput {
    orig_pixels: Vec<u64>,
    new_pixels: Vec<u64>,
}

/// Basis for the circuits that correspond to transformations
/// name - name of the transformation
/// config - circom config with compiled r1cs and witness generator files
/// witness - calculated witness
#[derive(Clone)]
struct GenericCircuit<F: PrimeField> {
    name: String,
    config: CircomConfig<F>,
    witness: Vec<F>
}

impl<F: PrimeField> GenericCircuit<F> {
    
    fn new(trans_name: &str) -> Self {

        // getting the files 
        let root = current_dir().unwrap().join("circuits/").join(trans_name);
        let mut wtns = root.join(trans_name);
        wtns.set_extension("wasm");
        let mut r1cs = root.join(trans_name);
        r1cs.set_extension("r1cs");

        // generating circom config
        let cfg:CircomConfig<vesta_base> = CircomConfig::new(wtns, r1cs).unwrap();

        // parsing intput values for circuit 
        let json_inputs = parse_inputs(current_dir().unwrap().join("images/input.json"))?;

        // formatting inputs for witness calculation
        let orig: (String, Vec<vesta_base>) = (String::from("orig_pixels"), json_inputs.orig_pixels.into_iter().map(|pixel| vesta_base::from(pixel)).collect());
        let new: (String, Vec<vesta_base>) = (String::from("new_pixels"), json_inputs.new_pixels.into_iter().map(|pixel| vesta_base::from(pixel)).collect());
        let input = vec![orig, new];

        // witness calculation
        let witness = calculate_witness(&cfg, input, true).unwrap();
        println!("{:?}", witness);

        //circuit synthesis
        let mut cs = TestConstraintSystem::<vesta_base>::new();
        let output = circom_scotia::synthesize(
            &mut cs.namespace(|| trans_name),
            cfg.r1cs.clone(),
            Some(witness),
        ); 

        Self {
            name: String::from(trans_name),
            config: cfg,
            witness: witness
        }
    }
}

impl StepCircuit<Fr> for GenericCircuit {
    fn arity(&self) -> usize {
        2
    }

    fn synthesize<CS: ConstraintSystem<Fr>>(
        &self,
        cs: &mut CS,
        z: &[AllocatedNum<Fr>],
      ) -> std::result::Result<Vec<AllocatedNum<Fr>>, SynthesisError> {
        let out = circom_scotia::synthesize(cs, r1cs, witness)
    }
}

#[derive(StructOpt)]
#[structopt(name = "Data about image")]
struct ImgData {
    
    //Path to image file
    #[structopt(short = "p", long, default_value = "images/pepa.png")]
    img_path: PathBuf,

    //Path to metadata file
    #[structopt(short, long, default_value = "images/")]
    metadata_path: PathBuf,

}

fn prove(path: &PathBuf) -> anyhow::Result<()> {
    let mut img = ImgReader::open(path)?.decode()?.to_rgba8();
    let new_img = red_channel(&img);
    new_img.save("images/new_pepa.png");
    Ok(())
}

fn verify() {

}

fn red_channel(img: &RgbaImage) -> RgbaImage {
    let mut new_img = img.clone();
    for pixel in new_img.pixels_mut() {
        for c in pixel.channels_mut()[1..3].iter_mut(){
            *c = 0;
        }
    }
    new_img
}

fn parse_inputs(path: PathBuf) -> Result<ProofInput> {
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s);
    let parse: ProofInput = serde_json::from_str(&s)?;

    Ok(parse)
}

fn synt_circuit(tranformation: &str) -> Result<String> {
    let root = current_dir().unwrap().join("circuits/").join(tranformation);
    let mut wtns = root.join(tranformation);
    wtns.as_mut_os_string().push("_js");
    wtns.push(tranformation);
    wtns.set_extension("wasm");
    let mut r1cs = root.join(tranformation);
    r1cs.set_extension("r1cs");

    let cfg:CircomConfig<Fr> = CircomConfig::new(wtns, r1cs).unwrap();

    let json_inputs = parse_inputs(current_dir().unwrap().join("images/input.json"))?;


    let orig: (String, Vec<Fr>) = (String::from("orig_pixels"), json_inputs.orig_pixels.into_iter().map(|pixel| Fr::from(pixel)).collect());
    let new: (String, Vec<Fr>) = (String::from("new_pixels"), json_inputs.new_pixels.into_iter().map(|pixel| Fr::from(pixel)).collect());

    let input = vec![orig, new];

    let witness = calculate_witness(&cfg, input, true).unwrap();
    println!("{:?}", witness);

    let mut cs = TestConstraintSystem::<Fr>::new();

    let output = synthesize(
        &mut cs.namespace(|| "red_channel"),
        cfg.r1cs.clone(),
        Some(witness),
    ); 

    Ok(output)

}

fn proof_gen() {

    let circuit_primary = 
    let circuit_secondary = 

    let pp = PublicParams::<E1>::setup(c_primary, c_secondary, ck_hint1, ck_hint2)
    let mut recursive_snark = RecursiveSNARK::<E1>::new()
}

fn main() {
    let out = proof_gen("red_channel");
    println!("{:?}", out)



}
