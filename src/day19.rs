/*
  Day 19: Monster Messages

  Rules for message input are numbered and built upon each other. Rules do not self reference and the number of matches is finite.

  Example:
    0: 4 1 5
    1: 2 3 | 3 2
    2: 4 4 | 5 5
    3: 4 5 | 5 4
    4: "a"
    5: "b"

    Possible matches include: aaaabb, aaabab, abbabb, abbbab, aabaab, aabbbb, abaaab, or ababbb.

  Part 1
  Match the received messages with the rules (puzzle input). How many messages completely match rule 0?

  Part 2
  Make the following two changes to the rules to allow for a self reference:
    8: 42 | 42 8
    11: 42 31 | 42 11 31
  The list of possible matches is now hypothetically infinite.
  But, solving for a general case is unnecessary since we know exactly what rules changed and how they are used (especially ruels 42 and 31).
  After updating rules 8 and 11, how many messages completely match rule 0?
*/

use regex::Regex;
use std::collections::HashMap;

pub fn count_valid_messages(rules: &HashMap<i32, &str>, messages: &Vec<&str>) -> usize {
  let final_rules = evaluate_rules(rules);
  let rule0 = final_rules.get(&0).unwrap();
  println!("Final Rule: ^{}$", rule0);
  let re = Regex::new(&format!("^{}$", rule0)).unwrap();
  return messages.iter()
    .filter(|message| re.is_match(message))
    .count();
}

fn evaluate_rules(rules: &HashMap<i32, &str>) -> HashMap<i32, String> {
  let mut finished_rules = HashMap::new();
  for (&key, &value) in rules.iter() {
    if finished_rules.contains_key(&key) {
      continue;
    }
    let rule = evaluate_rule(key, value, rules, &mut finished_rules);
    finished_rules.insert(key, rule);
  }

  return finished_rules;
}

fn evaluate_rule(rule_no: i32, rule: &str, rules: &HashMap<i32, &str>, memo: &mut HashMap<i32, String>) -> String {
  if rule.contains("\"") {
    return rule.replacen("\"", "", 2);
  }

  let mut map_rule_key = |token: &str| { // must be &mut closure becuase memo is mutated
    let key = token.parse().unwrap();
    return
      if memo.contains_key(&key) {
        memo.get(&key).unwrap().to_string()
      } else {
        let r = evaluate_rule(key, rules.get(&key).unwrap(), rules, memo);
        memo.insert(key, r.clone()); // I'm still really bad at &str vs String, lifetimes, etc.
        r
      };
  };
  let self_ref_check = Regex::new(&format!(" {} | {}$", rule_no, rule_no)).unwrap();
  let self_reference = self_ref_check.is_match(rule);
  let mut rule_string = rule.split("|")
    .filter(|rule_pipe| !self_ref_check.is_match(rule_pipe))
    .map(|rule_pipe| rule_pipe.trim().split_whitespace()
      .map(&mut map_rule_key)
      .fold("".to_string(), |acc, r| if acc == "" { r } else {format!("({})({})", acc, r)})
    ).fold("".to_string(), |acc, r| if acc == "" { r } else {format!("({})|({})", acc, r)});
    
    
  // Rule contains a reference to itself, ex
  // 11: 42 31 | 42 11 31
  if self_reference {
    //Note: solving the general case did not seem to work, just "hard coding" the two special rules
    if rule_no == 8 {
      rule_string = format!("({})+", rule_string);
    }
    else if rule_no == 11 {
      let rule31 = memo.get(&31).unwrap();
      let rule42 = memo.get(&42).unwrap();
      println!("rule 31 {}", rule31);
      rule_string = format!("({})({})*({})", rule42, rule_string, rule31);
    }
    // let self_ref_rule = rule.split("|").last().unwrap();
    // let self_regex = self_ref_rule.split_whitespace()
    //   .map(|token| if token == &rule_no.to_string() { format!("({})+", rule_string) } else { map_rule_key(token) })
    //   .fold("".to_string(), |acc, r| if acc == "" { r } else {format!("({})({})", acc, r)});
    // rule_string = format!("({})|({})", rule_string, self_regex);
    // println!("Self ref rule string {}", rule_string);
  }
  return rule_string;
}


fn parse_rules(input: &str) -> HashMap<i32, &str> {
  let mut rules = HashMap::new();
  for line in input.split("\n") {
    let rule = line.trim().split(":").collect::<Vec<&str>>();
    rules.insert(rule[0].parse().unwrap(), rule.last().unwrap().trim());
  }
  return rules;
}

pub fn read_rules<'a>() -> HashMap<i32, &'a str> {
  let input = "90: 86 86
    122: 86 1 | 99 20
    116: 86 58 | 99 75
    20: 86 123
    62: 99 95 | 86 113
    81: 76 99 | 90 86
    106: 120 86 | 93 99
    73: 99 72 | 86 45
    117: 131 99 | 72 86
    92: 86 96 | 99 98
    13: 3 99 | 118 86
    56: 90 86 | 58 99
    85: 72 99 | 51 86
    51: 99 99 | 86 86
    59: 99 25 | 86 62
    65: 99 15 | 86 97
    112: 86 13 | 99 38
    46: 33 86 | 2 99
    10: 67 86 | 68 99
    33: 120 99 | 76 86
    38: 35 86 | 125 99
    26: 86 10 | 99 55
    1: 33 99 | 60 86
    8: 42
    16: 51 86 | 93 99
    107: 40 99 | 2 86
    40: 17 120
    34: 86 82 | 99 127
    88: 93 17
    2: 99 51 | 86 120
    32: 100 99 | 7 86
    113: 86 127 | 99 82
    14: 73 86 | 44 99
    25: 86 101 | 99 56
    130: 110 86 | 109 99
    19: 86 4 | 99 49
    30: 86 92 | 99 70
    27: 17 86 | 86 99
    94: 47 86 | 53 99
    115: 86 107 | 99 84
    15: 76 99 | 58 86
    58: 86 99
    105: 130 86 | 32 99
    71: 120 99 | 131 86
    12: 99 131 | 86 82
    60: 72 86 | 93 99
    84: 86 102 | 99 80
    44: 99 76 | 86 72
    125: 76 99 | 131 86
    18: 99 71 | 86 52
    129: 37 86 | 111 99
    102: 99 131 | 86 76
    66: 86 105 | 99 41
    99: \"a\"
    9: 99 18 | 86 65
    131: 17 99 | 99 86
    39: 76 99 | 93 86
    64: 115 99 | 114 86
    57: 86 48 | 99 94
    35: 72 86 | 51 99
    0: 8 11
    77: 86 83 | 99 106
    118: 72 86 | 75 99
    47: 99 103 | 86 85
    23: 99 27 | 86 76
    48: 119 99 | 78 86
    49: 86 51 | 99 45
    67: 86 120
    61: 86 72 | 99 127
    108: 72 99 | 72 86
    95: 86 58 | 99 90
    83: 86 27 | 99 131
    75: 86 99 | 99 99
    101: 51 99 | 27 86
    103: 90 99 | 90 86
    128: 86 69 | 99 33
    70: 99 14 | 86 19
    52: 127 86 | 90 99
    21: 86 24 | 99 59
    22: 86 63 | 99 12
    42: 79 86 | 66 99
    97: 51 17
    104: 86 33 | 99 28
    100: 99 16 | 86 39
    72: 99 86
    78: 86 43 | 99 50
    55: 86 6 | 99 34
    45: 99 99
    5: 86 46 | 99 77
    93: 99 99 | 99 86
    6: 131 99 | 51 86
    110: 71 86 | 28 99
    68: 90 86 | 27 99
    29: 87 86 | 122 99
    80: 86 93 | 99 131
    54: 120 86 | 75 99
    43: 93 86
    98: 99 103 | 86 117
    7: 101 86 | 88 99
    127: 17 86 | 99 99
    96: 86 12 | 99 61
    41: 99 5 | 86 112
    79: 86 57 | 99 21
    11: 42 31
    86: \"b\"
    111: 45 99 | 72 86
    63: 76 99 | 127 86
    124: 86 81 | 99 116
    28: 86 75 | 99 58
    82: 99 86 | 86 99
    121: 64 86 | 74 99
    87: 86 124 | 99 104
    74: 99 26 | 86 9
    31: 126 99 | 121 86
    50: 99 72
    119: 34 86 | 36 99
    36: 86 82 | 99 75
    91: 86 131
    3: 58 99 | 27 86
    114: 99 129 | 86 22
    24: 86 89 | 99 128
    53: 91 99 | 95 86
    126: 29 86 | 30 99
    109: 99 108 | 86 23
    17: 86 | 99
    76: 86 86 | 99 86
    120: 17 17
    89: 54 86 | 37 99
    4: 99 131 | 86 58
    69: 27 17
    37: 99 58
    123: 86 76 | 99 82";
  return parse_rules(input); 
}

pub fn read_messages<'a>() -> Vec<&'a str> {
  let input = "bbababbaabbaaabaaaabbabbbbbababbbababaaaabbaabaaaaaabaaaabbaabba
    aaabbbabbabbbbbbaabbabababaaaaabaaabaaaaabaaaabbbbabbabb
    babaabaabbabaaaaabbababb
    babbabbaababbaaaababbaabbbbbaabaabbbababaabbbabbbabababaabbabbabaabbbaababbbbbbb
    bababbbabaaabbaabbababab
    aaaaabaaaabbaabaaaaabbaa
    aabbbaaaaababbaaabaabbbbabbbaaaaabbaaaab
    baaaabaaaabbababbaaabbab
    ababababaaaaabaaaaabbaaa
    baabaababaaabbaabbbababb
    abbbabbbbbabaaabaabbabbb
    aaababbbabababbbaabbaaaabababbaa
    babbaaabbaaaabaababbbabb
    bbbabbbabbabaaaabaabaaab
    bbbbaaabaaaabbbaabbbbabb
    aaaaaaaaaaaaabaabaaaabbb
    babaabbababbabaababbaaba
    bbabaaaabaaabbaaaabbbbbbbabbabbbbaaaabab
    babaabbbbbbbbbaabbaaabaa
    baabbbabababbababaabbbaa
    baaaabaaaabaaaaaaaaaaaab
    aaabbbbbbbabbbabbbbbabbb
    bbabaaabaaababaabbbbbbba
    baaabaaabbabbaaaaaaababb
    abbabaabbaaaaaaaaabaabba
    aaaabababbbbbbaabababbab
    bbbbbbabababbaabaabbbabaaaabbabbaabbabbbbaababbbabaabbbbabbabaabababaabaaaaaabab
    bbbabbaaaababbabaabaabbbabbbabab
    bbbabbbababbbbaabbbbbbab
    baabaaaaabaabbbbaaabaaaabbbbbbab
    bbbbaaaaaabaaaaaabbbabaababbaabb
    bbbbbabaabaaaaababbaaabbbaababbbbbaaabaa
    aaabaabaaaabaaaaababbaaaaaababbbaaababab
    bbaaaababbbaaabbabbabbabbabbbbabbababaabbbabbabbabaaabbabaaaabba
    aababbbaaaabbbaabbbbababbabbaaaa
    abbbaabbabaaaaababaaaaaa
    babbabbabbaabababbabaaaaaabbbabbbabaabbbbabbbaabbaaaaaba
    aabbbaabaaabbabbbbaabaaabbbabbababbaababababbbba
    baabbabbbaabbbbbbbaaaaaababababaaabbbbabaabaabbaaaaabbab
    bbabaaabbbbbaaaababbabbb
    baabaabaaababbbabaaababa
    aaaabbbbabbbaabbababaaaa
    babbababaaaababababaabab
    bbbaabbbabbbabaabbbabbbaaabbaabbabbbbbbbaaaabaaabaaabbaabbbbabbb
    aabbaabbabbabbbabaaaabba
    abaabbaaaaabbabbaaabbbabbabbaaaa
    bababaabbaaabaaababbababbbbbbabbbaaabaab
    aaababbbaabbaabbbbaaabbaaababaaabaabbbaa
    abbaaababbaaababbaaaabba
    bbaababaaabbabaaababbaabbaabbbbaaabbaabbabaaabbabbabbbbbabbbaaabbbababbb
    bbaaabbbbbaaabbabaaaabba
    abbbabaabbaabaaaaaabbaaa
    bbbabaabbbbbbbaababbbbaaabbbabba
    abbabbaaaaaabbbbbbabaaabbaaaaaab
    bbaababaaaababaabaabbababbbbbaab
    aabbababaaabbbaaaaabbabbaabbbbbbbabbbabb
    aaabbbaaaababbabbababbbb
    aababbababaaabaabababbbb
    bbabbbabbbabbbabababbbba
    aaabaaaaabbbbbbbbaaaaaab
    babaabbbaaabbbabbaaaaaab
    bbaabbbaaabbababbbaaabaa
    aabbbaaaaaaaaaaaaaaaabaaaabbbbbabbabaaaaabbbbbab
    abaaaaabaabbbaaaabbbaaab
    abaabbabbabbbbbabbababaa
    baaabbaaaabbbabbaaaabaaa
    abbbbbaabbaaababbbbabaab
    aababbaaaabbbbbabaababaabababaabbabbbaba
    abaabaaabbabababababababbaabaaabbbaabaabaabbbababbbabbbabbabbbbbabababaabbabbaba
    aaaaaabaaaaabababbbabbabbbaabbabaababbbb
    babaaaababaaababaabbabba
    ababbaaaabaaabbbbababaaa
    baababbbabbbbaaabaababbbabbbbaababaaabaaaabbaabababababaaaaababa
    bbaabbababbabaabbbabbabb
    bbabbbbaababbaabbbbabbbbbabbabaaaaabbbababbbbaabaaaaaabbbababbbbbabbbabb
    baaaaaaababaaaabbbbbaaaaabaabbbbbaababbb
    aabbabaaabbbbbaabbbaabab
    ababbabbbbaabaaabababaabaaaabbab
    aaabbabaaaaabbbbabbbbaaa
    bbaaaaabbaaaabaaabbbaaaa
    abbbaaaaabbbabbbabbabaaa
    bbaaaabbaaaababababaababaaaabbbaaabaabaaaababbabbbaaaabaabaaabbaababbbaaabaabbaababbbaabaabaabbb
    bbaabbaaaababbababbaaabbbabbbababbbbbbaabbaabababbababaababbaababbbaaabbaabbbbab
    aababbabaaabaaaabbaabbaa
    babaaabbaaabaababbabbbbabbbaabaabbaabbbababbabaabababaabbbababbabbbbaababbaabaaa
    baabbabbbbaabbbaaabbaaab
    abaaaaabbbaababaabbaabab
    aaabbbbbaaababaaabababaabbaababb
    bbaaababbaabbbbababbbbbabbaabbabbbbbaaabbbabbabb
    bbabbbbaabbbbbbbbaaaabba
    aababbabaaabaababaabbabbabaaaabaaabbbabaaabbbabbbabaaaaa
    babbbbbbaabbbaabbabbbabb
    baabbbbaababbaabaaabbabb
    aaabaabaabbbaabababbbbaaaaaabaabbabaabbbabaaabbbaaaabababaababbbababbabaaaaaabab
    aabbabaabaaaaaaababbababaabaabba
    ababbababbaabbbaabaabaababbaabbbaaaabbaa
    aabbababaabbbbbbbaababaababbabba
    baabbabbbaaaaaaaaaaabaaa
    baabbbabaaabbabbbbbbbabb
    babaabaaaabaaaaabbbaaaab
    baaaaaaababbbbbaabbbbbab
    bbabbbabababbaabbaaababa
    bbbbabababaaaaababbabbbababbabaaabbaabababbababaaaaabbab
    aabbbbbababbabaaabababaaaaabaababbbabbab
    babbbbaabaabbbbabbbbbabb
    aaabbbabbabaabbbabaaababbaaabaab
    abbaaabbbbaaabbbbabbbbbbbbbbabaa
    bbabbabaaaabbabbbbbababababbaaabbabaaaaababaaaba
    bbaabaaaaabaaababaababbb
    baabbaaaaaababbababbaabaabababbbabaaabbbbbbaaabbbbababaababaabbbbbabaaababbaaaabbbaaaaaa
    aaabaaabbbbbaaaabbbbabba
    abbbaabbbabbbaaaaabaabab
    aaabbbaabbaabbabbaabbbbbbbabbaaaaaaabbbaaaaaabab
    aaaaaaaaaabbaabaaabbbbaabbaaaaabaaabbaababbbbbab
    bbbbababbabababbbbababba
    abaaaabaabbbbaaaabbbbbbbbaababbabbbbaabbaabbbbbabaaabbbaaaaaabab
    bbbbbbaabaabaaaaaaaaabbaaaaaaababbabbabaabbabbab
    abbbbbbbbaaaabaaaababbabbbbbaabb
    bababaabaababbbaabbaabab
    abaaaabaabaabbabaaabbbbbbbaaaaaa
    aaaaaabaababababbbaabbbabbaaaaba
    abaabaabbbababbababbaabb
    baaabbaabbbaabbbbabbabbb
    baababaabaaabaaaabbabbbabababbaabbbbbbba
    bbbaabbabbbabaabbaaabbaaaabababaaabbbbaaababbaab
    abaabbbbabbabbbbaaaaabab
    bbbabaababaabbbbbbbaaaab
    abbaaabaaaaaaabaabaaaabaabbaaabbabbabbbbaabaabab
    aabaaaaababbbaaaaaaababb
    aaabbbaaabaabbbabbbbabaa
    aaaaabaababaabbabaaaabaaabbaaaaa
    aaaabaabababbbbbaaaaaaaaaabbbbbbbbbbbbababbbbabbbaaabaab
    bbabbbabbaabbbbaaaabaabaabaaabbbaaaaaaababbbaaab
    aabaaabaaabbababbabababa
    babbbaaaaabbbaabbbabbababbbbbbbaabbababa
    abbaabaabaabbaaaabaaaaaaababbaab
    abaaaababbaabababaabbbababbabaabaaababbbbbbbabba
    bbaaaabababbabababbabaabbbaaabbbbbbabbaa
    aabbaabbaaabbbaabaaaaaab
    bbaabbabaaaabbbabbabbabb
    abbaaabbabbbaaaabbbbbaaabbabbabababbbbbbabaabbbbbabbaaaaabaababb
    ababbaabbabbbbbbbbabaaaabbbbababbaaabbbabaabaabb
    aaabbabaaabbbabaaaaabaabaaababbaabbbabbababbbbabaabbabbb
    abbabbaaabababababbbabaaaaaabaabbabaabab
    babaabaaababbaaaaabbaaab
    aaaaaababababaababbaaaab
    ababbabaaababbaaabbbaababaaaaabb
    bbaaabbabaaaabaabbabaaaaaaaababbbbbabaaa
    bababbbabbaabbabbbbbabba
    bbaaabbaaabbbbaabababaabbabbaaaa
    aaaabaabaaaaabbaababbaabababbaaa
    ababbabaabbabbbbbbaababb
    aabbbbbbbaaabbbababababa
    bababbbaabbabbaabaaaabab
    bbaaabbbaabbbbbaaabaabba
    aabbbbaaababbbabbbaabbbb
    baaaabaaabaaabbbababbbabaabbbabbaabbbaabbbbbbbabaaaabbaababbaabaaababbbb
    babbbbabbbbababbabaaabba
    babbbbaaababbbbbbabbbaaabaabbaab
    bbbbbbaaaabbbabbaaabaaaabbbabaaaaabaabab
    babbbbbabbabaaaaaaaaabbb
    ababbbabbabaabbaaaabbabbaaaabbaa
    bbbabbbabababbbabbaaabbbbbbbaaba
    abbabbaaabaaababbaaaaabb
    bbababbabababaaaaababbbbbabbbaabababbaabaabbbaababbaaaba
    babbababaabbbaaaabbaaaab
    abbaaabbbaaaaaaabbababab
    bbbabbbabbabbabababbbbab
    babbbaaabababaabaababbbababbbbbaabbbbaba
    bbbababaabaabaabababbbbbabaaaababbabbbabbbaaabaababbaaaaabaaaabbaaaabbab
    babababbabbabbbaaaaabbab
    baabbabbabaabbaaababbbbb
    aabbabaaaaaaabbaabbabbbaababbaabababbbbbbabbbababbbaaaba
    aababababbbabbbaabbaaaaa
    aabababaaabaaabaaabbaababbbabbaabbaaababbbbbbbababababbaabbbbbab
    abbbaabbbbabbbbaabbabbab
    aaaabbbbaabbaabbbabaaaaa
    aababbababaabbbaaabaabba
    baaaaabaaaaaabbababaabbbabaaaabbabaabbbaabaaabbaabaababaaaabbaaa
    ababbaaabaababaabbbbbababaaabbbb
    babbabababbbaaaabbbaabaa
    baabbabbbaabbabaaabbbbaabbbaabbaabbabaababaaabaabbaababbbaaaabbb
    abaabbbbaaaabaabbbbbbbba
    baabbabaaaababaaabbaabbbbbbbbbbb
    abbaaaabababbbbabababaabbbaaabbaababaabbbaaaaaaaaabaaaab
    bbbabbabaaaabbababbaabababaaaabaaabbaaaabbbababa
    bbbababababbbbbbababbbba
    bbbbbbaaaaabbbaabaababab
    aaaabbbaabbaaababaaababa
    bbaaabbabbabaabaaaabaaaaabaaabbabaaababb
    ababbabaabaabbaababaaaaa
    bbbaabbaaaababbaabbbbabaaabaaaab
    abbbabbbbbbabbaaabbaabbbababbbbaaabababb
    bbbbbaaabaaabaabaabbaaab
    bbaabbababaaababaaabbbabaaaababaabbaaababaabaaab
    abbbabbbaabbbbbabbababbb
    bbbaabbabbabbbabbaaaabba
    bbabaabaababbbaababababaaabaabababaaaaaabaaababb
    babaabaaaaaaabaaaabaabba
    babbbbbaaabbbaaaaabbababbaababba
    aababbaaababbbbbbabbabba
    bbabbbabaabbaabbbbbbabaa
    ababbababbbababaababbbaa
    aaabbbaabbabaabaababbbba
    bbaabbabbbabbaaabaabbbababaaabaabbbbbabbbbaaabaaababaaba
    baababaabbaaabbabaaababb
    abbaabbbababababaaababbbbaabaabb
    baabbbbbaaabaababaababba
    babaabaababaabbbbaabaabaaaababaababbabba
    bbaaaaabaababbaaaabbaabaaabbabbbbababbaa
    abbaaabbaaabbbbaaabbbbaaabbaaaabaabbaaabbaababbbabbabbabaaaababb
    aaabbaabababaaaabaabaaab
    bbaabbaabbbabaaabbabbbbbbbbabaabbabaaabababbbbabbaabbabbbbbabbaa
    bbbabbbbbbabbbaabaababab
    bbabbbbabbaabbbababbabba
    aaabbbaabaabaababaabbabababbaaaabbbbbbba
    babbaabababbabababbbbbbaababaabbbaabbbbababaaaabbbaaaaababbabaaabbbbaaaaabaaababbbbaabba
    babbbabbbaaaaaaabbaabbbbabbbbabbaaabababaabbbbab
    bbaaaaababaaabbaabbbaaaaaababbabbbaabaabaaabaaabbabaabab
    baabbbbbabbaaababbbbaaabbbbbbaabbbbbbaabbaaaaababbaaaaaaaabbabba
    bbabbbaaabaaabbbabbbabaaaababbaaaabbbaabababbbbabbababaaaaaababb
    baaabaaaaabbababbbabbbbaabaabbbaaabbbaabbaaabbabbaaabaab
    aaaaaabaaaaabbbabbbaabaa
    bbaaabbbbabaaaabbabaaaabaabaaaaababaababaaaabaaaaababbbb
    babbaaababbbaabaabbbbaab
    aaabbaababbbbbabaaabbaaaaabbbbbaabaabaababababababbabbbb
    babababbbaabaaaabaaabbbb
    babbabaaaaabbbabbbbbbbaabbbaaaaa
    aabbabaabaaabbbaaaaaabab
    bbbabbaaabbbbabbbababbabaaabababaababaab
    bbabaaaabaabbabbbbbabaabaaababbb
    baababbbbaaaabbabbaabbbbbbbbbaaabbbbaaba
    aabbaababaaaabaabaabbaba
    bbbabbaababaabaaabbbaaaaabaaaabaaabaaababababaaabbababbbbbbbabba
    bbabaabaabaaabaabbabbaab
    aabaaaaaaaababbbaaababbbbbbaabab
    baaaabaaabbbabbbabaaababbaabbaaa
    baabbbbabababbbaaababbbabbabbbbabbaabbbaabbabababaaabababaabaabbbabbabba
    baabaabaabababaabaaaabbb
    abbaabbbbabaabaaababbababaabbabbbabbbbabbbbbaabababbaabb
    abbbabbbbbbababaabbbabbabbbbbaab
    abaaabbbbbbaabbbaabbaaab
    ababbbabbbabbbaaabaaabaababababa
    bbbbbbaaabbbaaaaaaabbabaabbbbaabbbbaaaab
    bababbbbbabbbbbbbaabbbbbbbababbbbabbabbbabbbbaabbaaaabba
    baaabbaaaaaaaababaaabbbaabaabaaabaabababaaabbaaaabbaabaa
    aabbaabbbbbabaababaabaaaababbaaabbabbababbbbaabb
    abbbbbaabbbabbbbbbbbaaba
    aaabbabbbbbabbbababaaabb
    abaabaaaaabbaaaababbbaab
    aaaabaabbbbbaabababaaaaaaaaaaabb
    bbbababaabbbabaabaababba
    abbbaabbabaaababbbbbbabb
    babbabbbaaaaabbbaababbabbaabbbbbbaaabbaaaabbbbbaabaababbbbaaaaba
    aaaaaababbbababababaabab
    bbaabaabbbaaaabbabbababaaaaaababaabaaabb
    aabbabaababaaaababbbabab
    ababbbababaabbbbbaaaabbbbbbbbaababbbbaaabbabbbaaabbbabbbbaaaabbbaaaaaaaaababaaab
    bbbabaabaaaaabbaaaabaabb
    babbaaabbaabaababbbbaaba
    bbabaabababbbbbaabaabaaabbaaabbabbbabaaa
    abbabaabbaaaabaabbbaaaba
    aabbabaabaaabbbaaaabbbabbabababbbbbaabbb
    aabbbbbbabababababbbbbab
    abbabbbabbabaababaaabbbb
    baaabbaaababbaaabaabaabb
    bbbabbbaaaabaaaaabaaababaaaabbbbaabbaaaaabbbabab
    aabbbabaaaababaabaaaaaba
    abbbabbbababbabbbbaabbaa
    abbbbbabaabababbbabababa
    bbaabaabaababbaabababaaa
    bbabaaabaababbbaaababbbaaaababba
    abbaabaababbababaaaabbbaabbabaababbbaabaaaabaaabaabbbbab
    babbbaaabbabbbaababbbbaaaabaababaaaabaaa
    babababbabaaaabaaabbbabaaabbbabababaaaabbbabbaab
    aabbbaababbaaabaababbaababaaabbbaaaaaabaababbaaaaabaaabbbbababaa
    abbbabbbaaabbbbaaabaabab
    abaabbaaaabbaabbbbbaaaaa
    bbabaaabaaaabbbbaabbbaaaaabbbbbbabbbbbbabbbabaaa
    bbbabbaababbbbbabbaaaaabbabaabaaababaababaaaaabb
    abbbabbbababbaaabbabaaaaabbabbbabbabaabbbaaaaabbabbabaaabbbbbbbb
    bababbbaaaabaabaabbababb
    abbbbbbbabababaaabbaababbabaaababbaaaaaa
    abbaabbbabaaaaabaaaaabbaabaaaaabababbbababbbabab
    abbabbbbbbabaaaabbbabbbabbbbaaba
    abbaabbbbaabaababaababba
    bbabaaababaabbbaaaaaabbb
    baabbabaaaaabbabaabbbbabbabbabbabbaaaaaa
    babbbbbbababbababbabbbabaabbaaab
    aaabaaabbbbabbbbaabbabbbbaaaaaabaabbbaabbaababaa
    baabbbbbaaabaababababbab
    babbaaabaaaabaababbabbaaaabaabaa
    abaabbbbbbabaaaaaabaabab
    aaaaabbaabababaabbbbbababbbababbbbaababb
    abababababaabaaabbaaababbabbabbb
    abaaaaabbbbbbabaaaaabbaa
    aaaabbbbbbbbababbbabaabb
    bbabbabaabaaabaaaaababab
    aaabbbbbabbabbbbbabaababbbaabaababbababb
    bbabaabaabbabbaaaabaabba
    abbabaaababbaabaaaaabbbabbaaaaabbbaabbabbababaaabaabbaabbbbaaabaaaabbbaabbbbabba
    ababbbabaaabbabbaaabbabaaabaabba
    bbbabbbbbabbbbbbabaaabaaaaababbbbababaaa
    babbabaaabbaaabbabbabaaa
    babbbbaaabbbbbaaabaaabaabbbbababbaaababb
    abbabbbbabababbbbbabbaaabaaababb
    babbababbbaababaabbabbab
    bbaaababbbbbaaababbbbabb
    baaaabaababaabbbabaabbbaaabbbaabaaaabbab
    abbaabbbbbaaaaabbbaabbabbaabbaab
    aaaaaababbaababaaabaaabababbbabb
    baaabbaabbbababaaababbbaaaaaabba
    bbbbabaaaaabaaaaaaababbaaaababbbbbaabaaaaabaaaabaaaabbaabbbbbbbb
    abaaabbbbaaabaaababbabba
    aaaaabaaaaabbbaabbabaaaaabababaaabbbbbaabbaaaaba
    ababbabababbbaaaaaaaabbb
    aaababaaabaabbaababbabbb
    ababababaabbbabbbaaabaab
    baabbbabbaaabaaabaaabbbabbabbabb
    bbbabbaabbabbababaabbbbaabaabbabaabaaabbbaaabbbb
    aaabbbabaaabbbbbabbabbbbbbbaaaaa
    aaabbababbaaababaaaabbaa
    bbbabbaaababbbabaaabaabb
    baababbabbbabaaababaaababababbaaaababaabbbbaaaabbabaaababbbabaababbabbbbbaaabbab
    bbbabbaababaabbabbbbbaaa
    abababaabaaaabaaabababbbbaababbbbbabbaaaaaabbaabbbbabababbabaabb
    babbbababaaabbbabaababbbaaabababaaaababbbabbaabbabaabbba
    bbbabbaabaabbbbaaaaaabaabaaabbab
    bbbabbbbaabbaabaaaaabbbbbaabbabbabbaabaababbaabbaaabbaaa
    aaababbbbaaaabaaabbabbbbabbbbaba
    aaaaabbababaabbabaaaaaaaaaaaaababbbbbbaaababbaaaaabaabab
    babbababbaaabbabbabbaaaabbaabaabbaaabbab";
  return parse_messages(input);
}

fn parse_messages(input: &str) -> Vec<&str> {
  return input.split("\n")
    .map(|s| s.trim())
    .collect();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_rules() {
    let rules = read_rules();
    assert_eq!(&"130 86 | 32 99", rules.get(&105).unwrap());
    assert_eq!(&"42", rules.get(&8).unwrap());
  }

  #[test]
  fn test_evaluate_rule() {
    let input = "0: 1 2
      1: \"a\"
      2: 1 3 | 3 1
      3: \"b\"";
    let rules = parse_rules(input);
    let eval = evaluate_rules(&rules);
    assert_eq!("b", eval.get(&3).unwrap());
    assert_eq!("(a)(((a)(b))|((b)(a)))", eval.get(&0).unwrap());
  }

  #[test]
  fn count_messages() {
    let rule_input = "0: 4 1 5
      1: 2 3 | 3 2
      2: 4 4 | 5 5
      3: 4 5 | 5 4
      4: \"a\"
      5: \"b\"";
    let message_input = "ababbb
      bababa
      abbbab
      aaabbb
      aaaabbb";
    let rules = parse_rules(rule_input);
    let messages = parse_messages(message_input);
    assert_eq!(2, count_valid_messages(&rules, &messages));
  }

  #[test]
  fn count_messages_2() {
    let rule_input = "42: 9 14 | 10 1
      9: 14 27 | 1 26
      10: 23 14 | 28 1
      1: \"a\"
      11: 42 31
      5: 1 14 | 15 1
      19: 14 1 | 14 14
      12: 24 14 | 19 1
      16: 15 1 | 14 14
      31: 14 17 | 1 13
      6: 14 14 | 1 14
      2: 1 24 | 14 4
      0: 8 11
      13: 14 3 | 1 12
      15: 1 | 14
      17: 14 2 | 1 7
      23: 25 1 | 22 14
      28: 16 1
      4: 1 1
      20: 14 14 | 1 15
      3: 5 14 | 16 1
      27: 1 6 | 14 18
      14: \"b\"
      21: 14 1 | 1 14
      25: 1 1 | 1 14
      22: 14 14
      8: 42
      26: 14 22 | 1 20
      18: 15 15
      7: 14 5 | 1 21
      24: 14 1";

    let message_input = "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
      bbabbbbaabaabba
      babbbbaabbbbbabbbbbbaabaaabaaa
      aaabbbbbbaaaabaababaabababbabaaabbababababaaa
      bbbbbbbaaaabbbbaaabbabaaa
      bbbababbbbaaaaaaaabbababaaababaabab
      ababaaaaaabaaab
      ababaaaaabbbaba
      baabbaaaabbaaaababbaababb
      abbbbabbbbaaaababbbbbbaaaababb
      aaaaabbaabaaaaababaa
      aaaabbaaaabbaaa
      aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
      babaaabbbaaabaababbaabababaaab
      aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
    let rules = parse_rules(rule_input);
    let messages = parse_messages(message_input);
    assert_eq!(3, count_valid_messages(&rules, &messages));
  }

  #[test]
  fn self_referencing_rule() {
    let rule_input = "42: 9 14 | 10 1
      9: 14 27 | 1 26
      10: 23 14 | 28 1
      1: \"a\"
      11: 42 31 | 42 11 31
      5: 1 14 | 15 1
      19: 14 1 | 14 14
      12: 24 14 | 19 1
      16: 15 1 | 14 14
      31: 14 17 | 1 13
      6: 14 14 | 1 14
      2: 1 24 | 14 4
      0: 8 11
      13: 14 3 | 1 12
      15: 1 | 14
      17: 14 2 | 1 7
      23: 25 1 | 22 14
      28: 16 1
      4: 1 1
      20: 14 14 | 1 15
      3: 5 14 | 16 1
      27: 1 6 | 14 18
      14: \"b\"
      21: 14 1 | 1 14
      25: 1 1 | 1 14
      22: 14 14
      8: 42 | 42 8
      26: 14 22 | 1 20
      18: 15 15
      7: 14 5 | 1 21
      24: 14 1";

    let message_input = "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
      bbabbbbaabaabba
      babbbbaabbbbbabbbbbbaabaaabaaa
      aaabbbbbbaaaabaababaabababbabaaabbababababaaa
      bbbbbbbaaaabbbbaaabbabaaa
      bbbababbbbaaaaaaaabbababaaababaabab
      ababaaaaaabaaab
      ababaaaaabbbaba
      baabbaaaabbaaaababbaababb
      abbbbabbbbaaaababbbbbbaaaababb
      aaaaabbaabaaaaababaa
      aaaabbaaaabbaaa
      aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
      babaaabbbaaabaababbaabababaaab
      aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
    let rules = parse_rules(rule_input);
    let messages = parse_messages(message_input);
    assert_eq!(12, count_valid_messages(&rules, &messages));
  }
}