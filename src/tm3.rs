// TM

/// 方向
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum D {
    L,
    R,
}

/// シンボル
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum S {
    B,
    I,
    O,
}

/// 状態
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Q {
    H,
    INIT,
    ML,
    MR,
    W,
    FIN,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum A {
    Move(D),
    Write(S),
}

/// 状態遷移関数δ (<entryList>)
type Delta = Vec<((Q, S), (Q, A))>;

/// プログラム (<program>)
type Program = (Q, Delta);

/// テープ (<T>)
type Tape = (List, S, List);

fn add_one_2_3() -> Program {
    use A::{Move, Write};
    use D::{L, R};
    use Q::{FIN, H, INIT, ML, MR, W};
    use S::{B, I, O};

    (
        INIT,
        vec![
            ((INIT, B), (MR, Move(R))),
            ((MR, I), (MR, Move(R))),
            ((MR, O), (MR, Move(R))),
            ((MR, B), (W, Move(L))),
            ((W, I), (ML, Write(O))),
            ((W, O), (ML, Write(I))),
            ((W, B), (FIN, Write(I))),
            ((ML, I), (W, Move(L))),
            ((ML, O), (W, Move(L))),
            ((FIN, I), (H, Move(L))),
            ((FIN, O), (H, Move(L))),
        ],
    )
}

// Eval

// 純粋関数型データ構造の「リスト」ではない。連結リストでもない。
// Vec を逆順に扱って、先頭要素の追加と削除を O(1) で実現しただけのもの
#[derive(Clone, Eq, PartialEq)]
struct List(Vec<S>);

impl List {
    fn from<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = S>,
        I::IntoIter: DoubleEndedIterator<Item = S>,
    {
        List(iter.into_iter().rev().collect::<Vec<S>>())
    }

    fn cons(mut self, s: S) -> Self {
        match (s, self.0.is_empty()) {
            (S::B, true) => return self,
            (s, _) => {
                self.0.push(s);
                self
            }
        }
    }

    fn head(&self) -> S {
        self.0.last().copied().unwrap_or(S::B)
    }

    fn tail(mut self) -> Self {
        self.0.pop();
        Self(self.0)
    }
}

impl std::fmt::Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter().rev()).finish()
    }
}

fn move_l(tape: Tape) -> Tape {
    let (left, head, right) = tape;
    let h = left.head();
    (left.tail(), h, right.cons(head))
}

fn move_r(tape: Tape) -> Tape {
    let (left, head, right) = tape;
    (left.cons(head), right.head(), right.tail())
}

fn mov(d: D, tape: Tape) -> Tape {
    match d {
        D::L => move_l(tape),
        D::R => move_r(tape),
    }
}

fn exec(delta: &Delta, q: &Q, (left, head, right): Tape) -> Tape {
    match delta.iter().find(|(it, _)| it == &(*q, head)) {
        None => (left, head, right),
        Some((_, (q_, a))) => match a {
            A::Move(d) => exec(delta, q_, mov(*d, (left, head, right))),
            A::Write(s) => exec(delta, q_, (left, *s, right)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        use S::{B, I, O};
        let l = List::from([]);
        assert_eq!(l.head(), B);
        assert_eq!(l.tail(), List::from([]));
        let l = List::from([]);
        assert_eq!(l.cons(B), List::from([]));
        let l = List::from([I]);
        assert_eq!(l.head(), I);
        assert_eq!(l.tail(), List::from([]));
        let l = List::from([I, O]);
        assert_eq!(l.head(), I);
        assert_eq!(l.tail(), List::from([O]));
        let l = List::from([I, O, O]);
        assert_eq!(l.head(), I);
        assert_eq!(l.tail(), List::from([O, O]));
        assert_eq!(format!("{:?}", List::from([I, O, O])), "[I, O, O]");
    }

    #[test]
    fn test_add_one_2_3() {
        use S::{B, I, O};
        let t: Tape = (List::from([]), B, List::from([I, I, I, I]));
        let (q, delta) = add_one_2_3();
        let r = exec(&delta, &q, t);
        assert_eq!(r, (List::from([]), B, List::from([I, O, O, O, O])));
    }
}
