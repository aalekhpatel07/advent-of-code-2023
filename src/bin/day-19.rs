use std::{collections::{HashMap}, str::FromStr};

use lazy_static::lazy_static;
use rayon::prelude::*;
use indicatif::ParallelProgressIterator;
use regex::Regex;


lazy_static! {
    static ref RULE_CONDITIONAL_RE: Regex = Regex::new(r"(?<part>[xmas])(?<conditional>[<>])(?<threshold>\d+):(?<target>\w+)").unwrap();
    static ref WORKFLOW_RE: Regex = Regex::new(r"(?<id>\w+)\{(?<rules>.*)\}").unwrap();
    static ref RATING_RE: Regex = Regex::new(r"\{(?<ratings>.*)\}").unwrap();
}


#[derive(Debug, Clone)]
pub struct Game {
    // workflows: Vec<Workflow>,
    ratings: Vec<Ratings>,
    workflows: HashMap<WorkflowId, Workflow>,
}

impl FromStr for Game {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (workflows, ratings) = s.split_once("\n\n").unwrap();

        let workflows = 
            workflows
            .lines()
            .map(Workflow::from_str)
            .collect::<Result<Vec<_>, String>>()?;

        let ratings = 
            ratings
            .lines()
            .map(Ratings::from_str)
            .collect::<Result<Vec<_>, String>>()?;

        let mut workflow_pool = std::collections::HashMap::new();

        for workflow in workflows.iter() {
            workflow_pool.insert(workflow.id.clone(), workflow.clone());
        }

        Ok(Self {
            ratings,
            workflows: workflow_pool
        })
    }
}

pub fn solve_part1(data: &str) -> usize {

    let game = data.parse::<Game>().unwrap();
    let mut accepted_ratings = std::collections::HashSet::new();

    for (rating_idx, rating) in game.ratings.iter().enumerate() {
        let mut workflow = game.workflows.get("in").unwrap().clone();
        loop {
            let (accepted, maybe_next_workflow) = workflow.process(rating);
            let Some(next_workflow_id) = maybe_next_workflow else {
                if accepted {
                    accepted_ratings.insert(rating_idx);
                }
                break;
            };
            workflow = game.workflows.get(&next_workflow_id).unwrap().clone();
        }
    }

    accepted_ratings
    .into_iter()
    .map(|rating| game.ratings[rating].inner.values().sum::<usize>())
    .sum()
}

pub fn solve_part2(data: &str) -> usize {

    let game = data.parse::<Game>().unwrap();

    Ratings::distinct_combinations()
    .par_bridge()
    .progress_count(4_000 * 4_000 * 4_000 * 4_000)
    .filter_map(|rating| {
        let mut workflow = game.workflows.get("in").unwrap().clone();
        loop {
            let (accepted, maybe_next_workflow) = workflow.process(&rating);
            let Some(next_workflow_id) = maybe_next_workflow else {
                if accepted {
                    return Some(true)
                } else {
                    return None;
                }
            };
            workflow = game.workflows.get(&next_workflow_id).unwrap().clone();
        }
    })
    .count()
}


pub fn main() {
    let data = include_str!("../../data/19.in");
    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data));
}


#[derive(Debug, Clone)]
pub struct Ratings {
    inner: HashMap<Part, usize>   
}

impl Ratings {
    pub fn distinct_combinations() -> impl Iterator<Item=Self> {
        (1..=4000)
        .flat_map(|x| 
            (1..=4000)
            .flat_map(move |m| {
                (1..=4000)
                .flat_map(move |a| {
                    (1..=4000)
                    .map(move |s| {
                        Ratings { inner: HashMap::from_iter(vec![(Part::X, x), (Part::M, m), (Part::A, a), (Part::S, s)])}
                    })
                })
            })
        )
    }
}

impl AsRef<HashMap<Part, usize>> for Ratings {
    fn as_ref(&self) -> &HashMap<Part, usize> {
        &self.inner
    }
}

impl FromStr for Ratings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let captures = RATING_RE.captures(s).expect("malformed ratings?");
        let inner = 
            captures
            .name("ratings")
            .unwrap()
            .as_str()
            .split(',')
            .map(|rating_str| {
                let (part_str, value) = rating_str.split_once('=').unwrap();
                let value = value.parse::<usize>().unwrap();
                let part: Part = part_str.chars().next().unwrap().into();
                (part, value)
            })
            .collect::<HashMap<Part, usize>>();

        Ok(Self {
            inner
        })
    }
}


#[derive(Debug, Clone)]
pub struct Workflow {
    id: WorkflowId,
    rules: Vec<Rule>
}

impl std::str::FromStr for Workflow {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let captures = WORKFLOW_RE.captures(s).expect("malformed workflow?");

        let id = captures.name("id").unwrap().as_str().to_string();
        let rules = captures.name("rules").unwrap().as_str();

        let rules = 
            rules
            .split(',')
            .map(Rule::from_str)
            .collect::<Result<Vec<_>, String>>()?;

        Ok(Self {
            id,
            rules,
        })
    }   
}

impl Workflow {
    pub fn process(&self, ratings: &Ratings) -> (bool, Option<WorkflowId>) {

        let mut index = 0;

        while index < self.rules.len() {

            match self.rules[index].clone() {
                Rule::Conclude { accept } => return (accept, None),
                Rule::ConcludeIfGreaterThan { part, accept, value } => {
                    if *ratings.inner.get(&part).unwrap() > value {
                        return (accept, None);
                    }
                },
                Rule::ConcludeIfLessThan { part, accept, value } => {
                    if *ratings.inner.get(&part).unwrap() < value {
                        return (accept, None);
                    }
                },
                Rule::Jump { target } => {
                    return (false, Some(target));
                },
                Rule::JumpIfGreaterThan { part, value, target } => {
                    if *ratings.inner.get(&part).unwrap() > value {
                        return (false, Some(target));
                    }
                },
                Rule::JumpIfLessThan { part, value, target } => {
                    if *ratings.inner.get(&part).unwrap() < value {
                        return (false, Some(target));
                    }
                },
            }
            index += 1;
        }
        unreachable!("");
    }
}

pub type WorkflowId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Part {
    X,
    M,
    A,
    S
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    JumpIfGreaterThan { 
        part: Part,
        value: usize,
        target: WorkflowId
    },
    JumpIfLessThan {
        part: Part,
        value: usize,
        target: WorkflowId
    },
    Jump {
        target: WorkflowId
    },
    ConcludeIfGreaterThan {
        part: Part,
        accept: bool,
        value: usize,
    },
    ConcludeIfLessThan {
        part: Part,
        accept: bool,
        value: usize,
    },
    Conclude {
        accept: bool,
    }
}

impl From<char> for Part {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("unknown part?!")
        }
    }
}


impl std::str::FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match RULE_CONDITIONAL_RE.captures(s) {
            None => {
                Ok(match s {
                    "A" => Rule::Conclude { accept: true },
                    "R" => Rule::Conclude { accept: false },
                    _ => Rule::Jump { target: s.to_string() }
                })
            },
            Some(capture) => {
                let part: Part = capture.name("part").unwrap().as_str().chars().next().unwrap().into();
                let conditional = capture.name("conditional").unwrap().as_str().chars().next().unwrap();
                let threshold = capture.name("threshold").unwrap().as_str().parse::<usize>().unwrap();
                let target = capture.name("target").unwrap().as_str();

                match (conditional, target) {
                    ('<', "A" | "R") => {
                        Ok(Rule::ConcludeIfLessThan { part, accept: target == "A", value: threshold })
                    },
                    ('>', "A" | "R") => {
                        Ok(Rule::ConcludeIfGreaterThan { part, accept: target == "A", value: threshold })
                    },
                    ('<', _) => {
                        Ok(Rule::JumpIfLessThan { part, target: target.to_string(), value: threshold })
                    },
                    ('>', _) => {
                        Ok(Rule::JumpIfGreaterThan { part, target: target.to_string(), value: threshold })
                    },
                    _ => Err(format!("Unknown case: {}", s))
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smol() {
        let data = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

        assert_eq!(solve_part1(data), 19114);
        assert_eq!(solve_part2(data), 167409079868000);
    }
    #[test]
    fn parse_rule() {
        let rule_strs = vec![
            "a<2006:qkq",
            "m>2090:A",
            "rfg",
            "a>3333:R",
            "R",
            "A"
        ];

        let expected = vec![
            Rule::JumpIfLessThan { part: Part::A, value: 2006, target: "qkq".to_string() },
            Rule::ConcludeIfGreaterThan { part: Part::M, accept: true, value: 2090 },
            Rule::Jump { target: "rfg".to_string() },
            Rule::ConcludeIfGreaterThan { part: Part::A, accept: false, value: 3333 },
            Rule::Conclude { accept: false },
            Rule::Conclude { accept: true },
        ];

        for (idx, s) in rule_strs.iter().enumerate() {
            let rule = s.parse::<Rule>().unwrap();
            assert_eq!(rule, expected[idx]);
        }

    }
}