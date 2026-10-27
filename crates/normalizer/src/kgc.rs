//! The RML-KGC core rule set, in the order it runs. A rule whose id starts
//! with a number is that step of the paper's normal form (Min & Hartig,
//! Appendix A). A node that a rule creates links with `nf:origin` to the
//! node of the original document it stands for; see HANDBOOK: The canonical
//! pipeline.

use std::collections::{HashMap, HashSet};

use oxigraph::model::vocab::rdf;
use oxigraph::model::{
    GraphNameRef, NamedNodeRef, NamedOrBlankNode, QuadRef, Term,
};
use oxigraph::store::Store;

use crate::error::NormalizerResult;
use crate::{run_update, Action, Rule};

/// A SPARQL Update with the prefixes `rdf:`, `rml:` and `nf:`.
macro_rules! sparql {
    ($body:literal) => {
        concat!(
            "PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>\n",
            "PREFIX rml: <http://w3id.org/rml/>\n",
            "PREFIX nf: <",
            nf!(),
            ">\n",
            $body
        )
    };
}

macro_rules! update {
    ($body:literal) => {
        Action::Update(sparql!($body))
    };
}

pub const RULES: &[Rule] = &[
    Rule {
        id: "2-constant-shortcuts",
        description: "Expand the constant shortcuts of term maps",
        action: update!(
            "
DELETE {
    ?xs rml:subject ?s . ?xp rml:predicate ?p .
    ?xo rml:object ?o . ?xg rml:graph ?g .
}
INSERT {
    ?xs rml:subjectMap _:s . _:s rml:constant ?s ; nf:origin ?xs .
    ?xp rml:predicateMap _:p . _:p rml:constant ?p ; nf:origin ?xp .
    ?xo rml:objectMap _:o . _:o rml:constant ?o ; nf:origin ?xo .
    ?xg rml:graphMap _:g . _:g rml:constant ?g ; nf:origin ?xg .
}
WHERE {
    { ?xs rml:subject ?s } UNION { ?xp rml:predicate ?p }
    UNION { ?xo rml:object ?o } UNION { ?xg rml:graph ?g }
}"
        ),
    },
    Rule {
        id: "datatype-and-language-shortcuts",
        description: "Expand rml:datatype and rml:language into maps",
        action: update!(
            "
DELETE { ?xd rml:datatype ?d . ?xl rml:language ?l . }
INSERT {
    ?xd rml:datatypeMap _:d . _:d rml:constant ?d ; nf:origin ?xd .
    ?xl rml:languageMap _:l . _:l rml:constant ?l ; nf:origin ?xl .
}
WHERE { { ?xd rml:datatype ?d } UNION { ?xl rml:language ?l } }"
        ),
    },
    Rule {
        id: "join-condition-shortcuts",
        description: "Expand rml:child and rml:parent into maps",
        action: update!(
            "
DELETE { ?xc rml:child ?c . ?xp rml:parent ?p . }
INSERT {
    ?xc rml:childMap _:c . _:c rml:reference ?c ; nf:origin ?xc .
    ?xp rml:parentMap _:p . _:p rml:reference ?p ; nf:origin ?xp .
}
WHERE { { ?xc rml:child ?c } UNION { ?xp rml:parent ?p } }"
        ),
    },
    Rule {
        id: "term-types",
        description: "Make every term type explicit, except of constants",
        action: update!(
            "
INSERT { ?map rml:termType ?type }
WHERE {
    {
        ?x rml:subjectMap|rml:predicateMap|rml:graphMap|rml:datatypeMap ?map .
        BIND (rml:IRI AS ?type)
    } UNION {
        ?x rml:objectMap ?map .
        FILTER NOT EXISTS { ?map rml:parentTriplesMap ?parent }
        BIND (IF(EXISTS {
            ?map rml:reference|rml:languageMap|rml:datatypeMap
                |rml:functionExecution ?v
        }, rml:Literal, rml:IRI) AS ?type)
    }
    FILTER NOT EXISTS { ?map rml:termType ?given }
    FILTER NOT EXISTS { ?map rml:constant ?constant }
}"
        ),
    },
    Rule {
        id: "1-classes",
        description:
            "Turn each class of a subject map into a predicate-object \
                      map",
        action: update!(
            "
DELETE { ?sm rml:class ?class }
INSERT {
    ?tm rml:predicateObjectMap _:pom .
    _:pom rml:predicateMap _:pm ; rml:objectMap _:om ; nf:origin ?sm .
    _:pm rml:constant rdf:type ; nf:origin ?sm .
    _:om rml:constant ?class ; nf:origin ?sm .
}
WHERE { ?tm rml:subjectMap ?sm . ?sm rml:class ?class }"
        ),
    },
    Rule {
        id: "same-source-joins",
        description: "Inline the parent subject map of a referencing object \
                      map without join condition over an effectively equal \
                      logical source",
        action: Action::Native(inline_same_source_joins),
    },
    Rule {
        id: "3-split-predicate-object-maps",
        description: "One predicate-object map per predicate map, object map \
                      and graph map, with the subject map's graph maps",
        action: update!(
            "
DELETE { ?tm rml:predicateObjectMap ?pom }
INSERT {
    ?tm rml:predicateObjectMap _:new .
    _:new rml:predicateMap ?pm ; rml:objectMap ?om ; rml:graphMap ?gm ;
        nf:origin ?origin .
}
WHERE {
    ?tm rml:predicateObjectMap ?pom .
    ?pom rml:predicateMap ?pm ; rml:objectMap ?om .
    OPTIONAL {
        { ?pom rml:graphMap ?gm } UNION { ?tm rml:subjectMap/rml:graphMap ?gm }
    }
    OPTIONAL { ?pom nf:origin ?o }
    BIND (COALESCE(?o, ?pom) AS ?origin)
} ;

# The split predicate-object maps, now linked from no triples map.
DELETE { ?pom ?property ?value }
WHERE {
    ?pom rml:predicateMap ?pm ; ?property ?value .
    FILTER NOT EXISTS { ?tm rml:predicateObjectMap ?pom }
}"
        ),
    },
    Rule {
        id: "subject-graph-maps",
        description: "Remove the graph maps of subject maps, which \
                      3-split-predicate-object-maps gave every \
                      predicate-object map",
        action: update!(
            "
DELETE { ?sm rml:graphMap ?gm }
WHERE { ?tm rml:subjectMap ?sm . ?sm rml:graphMap ?gm }"
        ),
    },
    Rule {
        id: "base-iris",
        description: "Give every triples map a base IRI, by default the \
                      processor's",
        action: update!(
            "
INSERT { ?tm rml:baseIRI ?base }
WHERE {
    ?tm rml:subjectMap ?sm .
    FILTER NOT EXISTS { ?tm rml:baseIRI ?given }
    nf:processor rml:baseIRI ?base .
}"
        ),
    },
    Rule {
        id: "5-split-triples-maps",
        description: "One triples map per predicate-object map",
        action: update!(
            "
DELETE { ?tm rml:predicateObjectMap ?pom }
INSERT { _:copy rml:predicateObjectMap ?pom ; nf:origin ?tm }
WHERE {
    ?tm rml:predicateObjectMap ?pom .
    FILTER EXISTS { ?tm rml:subjectMap ?sm }
} ;

INSERT { ?copy ?p ?v }
WHERE {
    ?copy nf:origin ?tm ; rml:predicateObjectMap ?pom .
    FILTER NOT EXISTS { ?copy rml:subjectMap ?own }
    ?tm ?p ?v .
    FILTER (?p IN (rdf:type, rml:logicalSource, rml:subjectMap, rml:baseIRI))
}"
        ),
    },
];

/// Inlines the parent subject map of a referencing object map without join
/// condition when every triples map with the object map reads a logical
/// source equal to the parent's: the same one, or one with an equal
/// description. The `nf:sameSource` links between equal sources exist only
/// while this rule runs.
fn inline_same_source_joins(store: &Store) -> NormalizerResult<()> {
    link_equal_sources(store)?;
    run_update(store, INLINE_SAME_SOURCE_JOINS)?;
    run_update(store, sparql!("DELETE WHERE { ?a nf:sameSource ?b }"))
}

const INLINE_SAME_SOURCE_JOINS: &str = sparql!(
    "
DELETE { ?om rml:parentTriplesMap ?ptm ; a rml:RefObjectMap }
INSERT { ?om ?p ?v ; nf:parentTriplesMap ?ptm }
WHERE {
    ?om rml:parentTriplesMap ?ptm .
    FILTER NOT EXISTS { ?om rml:joinCondition ?jc }
    OPTIONAL { ?ptm rml:logicalSource ?pls }
    # Every triples map with this object map reads an equal source.
    FILTER NOT EXISTS {
        ?tm rml:predicateObjectMap/rml:objectMap ?om .
        OPTIONAL { ?tm rml:logicalSource ?ls }
        FILTER (!IF(BOUND(?ls), BOUND(?pls) && (?ls = ?pls
            || EXISTS { ?ls nf:sameSource ?pls }), !BOUND(?pls)))
    }
    ?ptm rml:subjectMap ?psm .
    ?psm ?p ?v .
    FILTER (?p IN (rml:constant, rml:reference, rml:template,
        rml:functionExecution, rml:termType))
}"
);

/// Adds `ls1 nf:sameSource ls2` for every two distinct logical sources that
/// are equal by value: the same description, ignoring `rdf:type`, compared
/// recursively through blank nodes and described IRIs. The links are added
/// after all comparisons, so they take no part in them.
fn link_equal_sources(store: &Store) -> NormalizerResult<()> {
    let logical_source =
        NamedNodeRef::new_unchecked("http://w3id.org/rml/logicalSource");
    let same_source = NamedNodeRef::new_unchecked(concat!(nf!(), "sameSource"));
    let mut sources: Vec<Term> = store
        .quads_for_pattern(
            None,
            Some(logical_source),
            None,
            Some(GraphNameRef::DefaultGraph),
        )
        .map(|quad| quad.map(|quad| quad.object))
        .collect::<Result<_, _>>()?;
    sources.retain(|term| resource(term).is_some());
    sources.sort_by_key(Term::to_string);
    sources.dedup();
    let mut comparison = Comparison {
        store,
        descriptions: HashMap::new(),
        assumed: HashSet::new(),
    };
    let mut links = vec![];
    for (i, a) in sources.iter().enumerate() {
        for b in &sources[i + 1..] {
            if comparison.equal(a, b)? {
                links.push((resource(a).unwrap(), b.clone()));
                links.push((resource(b).unwrap(), a.clone()));
            }
        }
    }
    for (s, o) in &links {
        store.insert(QuadRef::new(
            s,
            same_source,
            o,
            GraphNameRef::DefaultGraph,
        ))?;
    }
    Ok(())
}

fn resource(term: &Term) -> Option<NamedOrBlankNode> {
    match term {
        Term::NamedNode(node) => Some(node.clone().into()),
        Term::BlankNode(node) => Some(node.clone().into()),
        _ => None,
    }
}

/// Compares terms by value. Two terms are equal if they are the same term,
/// or both are resources with a description, and each statement of one has
/// an equal statement in the other. Two distinct IRIs without description,
/// such as two reference formulations, differ. A pair met again while it is
/// being compared counts as equal, so a cycle of blank nodes ends.
struct Comparison<'a> {
    store: &'a Store,
    descriptions: HashMap<Term, Vec<(Term, Term)>>,
    assumed: HashSet<(Term, Term)>,
}

impl Comparison<'_> {
    fn equal(&mut self, a: &Term, b: &Term) -> NormalizerResult<bool> {
        if a == b {
            return Ok(true);
        }
        let pair = (a.clone(), b.clone());
        if self.assumed.contains(&pair) {
            return Ok(true);
        }
        let (da, db) = (self.description(a)?, self.description(b)?);
        if da.is_empty() || db.is_empty() {
            return Ok(false);
        }
        self.assumed.insert(pair.clone());
        let equal = self.covers(&da, &db)? && self.covers(&db, &da)?;
        self.assumed.remove(&pair);
        Ok(equal)
    }

    fn covers(
        &mut self,
        x: &[(Term, Term)],
        y: &[(Term, Term)],
    ) -> NormalizerResult<bool> {
        for (p, o) in x {
            let mut found = false;
            for (_, v) in y.iter().filter(|(q, _)| q == p) {
                if self.equal(o, v)? {
                    found = true;
                    break;
                }
            }
            if !found {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// The (predicate, object) pairs of a resource, without its `rdf:type`;
    /// none for a literal.
    fn description(
        &mut self,
        term: &Term,
    ) -> NormalizerResult<Vec<(Term, Term)>> {
        if let Some(description) = self.descriptions.get(term) {
            return Ok(description.clone());
        }
        let description = match resource(term) {
            None => vec![],
            Some(subject) => self
                .store
                .quads_for_pattern(
                    Some(subject.as_ref()),
                    None,
                    None,
                    Some(GraphNameRef::DefaultGraph),
                )
                .filter(|quad| {
                    quad.as_ref().map_or(true, |q| q.predicate != rdf::TYPE)
                })
                .map(|quad| quad.map(|q| (q.predicate.into(), q.object)))
                .collect::<Result<_, _>>()?,
        };
        self.descriptions.insert(term.clone(), description.clone());
        Ok(description)
    }
}
