use aoc2022::time_run2;

const INPUT: &str = include_str!("../inputs/13");

#[time_run2("13")]
fn main() {
    distress_signal(INPUT)
}

#[derive(Debug, PartialEq, Eq)]
enum Cmp {
    Ordered,
    Indetermined,
    NotOrdered,
}

impl Cmp {
    fn to_bool(self) -> bool {
        match self {
            Cmp::Ordered => true,
            Cmp::Indetermined => panic!("non determined ordering"),
            Cmp::NotOrdered => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
    Value(u64),
    List { elems: Vec<Element> },
}

impl Element {
    fn from_char_array(s: &[char]) -> Self {
        let mut elements: Vec<Element> = vec![];

        // Get rid of the beginning and end of the outer most brackets -- this assumes we always parse a list
        // rather than a standalone value.
        let mut rest = &s[1..s.len() - 1];

        let mut current_chars: Vec<char> = vec![];
        while !rest.is_empty() {
            match rest[0] {
                '[' => {
                    // Find the associated closed bracket.
                    let mut matching_closed_brackets_index = 0;
                    let mut matching_closed_braces = 0;

                    for (i, c) in rest[1..].iter().enumerate() {
                        // We have one more matching brace to find
                        if *c == '[' {
                            matching_closed_braces -= 1;
                        } else if *c == ']' {
                            matching_closed_braces += 1;
                            matching_closed_brackets_index = 1 + i;
                        }
                        if matching_closed_braces == 1 {
                            break;
                        }
                    }
                    debug_assert!(matching_closed_braces != 0);

                    let inner_list_elem =
                        Element::from_char_array(&rest[0..matching_closed_brackets_index + 1]);

                    elements.push(inner_list_elem);
                    rest = &rest[matching_closed_brackets_index + 1..]
                }
                ',' => {
                    if !current_chars.is_empty() {
                        let s: String = current_chars.iter().collect();
                        elements.push(Element::Value(s.parse::<u64>().unwrap()));
                        current_chars.clear();
                    }

                    rest = &rest[1..]
                }
                digit => {
                    current_chars.push(digit);
                    rest = &rest[1..]
                }
            }
        }

        if !current_chars.is_empty() {
            let s: String = current_chars.iter().collect();
            elements.push(Element::Value(s.parse::<u64>().unwrap()));
        }

        Self::List { elems: elements }
    }

    fn is_ordered(&self, other: &Element) -> Cmp {
        match (self, other) {
            (Element::Value(self_value), Element::Value(other_value)) => {
                if self_value > other_value {
                    return Cmp::NotOrdered;
                }
                if self_value == other_value {
                    return Cmp::Indetermined;
                }

                return Cmp::Ordered;
            }
            (Element::Value(self_value), Element::List { .. }) => {
                let self_list = Element::List {
                    elems: vec![Element::Value(*self_value)],
                };
                return self_list.is_ordered(&other);
            }
            (Element::List { .. }, Element::Value(other_value)) => {
                let other_list = Element::List {
                    elems: vec![Element::Value(*other_value)],
                };
                return self.is_ordered(&other_list);
            }
            (Element::List { elems: elems_self }, Element::List { elems: elems_other }) => {
                for i in 0..elems_self.len() {
                    let other = match elems_other.get(i) {
                        Some(elem) => elem,
                        None => return Cmp::NotOrdered,
                    };

                    match elems_self[i].is_ordered(&other) {
                        Cmp::Ordered => return Cmp::Ordered,
                        Cmp::Indetermined => continue,
                        Cmp::NotOrdered => return Cmp::NotOrdered,
                    }
                }
                return Cmp::Ordered;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Cmp, Element};

    #[test]
    fn single_value() {
        let input: Vec<char> = "[10]".chars().collect();

        let elem = Element::from_char_array(&input);

        assert_eq!(
            elem,
            Element::List {
                elems: vec![Element::Value(10),]
            }
        );
    }

    #[test]
    fn values() {
        let input: Vec<char> = "[1,1,3,1,1]".chars().collect();

        let elem = Element::from_char_array(&input);

        assert_eq!(
            elem,
            Element::List {
                elems: vec![
                    Element::Value(1),
                    Element::Value(1),
                    Element::Value(3),
                    Element::Value(1),
                    Element::Value(1),
                ]
            }
        );
    }

    #[test]
    fn single_nested() {
        let input: Vec<char> = "[[1],4]".chars().collect();

        let elem = Element::from_char_array(&input);

        assert_eq!(
            elem,
            Element::List {
                elems: vec![
                    Element::List {
                        elems: vec![Element::Value(1)]
                    },
                    Element::Value(4),
                ]
            }
        );
    }

    #[test]
    fn multiple_nested() {
        let input: Vec<char> = "[[1],[2]]".chars().collect();

        let elem = Element::from_char_array(&input);

        assert_eq!(
            elem,
            Element::List {
                elems: vec![
                    Element::List {
                        elems: vec![Element::Value(1)]
                    },
                    Element::List {
                        elems: vec![Element::Value(2)]
                    },
                ]
            }
        );
    }

    #[test]
    fn double_nested() {
        let input: Vec<char> = "[[[1],2],3]".chars().collect();

        let elem = Element::from_char_array(&input);

        assert_eq!(
            elem,
            Element::List {
                elems: vec![
                    Element::List {
                        elems: vec![
                            Element::List {
                                elems: vec![Element::Value(1)]
                            },
                            Element::Value(2)
                        ]
                    },
                    Element::Value(3)
                ]
            }
        );
    }

    #[test]
    fn empty_nested() {
        let input: Vec<char> = "[[]]".chars().collect();

        let elem = Element::from_char_array(&input);

        assert_eq!(
            elem,
            Element::List {
                elems: vec![Element::List { elems: vec![] }]
            }
        );
    }

    #[test]
    fn empty_multiple_nested() {
        let input: Vec<char> = "[[[[]]],[]]".chars().collect();

        let elem = Element::from_char_array(&input);

        assert_eq!(
            elem,
            Element::List {
                elems: vec![
                    Element::List {
                        elems: vec![Element::List {
                            elems: vec![Element::List { elems: vec![] }]
                        }]
                    },
                    Element::List { elems: vec![] }
                ]
            }
        );
    }

    #[test]
    fn is_ordered_simple() {
        let left: Vec<char> = "[1,2,4]".chars().collect();
        let right: Vec<char> = "[1,2,5]".chars().collect();

        let elem_l = Element::from_char_array(&left);
        let elem_r = Element::from_char_array(&right);

        assert_eq!(elem_l.is_ordered(&elem_r), Cmp::Ordered);
    }

    #[test]
    fn is_ordered_diff_size() {
        let left: Vec<char> = "[7,7,7,7]".chars().collect();
        let right: Vec<char> = "[7,7,7]".chars().collect();

        let elem_l = Element::from_char_array(&left);
        let elem_r = Element::from_char_array(&right);

        assert_eq!(elem_l.is_ordered(&elem_r), Cmp::NotOrdered);
    }

    #[test]
    fn is_ordered_nested() {
        let left: Vec<char> = "[[2,3,4]]".chars().collect();
        let right: Vec<char> = "[4]".chars().collect();

        let elem_l = Element::from_char_array(&left);
        let elem_r = Element::from_char_array(&right);

        assert_eq!(elem_l.is_ordered(&elem_r), Cmp::Ordered);
    }

    #[test]
    fn is_ordered_empty_nested() {
        let left: Vec<char> = "[[[[]]]]".chars().collect();
        let right: Vec<char> = "[[]]".chars().collect();

        let elem_l = Element::from_char_array(&left);
        let elem_r = Element::from_char_array(&right);

        assert_eq!(elem_l.is_ordered(&elem_r), Cmp::NotOrdered);
    }
}

fn distress_signal(i: &str) -> (String, String) {
    let groups: Vec<&str> = i.split("\n\n").collect();
    let pairings: Vec<(Element, Element)> = groups
        .into_iter()
        .map(|group| {
            let v: Vec<&str> = group.split("\n").collect();
            let first: Vec<char> = v[0].chars().collect();
            let second: Vec<char> = v[1].chars().collect();
            (
                Element::from_char_array(&first),
                Element::from_char_array(&second),
            )
        })
        .collect();

    let mut ordered_indices: Vec<usize> = vec![];
    for (i, pair) in pairings.iter().enumerate() {
        if pair.0.is_ordered(&pair.1) == Cmp::Ordered {
            ordered_indices.push(i + 1)
        }
    }

    let ordered_indices = dbg!(ordered_indices);

    let part1 = ordered_indices.into_iter().sum::<usize>().to_string();

    (part1, "".to_string())
}
