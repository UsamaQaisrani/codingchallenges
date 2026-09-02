use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::{char, collections::HashMap};

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
pub struct HuffmanNode {
    pub is_leaf: bool,
    pub character: Option<char>,
    pub frequency: u32,
    pub left_child: Option<Box<HuffmanNode>>,
    pub right_child: Option<Box<HuffmanNode>>,
}

pub fn create_pq(freqs: &HashMap<char, u32>) -> PriorityQueue<HuffmanNode, Reverse<u32>> {
    let mut pq: PriorityQueue<HuffmanNode, Reverse<u32>> = PriorityQueue::new();

    let mut entries: Vec<(&char, &u32)> = freqs.iter().collect();
    entries.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));

    for (key, val) in entries {
        pq.push(
            HuffmanNode {
                is_leaf: true,
                character: Some(*key),
                frequency: *val,
                left_child: None,
                right_child: None,
            },
            Reverse(*val),
        );
    }

    pq
}

pub fn build_huffman_tree(
    pq: &mut PriorityQueue<HuffmanNode, Reverse<u32>>,
) -> Result<HuffmanNode, Box<dyn std::error::Error>> {
    if pq.is_empty() {
        return Ok(HuffmanNode::default());
    }

    if pq.len() == 1 {
        let Some((child, _)) = pq.pop() else {
            return Err("Empty queue, cannot build HuffmanTree".into());
        };

        let parent = HuffmanNode {
            is_leaf: false,
            character: None,
            frequency: child.frequency,
            left_child: Some(Box::new(child)),
            right_child: None,
        };

        return Ok(parent);
    }

    while pq.len() >= 2 {
        let Some((left, _)) = pq.pop() else {
            break;
        };

        let Some((right, _)) = pq.pop() else {
            break;
        };

        let parent_freq = left.frequency + right.frequency;
        let parent: HuffmanNode = HuffmanNode {
            is_leaf: false,
            character: None,
            frequency: parent_freq,
            left_child: Some(Box::new(left)),
            right_child: Some(Box::new(right)),
        };

        pq.push(parent, Reverse(parent_freq));
    }

    let Some((root, _)) = pq.pop() else {
        return Err("Empty queue, cannot build HuffmanTree".into());
    };

    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::*;

    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn test_create_pq() {
        let encoder = Encoder::default();
        let freqs = encoder.get_frequencies(&chars("aaaa"));
        let pq = create_pq(&freqs);

        let mut expected: PriorityQueue<HuffmanNode, Reverse<u32>> = PriorityQueue::new();
        expected.push(
            HuffmanNode {
                is_leaf: true,
                character: Some('a'),
                frequency: 4,
                left_child: None,
                right_child: None,
            },
            Reverse(4),
        );

        assert_eq!(pq, expected);
    }

    #[test]
    fn test_build_huffman_tree() {
        let encoder = Encoder::default();
        let freqs = encoder.get_frequencies(&chars("aaaabb"));
        let mut pq = create_pq(&freqs);
        let output = build_huffman_tree(&mut pq).unwrap();

        let left_child = HuffmanNode {
            is_leaf: true,
            character: Some('b'),
            frequency: 2,
            left_child: None,
            right_child: None,
        };

        let right_child = HuffmanNode {
            is_leaf: true,
            character: Some('a'),
            frequency: 4,
            left_child: None,
            right_child: None,
        };

        let expected = HuffmanNode {
            is_leaf: false,
            character: None,
            frequency: right_child.frequency + left_child.frequency,
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
        };

        assert_eq!(output, expected);
    }
}
