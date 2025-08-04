#[cfg(test)]
mod tests {
    use codecrafters_grep::pattern::{Pattern, Quantifier};

    #[test]
    fn test_literal_pattern() {
        let p = Pattern::parse("abc");
        assert!(matches!(p[0], Pattern::Literal(ref s) if *s == 'a'));
        assert!(matches!(p[1], Pattern::Literal(ref s) if *s == 'b'));
        assert!(matches!(p[2], Pattern::Literal(ref s) if *s == 'c'));
    }

    #[test]
    fn test_digit_pattern() {
        let p = Pattern::parse(r"\d");
        assert!(matches!(p[0], Pattern::Digit));
    }

    #[test]
    fn test_alphanumeric_pattern() {
        let p = Pattern::parse(r"\w");
        assert!(matches!(p[0], Pattern::Alphanumeric));
    }

    #[test]
    fn test_backreference() {
        let p = Pattern::parse(r"\5");
        assert!(matches!(p[0], Pattern::BackReference(n) if n == 5));
    }

    #[test]
    fn test_any_pattern() {
        let p = Pattern::parse(".");
        assert!(matches!(p[0], Pattern::Any));
    }
    #[test]
    fn test_positive_group_pattern() {
        let p = Pattern::from("[abc]");
        assert!(matches!(p, Pattern::PositiveGroup(_)));
    }

    #[test]
    fn test_negative_group_pattern() {
        let p = Pattern::parse("[^abc]");
        assert!(matches!(p[0], Pattern::NegativeGroup(_)));
    }

    #[test]
    fn test_either_pattern() {
        let p = Pattern::parse("(a|b|c)");
        assert!(matches!(p[0], Pattern::CaptureGroup(_)));
    }
    // #[test]
    // fn test_combined_patterns() {
    //     // Combination: digit, literal, alphanumeric, any, group, either
    //     let p = Pattern::parse(r"\dabc\w.(x|y|z)");
    //     assert!(matches!(p[0], Pattern::Digit));
    //     assert!(matches!(p[1], Pattern::Literal(ref s) if *s == 'a'));
    //     assert!(matches!(p[2], Pattern::Literal(ref s) if *s == 'b'));
    //     assert!(matches!(p[3], Pattern::Literal(ref s) if *s == 'c'));
    //     assert!(matches!(p[4], Pattern::Alphanumeric));
    //     assert!(matches!(p[5], Pattern::Any));
    //     assert!(matches!(p[6], Pattern::Alternate(_)));
    // }

    // #[test]
    // fn test_nested_groups_and_either() {
    //     let p = Pattern::parse(r"[^abc]\d[dfg](\d\d|\w\w)");
    //     assert!(matches!(p[0], Pattern::NegativeGroup(_)));
    //     assert!(matches!(p[1], Pattern::Digit));
    //     assert!(matches!(p[2], Pattern::PositiveGroup(_)));
    //     if let Pattern::Alternate(ref variants) = p[3] {
    //         assert!(matches!(&variants[0][..], [Pattern::Digit, Pattern::Digit]));
    //         assert!(matches!(
    //             &variants[1][..],
    //             [Pattern::Alphanumeric, Pattern::Alphanumeric]
    //         ));
    //     } else {
    //         panic!("Expected Either pattern at p[3]");
    //     }
    // }

    // #[test]
    // fn test_nested_either_with_groups() {
    //     let p = Pattern::parse(r"(a|[bc]|(d|e))");
    //     if let Pattern::Alternate(ref variants) = p[0] {
    //         // First variant: "a"
    //         assert!(matches!(&variants[0][..], [Pattern::Literal(ref s)] if *s == 'a'));
    //
    //         // Second variant: [bc] -> PositiveGroup with two literals
    //         if let [Pattern::PositiveGroup(ref group)] = &variants[1][..] {
    //             assert_eq!(group.len(), 2);
    //             assert!(matches!(group[0], Pattern::Literal(ref s) if *s == 'b'));
    //             assert!(matches!(group[1], Pattern::Literal(ref s) if *s == 'c'));
    //         } else {
    //             panic!("Expected PositiveGroup in second variant");
    //         }
    //
    //         // Third variant: (d|e) -> Either with two literals
    //         if let [Pattern::Alternate(ref nested_either)] = &variants[2][..] {
    //             assert_eq!(nested_either.len(), 2);
    //             assert!(matches!(&nested_either[0][..], [Pattern::Literal(ref s)] if *s == 'd'));
    //             assert!(matches!(&nested_either[1][..], [Pattern::Literal(ref s)] if *s == 'e'));
    //         } else {
    //             panic!("Expected nested Either in third variant");
    //         }
    //     } else {
    //         panic!("Expected Either pattern at p[0]");
    //     }
    // }

    #[test]
    fn test_literal_quantifier_pattern() {
        let p = Pattern::parse("a{3}");
        assert_eq!(p.len(), 1);
        if let Pattern::PatternWithQuantifier(inner, Quantifier::Literal(n)) = &p[0] {
            if let Pattern::Literal(ref s) = **inner {
                assert_eq!(*s, 'a');
                assert_eq!(*n, 3);
            }
        } else {
            panic!("Expected PatternWithQuantifier(Literal, LiteralQuantifier)");
        }
    }

    #[test]
    fn test_one_or_more_quantifier_pattern() {
        let p = Pattern::parse("b+");
        assert_eq!(p.len(), 1);
        if let Pattern::PatternWithQuantifier(inner, Quantifier::OneOrMore) = &p[0] {
            if let Pattern::Literal(ref s) = **inner {
                assert_eq!(*s, 'b');
            }
        } else {
            panic!("Expected PatternWithQuantifier(Literal, OneOrMore)");
        }
    }

    #[test]
    fn test_zero_or_one_quantifier_pattern() {
        let p = Pattern::parse("c?");
        assert_eq!(p.len(), 1);
        if let Pattern::PatternWithQuantifier(inner, Quantifier::ZeroOrOne) = &p[0] {
            if let Pattern::Literal(ref s) = **inner {
                assert_eq!(*s, 'c');
            }
        } else {
            panic!("Expected PatternWithQuantifier(Literal, ZeroOrOne)");
        }
    }

    // #[test]
    // fn test_nested_either_with_groups_and_quantifiers() {
    //     let p = Pattern::parse(r"(a+|[bc]{2}|(d?|e*))");
    //     if let Pattern::Alternate(ref variants) = p[0] {
    //         // First variant: "a+"
    //         if let [Pattern::PatternWithQuantifier(inner, Quantifier::OneOrMore)] = &variants[0][..]
    //         {
    //             if let Pattern::Literal(ref s) = **inner {
    //                 assert_eq!(*s, 'a');
    //             } else {
    //                 panic!("Expected Literal in first variant quantifier");
    //             }
    //         } else {
    //             panic!("Expected PatternWithQuantifier in first variant");
    //         }
    //
    //         // Second variant: [bc]{2} -> PositiveGroup with quantifier
    //         if let [Pattern::PatternWithQuantifier(group, Quantifier::Literal(n))] =
    //             &variants[1][..]
    //         {
    //             assert_eq!(*n, 2);
    //             if let Pattern::PositiveGroup(ref group) = **group {
    //                 assert_eq!(group.len(), 2);
    //                 assert!(matches!(group[0], Pattern::Literal(ref s) if *s == 'b'));
    //                 assert!(matches!(group[1], Pattern::Literal(ref s) if *s == 'c'));
    //             } else {
    //                 panic!("Expected PositiveGroup in second variant quantifier");
    //             }
    //         } else {
    //             panic!("Expected PatternWithQuantifier in second variant");
    //         }
    //
    //         // Third variant: (d?|e*) -> Either with two quantifiers
    //         if let [Pattern::Alternate(ref nested_either)] = &variants[2][..] {
    //             assert_eq!(nested_either.len(), 2);
    //             if let [Pattern::PatternWithQuantifier(inner, Quantifier::ZeroOrOne)] =
    //                 &nested_either[0][..]
    //             {
    //                 if let Pattern::Literal(ref s) = **inner {
    //                     assert_eq!(*s, 'd');
    //                 } else {
    //                     panic!("Expected Literal in nested first variant quantifier");
    //                 }
    //             } else {
    //                 panic!("Expected PatternWithQuantifier(ZeroOrOne) in nested first variant");
    //             }
    //             if let [Pattern::PatternWithQuantifier(inner, Quantifier::ZeroOrMore)] =
    //                 &nested_either[1][..]
    //             {
    //                 if let Pattern::Literal(ref s) = **inner {
    //                     assert_eq!(*s, 'e');
    //                 } else {
    //                     panic!("Expected Literal in nested second variant quantifier");
    //                 }
    //             } else {
    //                 panic!("Expected PatternWithQuantifier(ZeroOrMore) in nested second variant");
    //             }
    //         } else {
    //             panic!("Expected nested Either in third variant");
    //         }
    //     } else {
    //         panic!("Expected Either pattern at p[0]");
    //     }
    // }
}
