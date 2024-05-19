
#[cfg(test)]
mod tests {
    use crate::*;
    use bevy::prelude::*;

    #[test]
    fn test_rule_evaluation() {
        let facts = hashmap! {
            "score".to_string() => Fact::Int("score".to_string(), 42),
            "player".to_string() => Fact::String("player".to_string(), "Alice".to_string()),
            "is_alive".to_string() => Fact::Bool("is_alive".to_string(), true),
        };

        // Test IntEquals condition
        let int_equals_condition = Condition::IntEquals {
            fact_name: "score".to_string(),
            expected_value: 42,
        };
        assert!(int_equals_condition.evaluate(&facts));

        // Test StringEquals condition
        let string_equals_condition = Condition::StringEquals {
            fact_name: "player".to_string(),
            expected_value: "Alice".to_string(),
        };
        assert!(string_equals_condition.evaluate(&facts));

        // Test BoolEquals condition
        let bool_equals_condition = Condition::BoolEquals {
            fact_name: "is_alive".to_string(),
            expected_value: true,
        };
        assert!(bool_equals_condition.evaluate(&facts));
    }

    #[test]
    fn test_story_beat_evaluation() {
        let facts = hashmap! {
            "score".to_string() => Fact::Int("score".to_string(), 42),
            "player".to_string() => Fact::String("player".to_string(), "Alice".to_string()),
            "is_alive".to_string() => Fact::Bool("is_alive".to_string(), true),
        };

        let rule1 = Rule {
            name: "Rule1".to_string(),
            conditions: vec![
                Condition::IntEquals {
                    fact_name: "score".to_string(),
                    expected_value: 42,
                },
                Condition::StringEquals {
                    fact_name: "player".to_string(),
                    expected_value: "Alice".to_string(),
                },
                Condition::BoolEquals {
                    fact_name: "is_alive".to_string(),
                    expected_value: true,
                },
            ],
        };

        let story_beat = StoryBeat {
            name: "Beat1".to_string(),
            rules: vec![rule1],
            finished: false,
        };

        let mut story_beat_clone = story_beat.clone();
        story_beat_clone.evaluate(&facts);
        assert!(story_beat_clone.finished);
    }

    #[test]
    fn test_story_evaluation() {
        let facts = hashmap! {
            "score".to_string() => Fact::Int("score".to_string(), 42),
            "player".to_string() => Fact::String("player".to_string(), "Alice".to_string()),
            "is_alive".to_string() => Fact::Bool("is_alive".to_string(), true),
        };

        let rule1 = Rule {
            name: "Rule1".to_string(),
            conditions: vec![
                Condition::IntEquals {
                    fact_name: "score".to_string(),
                    expected_value: 42,
                },
                Condition::StringEquals {
                    fact_name: "player".to_string(),
                    expected_value: "Alice".to_string(),
                },
                Condition::BoolEquals {
                    fact_name: "is_alive".to_string(),
                    expected_value: true,
                },
            ],
        };

        let rule2 = Rule {
            name: "Rule2".to_string(),
            conditions: vec![Condition::IntMoreThan {
                fact_name: "score".to_string(),
                expected_value: 50,
            }],
        };

        let story_beat1 = StoryBeat {
            name: "Beat1".to_string(),
            rules: vec![rule1],
            finished: true, // Beat1 is already finished
        };

        let story_beat2 = StoryBeat {
            name: "Beat2".to_string(),
            rules: vec![rule2],
            finished: false,
        };

        let story = Story {
            name: "MyStory".to_string(),
            beats: vec![story_beat1, story_beat2],
            active_beat_index: 0,
        };

        let mut story_clone = story.clone();
        story_clone.evaluate_active_beat(&facts);
        assert_eq!(story_clone.active_beat_index, 1); // Beat2 is now active

        story_clone.evaluate_active_beat(&facts);
        assert!(story_clone.is_finished());
    }
}
