use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

use sophia_api::graph::CollectibleGraph;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::parser::turtle;

use super::error::ParseError;
use super::triplesmap_extractor::extract_triples_maps;
use super::ExtractorResult;
use crate::rml_model::Document;

fn extract_base_iri(input: &str) -> Option<String> {
    input
        .trim()
        .strip_prefix("@base")
        .map(|e| e[0..e.len() - 1].replace(['<', '>'], "").trim().to_string())
}

pub fn load_graph_bread(buf_read: impl BufRead) -> ExtractorResult<FastGraph> {
    let source = turtle::parse_bufread(buf_read);

    let graph_res = FastGraph::from_triple_source(source);
    match graph_res {
        Ok(it) => Ok(it),
        Err(err) => {
            Err(ParseError::GenericError(format!(
                "Something went wrong with sophia's turtle parsing: {}",
                err
            ))
            .into())
        }
    }
}

pub fn load_graph_str(input_str: &str) -> ExtractorResult<FastGraph> {
    let source = turtle::parse_str(input_str);

    let graph_res = FastGraph::from_triple_source(source);
    match graph_res {
        Ok(it) => Ok(it),
        Err(err) => {
            Err(ParseError::GenericError(format!(
                "Something went wrong with sophia's turtle parsing: {}",
                err
            ))
            .into())
        }
    }
}

/// Parses a mapping document. Every triples map without `rml:baseIRI` takes
/// the processor's `base_iri`, or else the document's `@base`.
pub fn parse_str(
    input_str: &str,
    base_iri: Option<&str>,
) -> ExtractorResult<Document> {
    let graph = load_graph_str(input_str)?;
    let triples_maps = extract_triples_maps(&graph)?;
    if triples_maps.is_empty() {
        return Err(ParseError::GenericError(
            "No Triples maps extracted for the input RML document".to_string(),
        )
        .into());
    }
    let base_iri = base_iri.map(str::to_string).or_else(|| {
        input_str.split('\n').filter_map(extract_base_iri).next()
    });
    Ok(Document::new(base_iri, triples_maps))
}

pub fn parse_file(path: PathBuf) -> ExtractorResult<Document> {
    if let Some(ext) = path.extension() {
        if ext != "ttl" {
            return Err(ParseError::ExtensionError(format!(
                "Extension does not exist {}",
                ext.to_str().unwrap()
            ))
            .into());
        }

        let mut buf_read = BufReader::new(File::open(path.clone())?);
        let mut input_str = String::new();
        buf_read.read_to_string(&mut input_str)?;
        return parse_str(&input_str, None);
    }

    Err(ParseError::IOErrorStr(format!(
        "File extention can't be determined for file: {}",
        path.to_str().unwrap()
    ))
    .into())
}

