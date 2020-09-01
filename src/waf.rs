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

pub fn req_check<T>(req: &Request<T>) {
    let variants = vec![
        regex!("agggtaaa|tttaccct"),
        regex!("[cgt]gggtaaa|tttaccc[acg]"),
        regex!("a[act]ggtaaa|tttacc[agt]t"),
        regex!("ag[act]gtaaa|tttac[agt]ct"),
        regex!("agg[act]taaa|ttta[agt]cct"),
        regex!("aggg[acg]aaa|ttt[cgt]ccct"),
        regex!("agggt[cgt]aa|tt[acg]accct"),
        regex!("agggta[cgt]a|t[acg]taccct"),
        regex!("([<＜]script[^>＞]*[>＞][\\s\\S]*?)"),
    ];

    let query = req.uri().query().unwrap();

    for variant in variants {
        let a = variant.captures(query);
        println!("{:#?}", a);
    }
}

pub fn white_check<T>(req: &Request<T>) {
    let variants = vec![
        regex!("([<＜]script[^>＞]*[>＞][\\s\\S]*?)"),
    ];

    let query = req.uri().query().unwrap();

    for variant in variants {
        let a = variant.captures(query);
        println!("{:#?}", a);
    }
}
