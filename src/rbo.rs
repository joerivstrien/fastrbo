use std::collections::HashSet;
use std::convert::TryInto;

// input data structures
pub struct Config {
    search_depth: Option<usize>,
    p: f64,
}

impl Config {
    pub fn new(search_depth: Option<usize>, p: f64) -> Config {
        // check that p is between 0 and 1

        Config {
            search_depth: search_depth,
            p: p,
        }
    }
}

pub struct RankedList {
    elements: Vec<String>,
}

impl RankedList {
    pub fn new(elements: Vec<String>) -> Result<RankedList, &'static str> {
        // check that there are no duplicate elements
        if contains_dups(&elements) == true {
            return Err("ranked list contains duplicate ids");
        }

        Ok(RankedList { elements })
    }
}

fn contains_dups(list: &[String]) -> bool {
    let mut id_set: HashSet<&String> = HashSet::new();
    for val in list.iter() {
        id_set.insert(&val);
    }
    if id_set.len() == list.len() {
        false
    } else {
        true
    }
}

pub fn rbo(conf: Config, left: RankedList, right: RankedList) -> f64 {
    // edge cases
    if left.elements.len() == 0 && right.elements.len() == 0 {
        // both empty: 1
        return 1.0;
    } else if left.elements.len() == 0 || right.elements.len() == 0 {
        // one empty: 0
        return 0.0;
    };

    // handle search depth (if none length of shortest list)
    // if longer than shortest list, length of shortest list
    let search_depth: usize;
    let len_shortest;
    let len_right = &right.elements.len();
    let len_left = &left.elements.len();
    if len_left < len_right{
        len_shortest = len_left;
    } else {
        len_shortest = len_right;
    }
    match conf.search_depth {
        None => {
            search_depth = *len_shortest;
        }
        Some(i) => {
            if i > *len_shortest {
                search_depth = *len_shortest;
            } else {
                search_depth = i
            }

        }
    }

    // Initialize A(agreement), AO (average overlap)
    // the only information we need is current and previous, not all ranks
    // so we can store on the stack
    let mut a: [f64;2] = [0.0,0.0];
    let mut ao: [f64;2] = [0.0,0.0];
        
    // set weights
    let mut weights = Vec::new();
    for i in 0..search_depth {
        weights.push(1.0*(1.0-conf.p)*conf.p.powi(i.try_into().unwrap()))
    }
    
    // create running sets
    let mut left_running: HashSet<&String> = HashSet::new();
    let mut right_running: HashSet<&String> = HashSet::new();

    // initial state for first rank before looping
    left_running.insert(&left.elements[0]);
    right_running.insert(&right.elements[0]);
    
    // set initial a and ao values, overlap and average overlap
    if left.elements[0] == right.elements[0] {
        a[0] = 1.0;
        ao[0] = weights[0];
    } else {
        a[0] = 0.0;
        ao[0] = 0.0;
    }

    // loop over consecutive ranks
    for i in 1..search_depth {

        // count number of matches for current new rank
        let mut count = 0;
        if right_running.contains(&left.elements[i]) {
            count += 1
        }
        if left_running.contains(&right.elements[i]) {
            count += 1
        }
        if left.elements[i] == right.elements[i] {
            count += 1
        }

        // update agreement
        a[1] = 1.0*((a[0]*i as f64)+count as f64)/(i as f64+1.0);
        // update overlap
        if conf.p==1.0 {
            ao[1] = ((ao[0]*i as f64)+a[1])/(i as f64 + 1.0);
        } else {
            ao[1] = ao[0] + weights[i]*a[1];
        }
        
        // add current rank to the running sets
        left_running.insert(&left.elements[i]);
        right_running.insert(&right.elements[i]);

        // move a[1] and ap[1] to 0'th position
        a[0] = a[1];
        ao[0] = ao[1];
        
    }
    ao[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_dup() {
        let testvec = [
            "hallo".to_string(),
            "test".to_string(),
            "hallo".to_string(),
            "daar".to_string(),
        ];

        let result = contains_dups(&testvec);
        assert!(result)
    }

    #[test]
    #[should_panic]
    fn create_non_uniq_ranked_list() {
        let testvec = vec![
            "hallo".to_string(),
            "test".to_string(),
            "hallo".to_string(),
            "daar".to_string(),
        ];
        let rlist = RankedList::new(testvec).unwrap();
        println!("{:?}", rlist.elements);
    }

    #[test]
    fn both_lists_empty() {
        let left = RankedList::new(Vec::new()).unwrap();
        let right = RankedList::new(Vec::new()).unwrap();
        let conf = Config {
            search_depth: Some(5),
            p: 0.9,
        };
        let res = rbo(conf,left,right);
        assert_eq!(res,1.0)

    }

    #[test]
    fn one_list_empty() {
        let left = RankedList::new(vec![
            "test".to_string(),
            "hallo".to_string(),
            "daar".to_string(),
        ])
        .unwrap();
        let right = RankedList::new(Vec::new()).unwrap();
        let conf = Config {
            search_depth: Some(5),
            p: 0.9,
        };
        let res = rbo(conf, left, right);
        assert_eq!(res, 0.0);
    }

    #[test]
    fn search_depth() {
        let right = RankedList::new(vec![
            "test".to_string(),
            "hallo".to_string(),
            "daar".to_string(),
            "waar".to_string(),
            "raar".to_string(),
        ])
        .unwrap();
        let left = RankedList::new(vec![
            "test".to_string(),
            "hallo".to_string(),
            "daar".to_string(),
            "waar".to_string(),
            "raar".to_string(),
        ])
        .unwrap();

        let conf = Config {
            search_depth: None,
            p: 0.9
        };
        let res = rbo(conf,left,right);
        // expected value from python implementation
        let exp: f64 = 0.4095099999999999;
        assert_eq!(res,exp);

    }
}
