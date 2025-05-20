// 1.1.3
//
// チューリング機械の定義
// Post らによって改良された定義
//
// M = (Q, Σ, q_0, δ)
//
// Q は有限の状態集合
// Σ は有限のシンボル集合. B と表記する空白記号を含む
// q_0 ∈ Q は初期状態
// δ は状態遷移関数. 現在の状態 q と現在のヘッド位置 s の組を引数に取り、
//     次の状態 q' と現在のヘッド位置に書き出すシンボル s' と書き出した後のヘッドの移動方向を指定する3つ組を返す
//   δ: Q × Σ → Q × Σ × D
//

// 1.2.1
//
// <program> ::= (<Q>, <entryList>)              // 初期状態 q_0 と δ
// <entry>   ::= ((<Q>, <S>) => (<Q>, <S>, <D>)) // δ の要素
// <Q>       ::= <string>                        // 状態 Q . M | H
// <S>       ::= B | <string>                    // 集合 Σ . B | I | O
// <D>       ::= L | R                           // ヘッダーの移動方向

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
    M,
}

/// 状態遷移関数δ (<entryList>)
type Delta = Vec<((Q, S), (Q, S, D))>;

/// プログラム (<program>)
type Program = (Q, Delta);

/// テープ (<T>)
type Tape = (List, S, List);

fn add_one() -> Program {
    use D::L;
    use Q::{H, M};
    use S::{B, I, O};

    (
        M,
        vec![
            ((M, I), (M, O, L)),
            ((M, O), (H, I, L)),
            ((M, B), (H, I, L)),
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
        Some((_, (q_, s, d))) => exec(delta, q_, mov(*d, (left, *s, right))),
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
    fn test_add_one() {
        use S::{B, I, O};
        let t: Tape = (List::from([I, I, I]), I, List::from([]));
        let (q, delta) = add_one();
        let r = exec(&delta, &q, t);
        assert_eq!(r, (List::from([]), B, List::from([I, O, O, O, O])));
    }
}
