use std::fs::File;
use std::io::prelude::*;

use risp::eval_risp_script;
use risp::core::create_core_environment;

use patch::Patch;

use songs::amazon::create_amazon;
use songs::kirschblueten::create_kirschblueten;
use songs::test::create_test_song;
use songs::polly::create_polly;

pub fn load_patches() -> Vec<Patch> {
    let mut file = File::open("src/songs/amazon.risp").unwrap();
    let mut risp_code = String::new();
    file.read_to_string(&mut risp_code).unwrap();

    let mut env = create_core_environment();
    let result = eval_risp_script(&risp_code, &mut env);
    // println!("amazon = {:?}", result);
    assert!(result.is_ok());

    vec![create_test_song(), create_amazon(), create_kirschblueten(), create_polly()]
}