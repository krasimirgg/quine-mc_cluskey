extern crate quine_mc_cluskey;

use quine_mc_cluskey::*;
use quine_mc_cluskey::Bool::*;

#[test]
fn ident() {
    assert_eq!(prime_implicants(&And(vec![Term(0), Term(1)])), vec![Term::new(3)]);
    assert_eq!(prime_implicants(&True), vec![Term::new(0)]);
    assert_eq!(prime_implicants(&False), vec![]);
    assert_eq!(prime_implicants(&Term(0)), vec![Term::new(1)]);
}

#[test]
fn wikipedia() {
    let a = || Term(0);
    let b = || Term(1);
    let c = || Term(2);
    let d = || Term(3);
    let not = |x| Not(Box::new(x));
    let expr = Or(vec![
        And(vec![not(a()), b(), not(c()), not(d())]),
        And(vec![a(), not(b()), not(c()), not(d())]),
        And(vec![a(), not(b()), c(), not(d())]),
        And(vec![a(), not(b()), c(), d()]),
        And(vec![a(), b(), not(c()), not(d())]),
        And(vec![a(), b(), c(), d()]),
        And(vec![a(), not(b()), not(c()), d()]),
        And(vec![a(), b(), c(), not(d())]),
    ]);

    assert_eq!(
        prime_implicants(&expr),
        vec![
            Term::with_dontcare(2, 1),
            Term::with_dontcare(1, 6),
            Term::with_dontcare(1, 12),
            Term::with_dontcare(5, 10),
        ]
    );
}
