mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

fn main() {
    let expenses = day1::read_expenses();
    println!("Day 1 expense result: {}", day1::find_expense(&expenses));
    println!("Day 1 (three) expense: {}", day1::find_expense_three(&expenses));

    let passwords = day2::read_passwords();
    // let passwords = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    println!("Day2 valid passwords (range): {}", day2::count_valid_passwords_range(&passwords));
    println!("Day2 valid passwords (position): {}", day2::count_valid_passwords_position(&passwords));

    let geo = day3::read_geology();
    let slope_3_1 = day3::count_trees_using_slope(&geo, 1, 3);
    println!("Day3 num trees by slope (right 3, down 1) {}", slope_3_1);
    let slope_1_1 = day3::count_trees_using_slope(&geo, 1, 1);
    let slope_5_1 = day3::count_trees_using_slope(&geo, 1, 5);
    let slope_7_1 = day3::count_trees_using_slope(&geo, 1, 7);
    let slope_1_2 = day3::count_trees_using_slope(&geo, 2, 1);
    println!("Day3 num trees multiplied for all slopes: {}", slope_1_1 * slope_1_2 * slope_3_1 * slope_5_1 * slope_7_1 );

    let passports = day4::read_passports();
    println!("Day4 count valid passports {}", day4::count_valid_passports(&passports));
    println!("Day4 count validated passports {}", day4::count_validated_passports(&passports));

    let boarding_passes = day5::read_boarding_passes();
    println!("Day 5 highest boarding pass id {}", day5::highest_id(&boarding_passes));
    println!("Day 5 find missing seat id {}", day5::find_missing_seat(&boarding_passes));

    let customs = day6::read_customs();
    println!("Day6 add all families customs {}", day6::add_all_customs_union(&customs));
    println!("Day6 add all families customs intersect {}", day6::add_all_customs_intersect(&customs));

    let bag_rules = day7::read_rules();
    println!("Day 7 possible gold containing bags {}", day7::count_bags_with_gold(&bag_rules));
    println!("Day 7 count required bags {}", day7::count_required_bags(&bag_rules));

    let boot_code = day8::read_boot_instructions();
    println!("Day 8 accumulator at infinite loop: {}", day8::accumulator_at_infinite_loop(&boot_code).1);
    println!("Day 8 program terminates with {}", day8::find_termination(&boot_code));

    let cypher = day9::read_cypher();
    let first_invalid = day9::find_first_not_sum(&cypher, 25);
    println!("Day 9 first not matching value {}", first_invalid);
    println!("Day 9 find encryption weakness {}", day9::find_contiguous_sum(first_invalid, &cypher));

    let adapters = day10::read_adapters();
    println!("Day 10 all adapter jolt diff {}", day10::jolt_diff_using_all_adapters(&adapters));
    println!("Day 10 all combos {}", day10::total_configurations(&adapters));

    let seats = day11::parse_seating(&day11::read_input());
    println!("Day 11 total occupied when stable {}", day11::count_stable_occupied(&seats));
    println!("Day 11 total occupied when stable {}", day11::count_stable_los(&seats));

    let instructions = day12::parse_instructions(&day12::read_input());
    println!("Day12 Manhattan distance {}", day12::navigate_and_get_position(&instructions));
    println!("Day12 Navigate using waypoint {}", day12::naviage_using_waypoint(&instructions));

    let bus_times = day13::parse_input_start_time(&day13::read_input());
    println!("Day13 bus id times time to wait {}", day13::earliest_bus(bus_times.0, &bus_times.1));
    println!("Day13 first matching timestamp {}", day13::find_first_contiguous_time(&day13::parse_input_with_offsets(&day13::read_input())));

    let mask_instructions = day14::parse_input(&day14::read_input());
    println!("Day 14 sum memory after masks {}", day14::add_mem(&mask_instructions));
    println!("Day 14 sum memory version 2 {}", day14::add_mem_v2(&mask_instructions));

    let starting_numbers = day15::parse_input(&day15::read_input());
    println!("Day 15 2020th number {}", day15::find_nth_number_memo(&starting_numbers, 2020));
    // just a little too slow
    //println!("Day 15 30000000 number {}", day15::find_nth_number_memo(&starting_numbers, 30000000));

    let nearby_tickets = day16::parse_tickets(&day16::read_nearby_tickets());
    let ticket_rules = day16::parse_rules(&day16::read_rules());
    let my_ticket = day16::read_my_ticket();
    println!("Day 16 Ticket scanning error rate {}", day16::ticket_scanning_error_rate(&ticket_rules, &nearby_tickets));
    println!("Day 16 departure fields {}", day16::multiply_departure_values(&ticket_rules, &nearby_tickets, &my_ticket));

    let energy_grid_input = day17::read_input();
    println!("Day 17 active after 6 cycles {}", day17::active_after_6cycles(&day17::parse_input3d(&energy_grid_input)));
    // also a little too slow
    // println!("Day 17 active after 6 cycles in 4d {}", day17::active_after_6cycles_4d(&day17::parse_input4d(&energy_grid_input)));

    let equations = day18::read_expressions();
    println!("Day 18 sum all expressions {}", day18::sum_all_expressions(&equations));
    println!("Day 18 sum all with addition first {}", day18::sum_all_plus_order(&equations));

    let rules = day19::read_rules();
    let messages = day19::read_messages();
    println!("Day 10 valid messages {}", day19::count_valid_messages(&rules, &messages));
}
