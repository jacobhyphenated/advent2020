/*
  Day 14: Docking Data

  The docking program (puzzle input) can either update a bitmask or write a value to memory.
  The values are 36 bit integers. The bitmask is applied to values immediately before they are written to memory.

  Part 1
  The bitmask of 0 or 1 overwrites the corresponding bit of the value, while an X leaves the bit unchanged.
  Example:
    mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11

    value:  000000000000000000000000000000001011  (decimal 11)
    mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    result: 000000000000000000000000000001001001  (decimal 73)
  What is the sum of all values left in memory after it completes?

  Part 2
  Rather than modifying the bit, the mask acts as a memory address decoder.
    if the bitmask bit is 0, the corresponding bit in the value is unchanged
    if the bitmask bit is 1, the corresponding bit in the value is overwritten with a 1
    if the bitmask bit is an X, the corresponding bit in the value is floating
    A floating bit will take on all possible values, potentially causing many different memory addresses to be written to at once.
  Example:
    mask = 000000000000000000000000000000X1001X
    mem[42] = 100

    address: 000000000000000000000000000000101010  (decimal 42)
    mask:    000000000000000000000000000000X1001X
    result:  000000000000000000000000000000X1101X

    After applying the mask, two bits are floating leaving 4 possible memory addresses:
    000000000000000000000000000000011010  (decimal 26)
    000000000000000000000000000000011011  (decimal 27)
    000000000000000000000000000000111010  (decimal 58)
    000000000000000000000000000000111011  (decimal 59)
    The value 100 is written to each of these 4 addresses.
  What is the sum of all values left in memory after it completes?
*/

use std::i64;
use regex::Regex;
use std::collections::HashMap;

pub fn add_mem(instructions: &Vec<String>) -> i64 {
  let mem_regex = Regex::new(r"mem\[([0-9]+)\]").unwrap();
  let mut mem_map = HashMap::new();
  let mut current_mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
  for instruction in instructions {
    if instruction.contains("mask =") {
      current_mask = instruction.split("=").last().unwrap().trim();
    } else {
      let mem = mem_regex.captures(instruction).unwrap().get(1).map_or("", |m| m.as_str());
      let val: u64 = instruction.split("=").last().unwrap().trim().parse().unwrap();

      let base_2 = pad_36(&format_radix(val, 2));
      mem_map.insert(mem, apply_mask(current_mask, &base_2));
    }
  }
  return mem_map.values()
    .map(|val| i64::from_str_radix(val, 2).unwrap())
    .sum();
}

pub fn add_mem_v2(instructions: &Vec<String>) -> u64 {
  let mem_regex = Regex::new(r"mem\[([0-9]+)\]").unwrap();
  let mut mem_map = HashMap::new();
  let mut current_mask = "000000000000000000000000000000000000";
  for instruction in instructions {
    if instruction.contains("mask =") {
      current_mask = instruction.split("=").last().unwrap().trim();
    } else {
      let mem: u64 = mem_regex.captures(instruction).unwrap()
        .get(1).map_or("", |m| m.as_str())
        .parse().unwrap();
      let val: u64 = instruction.split("=").last().unwrap().trim().parse().unwrap();

      let all_mem = apply_mem_mask(current_mask, &pad_36(&format_radix(mem, 2)));
      for mem_addr in all_mem {
        mem_map.insert(mem_addr, val);
      }
    }
  }
  return mem_map.values().sum();
}

fn apply_mem_mask(mask: &str, value: &str) -> Vec<i64> {
  let mut all_mem = vec![Vec::new()];
  let mut digits = value.chars();
  for m in mask.chars() {
    let digit = digits.next().unwrap();
    for i in 0..all_mem.len() {
      let current_mem = &mut all_mem[i];
      if m == '0' {
        current_mem.push(digit);
      } else if m == '1' {
        current_mem.push('1');
      } else {
        let mut new_mem = current_mem.clone();
        current_mem.push('0');
        new_mem.push('1');
        all_mem.push(new_mem);
      }
    }
  }
  return all_mem.iter()
    .map(|m| m.iter().collect::<String>())
    .map(|s| i64::from_str_radix(&s, 2).unwrap())
    .collect();
}

fn apply_mask(mask: &str, value: &str) -> String {
  let mut mask_applied = Vec::new();
  let mut digits = value.chars();
  for m in mask.chars() {
    let digit = digits.next().unwrap();
    if m == 'X' {
      mask_applied.push(digit);
    } else {
      mask_applied.push(m);
    }
  }
  return mask_applied.iter().collect();
}

pub fn parse_input(input: &str) -> Vec<String> {
  let mut vec = Vec::new();
  for line in input.split("\n"){
    vec.push(line.trim().to_string());
  }
  return vec;
}

// https://stackoverflow.com/a/50278316/1856960
fn format_radix(mut x: u64, radix: u32) -> String {
  let mut result = vec![];

  loop {
    let m = (x % radix as u64) as u32;
    x = x / radix as u64;

    // will panic if you use a bad radix (< 2 or > 36).
    result.push(std::char::from_digit(m, radix).unwrap());
    if x == 0 {
      break;
    }
  }
  result.into_iter().rev().collect::<String>()
}

fn pad_36(input: &str) -> String {
  return format!("{:0>36}", input);
}

pub fn read_input() -> String {
  return "mask = 000001001011XX1XX100X0001011X0001101
    mem[54977] = 194579
    mem[29691] = 71157948
    mem[27205] = 122030256
    mem[43160] = 91267
    mem[45028] = 254793847
    mem[27137] = 1696
    mask = 1000011X10X10X1X1100101X00X011010001
    mem[20727] = 25071621
    mem[37927] = 626522009
    mem[4815] = 119068316
    mask = 00X001000000110X0X00110X10X10XXX0100
    mem[10400] = 6188570
    mem[27399] = 117579
    mem[16713] = 1592
    mask = 000101001X00110101101X001X001010X1X1
    mem[62042] = 11879
    mem[6190] = 317
    mem[2353] = 4338928
    mem[31175] = 586869909
    mem[26763] = 115919
    mem[58433] = 948899805
    mask = 0XX00X00110X11100111100X1X11X01X0010
    mem[27399] = 41077
    mem[45109] = 801841
    mem[23004] = 101713
    mem[2083] = 27746
    mask = 000X1100X00111X011X000010100X01XX001
    mem[51416] = 38881
    mem[52268] = 11555
    mem[9105] = 1895
    mask = 0000X00010XX011111000X0011X001011100
    mem[41414] = 279255497
    mem[41936] = 13975
    mem[12019] = 178049790
    mem[37328] = 837
    mem[34264] = 28896171
    mem[10161] = 2338
    mask = 00X00X00X100011XX1X0X010010000100010
    mem[52076] = 890751
    mem[11042] = 3792
    mask = 0XX0010000000101000X1000XX1011100110
    mem[18656] = 829925824
    mem[39076] = 509523
    mem[38827] = 6246
    mem[30529] = 27164257
    mem[38415] = 627755
    mask = X00001X11110111XX11000X11XX00X001001
    mem[25305] = 625
    mem[43178] = 152
    mem[35066] = 21090
    mem[8985] = 300087
    mem[48903] = 43704
    mask = 0001010010X01X1101XX10X110011X00010X
    mem[32378] = 2225
    mem[26042] = 69485948
    mem[49589] = 128819
    mem[9137] = 1482
    mem[1877] = 964153
    mem[7605] = 15865195
    mask = 000001X010X111100X001000100000X11100
    mem[14311] = 34388663
    mem[55974] = 20365
    mem[40457] = 13694402
    mem[17283] = 6362
    mask = 000001011XX0X111X0011100000000001010
    mem[6190] = 160501619
    mem[63533] = 170553877
    mem[4200] = 568690145
    mem[50785] = 2950313
    mem[22906] = 99588
    mem[33127] = 12019
    mem[15513] = 2884368
    mask = 100101X01X001101X1X0100000X00001110X
    mem[12489] = 27222771
    mem[49637] = 3293
    mem[42893] = 3050930
    mask = 0000X110X01111100000X010000X10001011
    mem[61719] = 182970
    mem[10774] = 3146
    mask = X000X00X1011011111000010X0X0101X1X0X
    mem[8693] = 29356
    mem[5498] = 3763
    mem[6468] = 4375
    mem[48837] = 741177
    mem[17108] = 39010
    mem[37452] = 6800613
    mask = 0100110X110X111001XX001X0010X11XX011
    mem[60650] = 9977
    mem[9137] = 3521
    mem[41035] = 5961
    mem[31775] = 15148399
    mem[59554] = 467182665
    mem[22790] = 25254
    mem[31588] = 126738299
    mask = 0XX001X01010X101X010010XX111X1001X01
    mem[61947] = 40390065
    mem[49445] = 304844
    mem[8929] = 102044152
    mem[33132] = 4864185
    mask = 01X001001101XX1001X100X11X0000010100
    mem[23919] = 261251
    mem[43749] = 3455
    mask = 00X00X001000XX1X0100X1001001001XX100
    mem[42199] = 436
    mem[61293] = 243299307
    mem[4] = 126598
    mem[6567] = 3430
    mem[47441] = 252938862
    mem[12399] = 74446
    mem[56225] = 15423
    mask = 0000X00X10X10110110000X110X0XX001X00
    mem[15993] = 101940097
    mem[38374] = 3770
    mem[54215] = 91086147
    mem[41414] = 1939
    mem[12528] = 2251037
    mask = 000001X010111110XXXX00X010101X0X1111
    mem[24941] = 144
    mem[10771] = 133581619
    mem[56280] = 54
    mask = 00000XXX1011X1100100001010X00111111X
    mem[46131] = 166
    mem[27421] = 297794521
    mem[58919] = 963
    mem[27836] = 404
    mem[18894] = 2052778
    mem[2181] = 559420
    mask = 00010100XX0011X101X00XX00001100X0101
    mem[61468] = 232744
    mem[22786] = 225590
    mem[34992] = 1686
    mem[46982] = 301
    mem[25985] = 136773808
    mem[9237] = 8105
    mask = 0001X1001000110XXXXXX00X00000110X101
    mem[41414] = 783022
    mem[4425] = 2493963
    mem[3855] = 70168785
    mem[5364] = 65108
    mem[49761] = 119528303
    mem[4044] = 47817
    mask = X0000000101X011XX100001X00001X111XXX
    mem[41155] = 18801
    mem[18910] = 176109
    mem[10369] = 13090894
    mem[45605] = 4100336
    mem[46751] = 2228
    mask = 000001X111000100X1000011010011X0000X
    mem[22385] = 2137243
    mem[34992] = 13385188
    mem[17612] = 11274427
    mask = 00X0010010000011X10X0X00100000100111
    mem[13014] = 17899233
    mem[26405] = 187154132
    mem[6562] = 50815
    mem[22561] = 6259252
    mem[64082] = 5392
    mask = 00000100X000XXX10X0010X010X0X0100100
    mem[55018] = 20219327
    mem[37443] = 21057605
    mem[35073] = 30620676
    mem[47330] = 445
    mem[24868] = 1379
    mask = 000101000000110101X0X0X01110011X0000
    mem[28383] = 2533101
    mem[30051] = 64
    mem[58425] = 20047
    mem[59003] = 241
    mem[28034] = 1212
    mem[36860] = 29935270
    mem[31948] = 162180484
    mask = X00101XX10X0X10100100X0000X010000111
    mem[58919] = 5
    mem[4393] = 1929987
    mem[15637] = 98094246
    mem[24486] = 91105079
    mem[33928] = 5323
    mem[15009] = 2237
    mask = 0X00X1001001X1100100XX1X10000XX00101
    mem[28249] = 587029
    mem[31407] = 65875404
    mem[288] = 2867
    mask = 0000010X1100X11X011XXX00000X0X100001
    mem[43559] = 464644356
    mem[10885] = 68416
    mem[39739] = 371285
    mask = 0000110010X11110010000X1X0010X000100
    mem[47227] = 165771882
    mem[48356] = 1737
    mem[63598] = 2483374
    mem[2316] = 34095112
    mem[55289] = 74755
    mem[41105] = 261242
    mem[14118] = 11265010
    mask = 0000X10010011110X100000XX00X0XX011X0
    mem[2169] = 6081363
    mem[6190] = 2945060
    mem[52821] = 198612172
    mem[59003] = 11598655
    mem[52361] = 4707187
    mem[56280] = 196
    mem[10885] = 286349
    mask = X00101X010X011X10X1000X1XX0010X10100
    mem[33153] = 3094197
    mem[43769] = 378106
    mem[6419] = 1963
    mem[57515] = 2939
    mem[7665] = 178455
    mem[53942] = 61817
    mem[2169] = 268870
    mask = 0X00010010X111100X0000000001X0100111
    mem[42106] = 12354614
    mem[2361] = 1153440
    mem[41105] = 455
    mem[59601] = 18520827
    mask = 0100010X11010XX001X10X0111010X1011X0
    mem[6429] = 4758709
    mem[17816] = 31081595
    mem[61038] = 1538011
    mask = 00X0XX001001111X0100X0X11000X000101X
    mem[23434] = 9452012
    mem[2316] = 26173974
    mem[13540] = 974635
    mem[6886] = 370144
    mem[4507] = 106249
    mem[30978] = 2773340
    mask = 100011001101X1X0010X10010011X0X0X100
    mem[18680] = 367
    mem[52656] = 37743
    mask = 0001011XX0101X1X0110X111011XX000001X
    mem[37014] = 1381807
    mem[65197] = 11932464
    mem[37789] = 802
    mem[29691] = 129226332
    mem[31407] = 145121
    mask = 0100X10X11X011100100011100X0XX11101X
    mem[55826] = 7929
    mem[53828] = 245259
    mask = 00X001001100X111X1XX00001X11X0100X1X
    mem[43424] = 52459
    mem[60346] = 100025
    mem[42472] = 1922567
    mask = 000001X010X1X1100100X0101X1XXXX11101
    mem[42536] = 467259249
    mem[2998] = 208484
    mem[58340] = 221525908
    mem[2083] = 50801173
    mask = 1X00XX0X10010110010111010110X0X10000
    mem[16052] = 25329580
    mem[40234] = 25679682
    mem[8096] = 2721108
    mask = 000X0X00X1001X0101000100100110100001
    mem[7217] = 629693595
    mem[11248] = 19909152
    mem[33029] = 409771
    mem[64325] = 44760
    mem[22786] = 334
    mask = X00X01X110X00111X010100110100101101X
    mem[35427] = 146692
    mem[10885] = 24787653
    mem[8704] = 130590204
    mask = 0XX10100X000111101X01000000100X0X1X0
    mem[41650] = 104786
    mem[11186] = 4092
    mem[8615] = 2326
    mem[21984] = 119400806
    mem[37273] = 134
    mask = 0X01XX0001001X0X01001XX110X111101111
    mem[63598] = 3600
    mem[13006] = 48108985
    mem[2860] = 1278
    mem[46726] = 79405271
    mask = 0X0001XX11000XX001X0010XXX1010000100
    mem[4497] = 496767
    mem[27421] = 128467442
    mem[24801] = 10348797
    mask = 00X0X100110X1101011000011100X0011X1X
    mem[14376] = 2068153
    mem[28383] = 359910735
    mem[13969] = 30791824
    mem[62629] = 402435532
    mask = 000001XX100111101X01X10001010X101111
    mem[25690] = 50457545
    mem[1233] = 1291762
    mem[56844] = 397675015
    mem[38951] = 7383400
    mem[59586] = 12871618
    mem[24941] = 21686632
    mask = 0X0X010111001X11110X1XX1X110X010X000
    mem[41060] = 124962
    mem[54368] = 2911
    mem[20727] = 32214
    mem[46326] = 129851509
    mask = 00000101110001X001000X1100XX10X01100
    mem[30374] = 3455
    mem[64325] = 15309
    mem[25497] = 3309
    mem[40364] = 1830
    mem[63039] = 74775
    mem[25932] = 165291
    mask = 00X001XX10X111101100001000X0X01X11X1
    mem[52101] = 756809
    mem[5226] = 7
    mem[14307] = 234925
    mem[6395] = 109496000
    mem[10792] = 92833
    mem[46226] = 163865
    mask = 000X0X001011X1X0X1XX1010000X10011X00
    mem[52912] = 1339
    mem[29296] = 63918134
    mem[34563] = 343625
    mem[13008] = 9303286
    mem[1765] = 89091827
    mem[53413] = 14712
    mask = 010011001X011XX0X10X011X00101110X001
    mem[8985] = 82037
    mem[6567] = 466193
    mem[33293] = 935929
    mem[51318] = 231696746
    mem[55982] = 483157
    mask = 1001010010X11X11010X0000X001X0100100
    mem[47030] = 261036783
    mem[12812] = 6644
    mem[50536] = 18755307
    mem[41381] = 505
    mem[54593] = 20452
    mem[23092] = 75890388
    mask = X00X010010011X1X01001010000101101001
    mem[58866] = 36665
    mem[363] = 226991
    mem[15405] = 49937373
    mem[57454] = 871817
    mask = 01000110XX000X100XX000X001101X100100
    mem[20727] = 193101
    mem[36339] = 232513
    mem[18661] = 10662857
    mask = 010X11001X011X100111101X0X1XX1101011
    mem[9904] = 3046164
    mem[37443] = 256455
    mem[1041] = 150
    mem[8865] = 1636
    mem[34246] = 173885598
    mem[64959] = 88455264
    mem[49483] = 64891
    mask = X0000000101X0111X100100000X011X11X00
    mem[28531] = 11915
    mem[59199] = 131
    mem[1459] = 524071622
    mem[36860] = 9055
    mem[1174] = 48263
    mem[9545] = 7743
    mem[45185] = 20010
    mask = 000X010X1X0XXX0X0110000000X001X00000
    mem[62370] = 224058421
    mem[38859] = 409631
    mem[59400] = 174
    mem[36789] = 484044369
    mem[3458] = 63678
    mem[59303] = 216159
    mask = XX0101X01X00110101010001101000101101
    mem[5478] = 475223
    mem[47030] = 575987
    mem[56315] = 496590
    mask = 0010010X00001X010100111X10X101X00011
    mem[33813] = 29876402
    mem[1939] = 6791
    mem[54296] = 1415980
    mem[49761] = 4161201
    mem[3087] = 3082805
    mem[44185] = 1458126
    mem[33380] = 1574138
    mask = 0001010X10101X01011000X111X1100X0100
    mem[39169] = 6924
    mem[44518] = 550
    mem[62900] = 54431
    mem[11277] = 64
    mem[27209] = 29226137
    mem[47090] = 2232874
    mask = 010XX1X010001111010XX11001X0X0110X01
    mem[65127] = 22950
    mem[43419] = 107162216
    mem[5283] = 1501
    mem[32728] = 984590
    mask = XX00X1001XX10X1001010001010100100XX0
    mem[2361] = 1635
    mem[34560] = 197
    mem[52912] = 173403443
    mem[29240] = 5911444
    mask = 0001110X1000X100X00001000010X1101101
    mem[16416] = 20663923
    mem[39739] = 317
    mem[61038] = 275054
    mask = 000X011010101101X0100001X0X00X10XX0X
    mem[10489] = 23700
    mem[22453] = 2301
    mem[56957] = 362048754
    mem[53036] = 80024427
    mem[26144] = 125087
    mem[57227] = 144191
    mem[23435] = 724551
    mask = X00X01001X0X11010110X010000010100111
    mem[37068] = 1442
    mem[60589] = 81139773
    mem[31775] = 220923
    mem[30500] = 4862
    mem[11915] = 62908671
    mask = 0001010XX1001101XX0000010X0100001101
    mem[40457] = 120318
    mem[54520] = 1342
    mem[33156] = 220484401
    mem[60135] = 1408362
    mem[7395] = 75576
    mask = 000X01X01100111101X00X00100110000X11
    mem[9237] = 829933
    mem[9851] = 14818
    mem[58919] = 32111
    mem[14595] = 1623804
    mem[61075] = 62727
    mask = XX0001XX1X0111101101X0X110101X0X0110
    mem[43769] = 2774
    mem[42615] = 811
    mem[21176] = 105435272
    mask = 000001011XX0X111XXXX10000000X110100X
    mem[17931] = 510002
    mem[54880] = 3002
    mem[56280] = 57973141
    mem[6675] = 82200
    mem[770] = 574
    mem[13978] = 8578
    mem[20408] = 1000405042
    mask = 000X0110X0101X011X100111X100000X0X01
    mem[36150] = 11494126
    mem[63140] = 36830
    mem[30752] = 3099
    mem[46348] = 834895
    mem[9747] = 5886304
    mem[63762] = 513447
    mask = X001010XX00X110101100X00100100010X01
    mem[13477] = 2699
    mem[23838] = 31014
    mem[44430] = 2874805
    mem[35244] = 224489846
    mem[35854] = 116779
    mem[52912] = 146386
    mem[13340] = 166011
    mask = 0X0001000X001110X1101010X10001001X01
    mem[1964] = 754299614
    mem[47212] = 24758
    mem[16310] = 219448
    mem[28451] = 196161
    mask = 000001011000011X0000000X01XX0100XXX1
    mem[5082] = 113278507
    mem[42611] = 453335034
    mem[61959] = 148558273
    mem[50345] = 11235
    mem[30489] = 9511
    mem[47848] = 63623
    mask = X001110X10X01X0110X0X0XX101010101101
    mem[5883] = 689
    mem[15916] = 2875
    mem[1517] = 501286
    mask = 1X00010XXX01111001000X101001X0100100
    mem[54668] = 23172
    mem[16960] = 2017
    mem[2107] = 90696075
    mem[52539] = 2140275
    mem[32037] = 21705534
    mem[34853] = 1308
    mask = 0XX011001X0X100001X001X010100100XXX1
    mem[21170] = 211531689
    mem[8685] = 125900148
    mem[65403] = 1016
    mem[10635] = 1290678
    mem[25179] = 14286
    mem[16052] = 284451
    mask = 00000100XX00111X01XX10000000X11001X0
    mem[39191] = 366121
    mem[15405] = 3693416
    mem[8683] = 767619495
    mask = 0X010100101111001101000000X00XX00000
    mem[28999] = 1322760
    mem[26472] = 216672
    mem[32402] = 476392921
    mem[42717] = 3985941
    mem[38951] = 12294
    mask = 001X01101001X11011XX11X00X1000000100
    mem[46301] = 1320
    mem[23356] = 2331
    mem[20456] = 2746177
    mem[11049] = 3540
    mem[15652] = 14656035
    mask = 010001111X0000X0X1X0X111010011XX0000
    mem[54387] = 658
    mem[34337] = 940
    mem[30373] = 48529
    mem[4077] = 750
    mem[1340] = 64485
    mem[24868] = 401953136
    mask = 0000X0001011X11111001XX0110101000X00
    mem[21980] = 50716859
    mem[7078] = 15673
    mem[9550] = 474469863
    mask = 00000X0010X11X10110000X10X1001XX1111
    mem[37443] = 65084120
    mem[56057] = 51324
    mem[23081] = 365062378
    mem[10369] = 80230
    mem[25662] = 995
    mem[12226] = 8728723
    mask = 00X00X001X001111XX00111X0000X0100100
    mem[24337] = 67654
    mem[7422] = 219732
    mem[34006] = 90813
    mem[28012] = 20086018
    mem[11044] = 4202238
    mask = 0000X1X01001XX10010X0010X1100001100X
    mem[63006] = 2909
    mem[23861] = 109538
    mask = 0X0001001001X1100X0000XX00001X0XXX01
    mem[14316] = 41795149
    mem[42662] = 783
    mem[2588] = 556226
    mem[36063] = 21261
    mem[47357] = 7489764
    mem[48826] = 159937
    mask = 0101X100X000111101X01001000XX0010X01
    mem[41730] = 38337
    mem[37021] = 704
    mem[7973] = 110815
    mem[58423] = 1191
    mask = X001010XX000110101101010100000X00X0X
    mem[33929] = 127639490
    mem[28249] = 558365819
    mem[24652] = 84719
    mem[30439] = 227
    mem[9407] = 54748681
    mask = X00101X010X0110XX110X1001000011X1100
    mem[41037] = 2668741
    mem[42937] = 14638
    mem[9732] = 465258
    mem[45018] = 78913023
    mem[17076] = 206317731
    mem[8317] = 709616657
    mask = 0X0001X0100110100X010X1X11110001XX00
    mem[57629] = 156141947
    mem[23356] = 29013450
    mask = X00001X01X01X110110X11000X001X11X11X
    mem[23284] = 929
    mem[23092] = 986875
    mem[46089] = 2071258
    mem[57227] = 191024
    mem[49637] = 1585
    mask = 00X00110110101101X011X000000100001X0
    mem[2998] = 1243
    mem[28002] = 95174193
    mem[61488] = 8150
    mask = 0000011X110000000X000100X01X10010101
    mem[38827] = 1109
    mem[15337] = 24669
    mem[6661] = 897
    mask = X00X0100100X111X01001X10X001X00X0100
    mem[363] = 58081970
    mem[35105] = 31435010
    mem[52446] = 4898641
    mask = X000011X1001X1X00100110010X110000X0X
    mem[41625] = 54710327
    mem[6567] = 354249
    mem[23694] = 8860".to_string();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn radix_convert() {
    assert_eq!("000000000000000000000000000000001011", pad_36(&format_radix(11, 2)));
    assert_eq!("000000000000000000000000000001100101", pad_36(&format_radix(101, 2)));
  }

  #[test]
  fn sum_mem_after_mask() {
    let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
      mem[8] = 11
      mem[7] = 101
      mem[8] = 0";
    assert_eq!(165, add_mem(&parse_input(input)));
  }

  #[test]
  fn apply_36_mask() {
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    assert_eq!("000000000000000000000000000001001001", apply_mask(mask, "000000000000000000000000000000001011"));
    assert_eq!("000000000000000000000000000001000000", apply_mask(mask, "000000000000000000000000000000000000"));
    assert_eq!("000000000000000000000000000001100101", apply_mask(mask, "000000000000000000000000000001100101"));
  }

  #[test]
  fn convert_string_dec() {
    assert_eq!(101, i64::from_str_radix("000000000000000000000000000001100101", 2).unwrap());
    assert_eq!(64, i64::from_str_radix("000000000000000000000000000001000000", 2).unwrap());
  }

  #[test]
  fn sum_mem_version_2() {
    let input = "mask = 000000000000000000000000000000X1001X
      mem[42] = 100
      mask = 00000000000000000000000000000000X0XX
      mem[26] = 1";
    assert_eq!(208, add_mem_v2(&parse_input(input)));
  }

  #[test]
  fn appply_mask_v2(){
    let mask = "000000000000000000000000000000X1001X";
    let expected = vec![26,58,27,59];
    assert_eq!(expected, apply_mem_mask(mask, "000000000000000000000000000000101010"))
  }
}