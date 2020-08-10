#![allow(unused_imports)]
extern crate regex;

use std::io::{self, Read};
use std::sync::Arc;
use std::thread;
use hyper::{Body, Method, Request, Response, Server};

macro_rules! regex {
    ($re:expr) => {
        ::regex::Regex::new($re).unwrap()
    };
}

pub fn check(req: Request) {
    let variants = vec![
        regex!("agggtaaa|tttaccct"),
        regex!("[cgt]gggtaaa|tttaccc[acg]"),
        regex!("a[act]ggtaaa|tttacc[agt]t"),
        regex!("ag[act]gtaaa|tttac[agt]ct"),
        regex!("agg[act]taaa|ttta[agt]cct"),
        regex!("aggg[acg]aaa|ttt[cgt]ccct"),
        regex!("agggt[cgt]aa|tt[acg]accct"),
        regex!("agggta[cgt]a|t[acg]taccct"),
        regex!("agggtaa[cgt]|[acg]ttaccct"),
    ];
    println!("{:#?}", variants);
}
