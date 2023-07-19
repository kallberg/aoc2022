use core::panic;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type JobRef = Rc<RefCell<Job>>;

#[derive(Debug, Clone)]
enum Job {
    Yell(i64),
    Human(i64),
    Add(JobRef, JobRef),
    Sub(JobRef, JobRef),
    Mul(JobRef, JobRef),
    Div(JobRef, JobRef),
}

#[derive(Clone)]
struct RootJob {
    lhs: JobRef,
    rhs: JobRef,
}

#[derive(Debug)]
enum SymbolicJob {
    Yell(i64),
    Human(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[derive(Debug)]
struct Monkey {
    name: String,
    symbolic_job: SymbolicJob,
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let (name, value) = value.split_once(':').unwrap();
        let value = value.trim();

        let values: Vec<String> = value
            .split_whitespace()
            .map(|str| str.to_string())
            .collect();

        if values.len() != 3 {
            return Monkey {
                name: name.to_string(),
                symbolic_job: if name != "humn" {
                    SymbolicJob::Yell(value.parse().unwrap())
                } else {
                    SymbolicJob::Human(value.parse().unwrap())
                },
            };
        }

        let lhs = values[0].to_string();
        let op = values[1].to_string();
        let rhs = values[2].to_string();

        let job = match op.as_str() {
            "+" => SymbolicJob::Add(lhs, rhs),
            "-" => SymbolicJob::Sub(lhs, rhs),
            "/" => SymbolicJob::Div(lhs, rhs),
            "*" => SymbolicJob::Mul(lhs, rhs),
            _ => panic!("bad operand {}", op),
        };

        Monkey {
            name: name.to_string(),
            symbolic_job: job,
        }
    }
}

fn grab_monkey<'a>(name: &str, monkeys: &'a [Monkey]) -> &'a Monkey {
    monkeys.iter().find(|monkey| monkey.name.eq(name)).unwrap()
}

fn make_job(name: &String, monkeys: &Vec<Monkey>, jobs: &mut HashMap<String, JobRef>) -> JobRef {
    if let Some(cached) = jobs.get(name) {
        return cached.clone();
    }

    let monkey = grab_monkey(name, monkeys);

    match &monkey.symbolic_job {
        SymbolicJob::Yell(value) => {
            let job = Job::Yell(*value);
            let job_ref = Rc::new(RefCell::new(job));
            jobs.insert(name.clone(), job_ref.clone());
            job_ref
        }
        SymbolicJob::Add(lhs, rhs) => {
            let job = Job::Add(make_job(lhs, monkeys, jobs), make_job(rhs, monkeys, jobs));
            let job_ref = Rc::new(RefCell::new(job));
            jobs.insert(name.clone(), job_ref.clone());
            job_ref
        }
        SymbolicJob::Sub(lhs, rhs) => {
            let job = Job::Sub(make_job(lhs, monkeys, jobs), make_job(rhs, monkeys, jobs));
            let job_ref = Rc::new(RefCell::new(job));
            jobs.insert(name.clone(), job_ref.clone());
            job_ref
        }
        SymbolicJob::Mul(lhs, rhs) => {
            let job = Job::Mul(make_job(lhs, monkeys, jobs), make_job(rhs, monkeys, jobs));
            let job_ref = Rc::new(RefCell::new(job));
            jobs.insert(name.clone(), job_ref.clone());
            job_ref
        }
        SymbolicJob::Div(lhs, rhs) => {
            let job = Job::Div(make_job(lhs, monkeys, jobs), make_job(rhs, monkeys, jobs));
            let job_ref = Rc::new(RefCell::new(job));
            jobs.insert(name.clone(), job_ref.clone());
            job_ref
        }
        SymbolicJob::Human(value) => {
            let job = Job::Human(*value);
            let job_ref = Rc::new(RefCell::new(job));
            jobs.insert(name.clone(), job_ref.clone());
            job_ref
        }
    }
}

impl Job {
    fn wrap(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    fn yell(&self, human_input: Option<i64>) -> i64 {
        match self {
            Job::Yell(value) => *value,
            Job::Add(lhs, rhs) => lhs.borrow().yell(human_input) + rhs.borrow().yell(human_input),
            Job::Sub(lhs, rhs) => lhs.borrow().yell(human_input) - rhs.borrow().yell(human_input),
            Job::Mul(lhs, rhs) => lhs.borrow().yell(human_input) * rhs.borrow().yell(human_input),
            Job::Div(lhs, rhs) => lhs.borrow().yell(human_input) / rhs.borrow().yell(human_input),
            Job::Human(default) => match human_input {
                Some(input) => input,
                None => *default,
            },
        }
    }

    fn depends_on_human(&self) -> bool {
        match self {
            Job::Yell(_) => false,
            Job::Human(_) => true,
            Job::Add(lhs, rhs) | Job::Sub(lhs, rhs) | Job::Mul(lhs, rhs) | Job::Div(lhs, rhs) => {
                lhs.borrow().depends_on_human() || rhs.borrow().depends_on_human()
            }
        }
    }

    fn optimize(&self) -> Self {
        match self {
            Job::Yell(_) => self.clone(),
            Job::Human(_) => self.clone(),
            Job::Add(lhs, rhs) => {
                if !lhs.borrow().depends_on_human() && !rhs.borrow().depends_on_human() {
                    let self_result = self.yell(None);
                    Job::Yell(self_result)
                } else {
                    Job::Add(
                        lhs.borrow().optimize().wrap(),
                        rhs.borrow().optimize().wrap(),
                    )
                }
            }
            Job::Sub(lhs, rhs) => {
                if !lhs.borrow().depends_on_human() && !rhs.borrow().depends_on_human() {
                    let self_result = self.yell(None);
                    Job::Yell(self_result)
                } else {
                    Job::Sub(
                        lhs.borrow().optimize().wrap(),
                        rhs.borrow().optimize().wrap(),
                    )
                }
            }
            Job::Mul(lhs, rhs) => {
                if !lhs.borrow().depends_on_human() && !rhs.borrow().depends_on_human() {
                    let self_result = self.yell(None);
                    Job::Yell(self_result)
                } else {
                    Job::Mul(
                        lhs.borrow().optimize().wrap(),
                        rhs.borrow().optimize().wrap(),
                    )
                }
            }
            Job::Div(lhs, rhs) => {
                if !lhs.borrow().depends_on_human() && !rhs.borrow().depends_on_human() {
                    let self_result = self.yell(None);
                    Job::Yell(self_result)
                } else {
                    Job::Div(
                        lhs.borrow().optimize().wrap(),
                        rhs.borrow().optimize().wrap(),
                    )
                }
            }
        }
    }

    fn root(&self) -> RootJob {
        match self {
            Job::Add(lhs, rhs) => RootJob {
                lhs: lhs.clone(),
                rhs: rhs.clone(),
            },
            _ => panic!("cannot make non-add job into root job"),
        }
    }

    fn flip(&self) -> Option<Job> {
        match self {
            Job::Yell(_) => None,
            Job::Human(_) => None,
            Job::Add(lhs, rhs) => {
                if !lhs.borrow().depends_on_human() {
                    Some(Job::Add(rhs.clone(), lhs.clone()))
                } else {
                    None
                }
            }
            Job::Sub(lhs, rhs) => {
                if !lhs.borrow().depends_on_human() {
                    Some(Job::Sub(
                        Job::Mul(rhs.clone(), Job::Yell(-1).wrap()).wrap(),
                        Job::Mul(lhs.clone(), Job::Yell(-1).wrap()).wrap(),
                    ))
                } else {
                    None
                }
            }
            Job::Mul(lhs, rhs) => {
                if !lhs.borrow().depends_on_human() {
                    Some(Job::Mul(rhs.clone(), lhs.clone()))
                } else {
                    None
                }
            }
            Job::Div(_, _) => None,
        }
    }

    fn transfer(&self, other: Job) -> Option<(Job, Job)> {
        let job = self.flip().unwrap_or(self.clone());

        match job {
            Job::Yell(_) => None,
            Job::Human(_) => None,
            Job::Add(lhs, rhs) if lhs.borrow().depends_on_human() => {
                Some((lhs.borrow().clone(), Job::Sub(other.wrap(), rhs)))
            }
            Job::Sub(lhs, rhs) if lhs.borrow().depends_on_human() => {
                Some((lhs.borrow().clone(), Job::Add(other.wrap(), rhs)))
            }
            Job::Mul(lhs, rhs) if lhs.borrow().depends_on_human() => {
                Some((lhs.borrow().clone(), Job::Div(other.wrap(), rhs)))
            }
            Job::Div(lhs, rhs) if lhs.borrow().depends_on_human() => {
                Some((lhs.borrow().clone(), Job::Mul(other.wrap(), rhs)))
            }
            _ => None,
        }
    }
}

impl RootJob {
    fn yell(&self, human_input: Option<i64>) -> (i64, i64) {
        (
            self.lhs.borrow().yell(human_input),
            self.rhs.borrow().yell(human_input),
        )
    }

    fn optimize(&self) -> RootJob {
        RootJob {
            lhs: self.lhs.borrow().optimize().wrap(),
            rhs: self.rhs.borrow().optimize().wrap(),
        }
    }

    fn simplify(&self) -> Option<RootJob> {
        if let Some((lhs, rhs)) = self.lhs.borrow().transfer(self.rhs.borrow().clone()) {
            return Some(RootJob {
                lhs: lhs.wrap(),
                rhs: rhs.wrap(),
            });
        }
        None
    }

    fn solve(&self) -> Option<i64> {
        let mut current = self.clone();

        while let Some(next) = current.simplify() {
            current = next;
        }

        let solved = current.optimize();

        let result = match (solved.lhs.borrow().clone(), solved.rhs.borrow().clone()) {
            (Job::Human(_), Job::Yell(value)) => Some(value),
            _ => None,
        };

        result
    }
}

pub fn solve_1(input: &str) -> String {
    let monkeys: Vec<Monkey> = input.lines().map(Monkey::from).collect();

    let mut jobs = HashMap::new();

    let root_job = make_job(&"root".to_string(), &monkeys, &mut jobs)
        .borrow()
        .root();

    let root_job = root_job.optimize();

    let (lhs, rhs) = root_job.yell(None);

    (lhs + rhs).to_string()
}
pub fn solve_2(input: &str) -> String {
    let monkeys: Vec<Monkey> = input.lines().map(Monkey::from).collect();

    let mut jobs = HashMap::new();

    let root_job = make_job(&"root".to_string(), &monkeys, &mut jobs)
        .borrow()
        .root();

    let root_job = root_job.optimize();

    let result = root_job.solve();

    result.unwrap_or(0).to_string()
}
