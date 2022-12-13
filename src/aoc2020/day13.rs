/*
--- Day 13: Shuttle Search ---

Your ferry can make it safely to a nearby port, but it won't get much further. When you call to book another ship, you discover that no ships embark from that port to your vacation island. You'll need to get from the port to the nearest airport.

Fortunately, a shuttle bus service is available to bring you from the sea port to the airport! Each bus has an ID number that also indicates how often the bus leaves for the airport.

Bus schedules are defined based on a timestamp that measures the number of minutes since some fixed reference point in the past. At timestamp 0, every bus simultaneously departed from the sea port. After that, each bus travels to the airport, then various other locations, and finally returns to the sea port to repeat its journey forever.

The time this loop takes a particular bus is also its ID number: the bus with ID 5 departs from the sea port at timestamps 0, 5, 10, 15, and so on. The bus with ID 11 departs at 0, 11, 22, 33, and so on. If you are there when the bus departs, you can ride that bus to the airport!

Your notes (your puzzle input) consist of two lines. The first line is your estimate of the earliest timestamp you could depart on a bus. The second line lists the bus IDs that are in service according to the shuttle company; entries that show x must be out of service, so you decide to ignore them.

To save time once you arrive, your goal is to figure out the earliest bus you can take to the airport. (There will be exactly one such bus.)

For example, suppose you have the following notes:

939
7,13,x,x,59,x,31,19
Here, the earliest timestamp you could depart is 939, and the bus IDs in service are 7, 13, 59, 31, and 19. Near timestamp 939, these bus IDs depart at the times marked D:

time   bus 7   bus 13  bus 59  bus 31  bus 19
929      .       .       .       .       .
930      .       .       .       D       .
931      D       .       .       .       D
932      .       .       .       .       .
933      .       .       .       .       .
934      .       .       .       .       .
935      .       .       .       .       .
936      .       D       .       .       .
937      .       .       .       .       .
938      D       .       .       .       .
939      .       .       .       .       .
940      .       .       .       .       .
941      .       .       .       .       .
942      .       .       .       .       .
943      .       .       .       .       .
944      .       .       D       .       .
945      D       .       .       .       .
946      .       .       .       .       .
947      .       .       .       .       .
948      .       .       .       .       .
949      .       D       .       .       .
The earliest bus you could take is bus ID 59. It doesn't depart until timestamp 944, so you would need to wait 944 - 939 = 5 minutes before it departs. Multiplying the bus ID by the number of minutes you'd need to wait gives 295.

What is the ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus?

Your puzzle answer was 207.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---

The shuttle company is running a contest: one gold coin for anyone that can find the earliest timestamp such that the first bus ID departs at that time and each subsequent listed bus ID departs at that subsequent minute. (The first line in your input is no longer relevant.)

For example, suppose you have the same list of bus IDs as above:

7,13,x,x,59,x,31,19
An x in the schedule means there are no constraints on what bus IDs must depart at that time.

This means you are looking for the earliest timestamp (called t) such that:

Bus ID 7 departs at timestamp t.
Bus ID 13 departs one minute after timestamp t.
There are no requirements or restrictions on departures at two or three minutes after timestamp t.
Bus ID 59 departs four minutes after timestamp t.
There are no requirements or restrictions on departures at five minutes after timestamp t.
Bus ID 31 departs six minutes after timestamp t.
Bus ID 19 departs seven minutes after timestamp t.
The only bus departures that matter are the listed bus IDs at their specific offsets from t. Those bus IDs can depart at other times, and other bus IDs can depart at those times. For example, in the list above, because bus ID 19 must depart seven minutes after the timestamp at which bus ID 7 departs, bus ID 7 will always also be departing with bus ID 19 at seven minutes after timestamp t.

In this example, the earliest timestamp at which this occurs is 1068781:

time     bus 7   bus 13  bus 59  bus 31  bus 19
1068773    .       .       .       .       .
1068774    D       .       .       .       .
1068775    .       .       .       .       .
1068776    .       .       .       .       .
1068777    .       .       .       .       .
1068778    .       .       .       .       .
1068779    .       .       .       .       .
1068780    .       .       .       .       .
1068781    D       .       .       .       .
1068782    .       D       .       .       .
1068783    .       .       .       .       .
1068784    .       .       .       .       .
1068785    .       .       D       .       .
1068786    .       .       .       .       .
1068787    .       .       .       D       .
1068788    D       .       .       .       D
1068789    .       .       .       .       .
1068790    .       .       .       .       .
1068791    .       .       .       .       .
1068792    .       .       .       .       .
1068793    .       .       .       .       .
1068794    .       .       .       .       .
1068795    D       D       .       .       .
1068796    .       .       .       .       .
1068797    .       .       .       .       .
In the above example, bus ID 7 departs at timestamp 1068788 (seven minutes after t). This is fine; the only requirement on that minute is that bus ID 19 departs then, and it does.

Here are some other examples:

The earliest timestamp that matches the list 17,x,13,19 is 3417.
67,7,59,61 first occurs at timestamp 754018.
67,x,7,59,61 first occurs at timestamp 779210.
67,7,x,59,61 first occurs at timestamp 1261476.
1789,37,47,1889 first occurs at timestamp 1202161486.
However, with so many bus IDs in your list, surely the actual earliest timestamp will be larger than 100000000000000!

What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list?
530015546283687
*/

pub fn parse_input(input: &str) -> (usize, Vec<&str>) {
    let input_split: Vec<&str> = input.split('\n').collect();
    let start_time: usize = input_split[0].parse().unwrap();
    let entries: Vec<&str> = input_split[1].split(',').collect();
    (start_time, entries)
}

pub fn part1(input: &(usize, Vec<&str>)) -> usize {
    let local_input = input.clone();
    let start_time = local_input.0;
    let bus_lines: Vec<usize> = local_input
        .1
        .iter()
        .filter_map(|i| i.parse::<usize>().ok())
        .collect();
    let (bus, time) = get_earliest_bus_and_departure_time(start_time, bus_lines);
    (time - start_time) * bus
}

fn get_earliest_bus_and_departure_time(start_time: usize, bus_lines: Vec<usize>) -> (usize, usize) {
    bus_lines
        .iter()
        .map(|bus| {
            let time = {
                let d = start_time / *bus;
                let r = start_time % *bus;
                if r > 0 && *bus > 0_usize {
                    d + 1
                } else {
                    d
                }
            };
            (*bus, bus * time)
            // }).fold(0_usize,|pre, current| pre.min(current.1) )
        })
        .reduce(|pre, current| if pre.1 > current.1 { current } else { pre })
        .expect("iterator was empty!!")
}

pub fn part2(input: &(usize, Vec<&str>)) -> usize {
    let mut bus_with_index: Vec<(usize, usize)> = input
        .1
        .clone()
        .iter()
        .enumerate()
        .filter(|(_, bus)| **bus != "x")
        .map(|(index, bus)| (index, bus.parse::<usize>().expect("wrong value in input!")))
        .collect();

    // println!("busses {:?}", bus_with_index);
    let mut increase = bus_with_index.remove(0).1;
    let mut timestamp = increase;
    // println!("increase {}", increase);

    for (index, bus) in bus_with_index {
        while (timestamp + index) % bus != 0 {
            timestamp += increase;
            // println!(
            //     "timestamp {} looking for bus {} with index {}",
            //     timestamp, bus, index
            // );
        }
        increase *= bus;
        // println!(
        //     "solved for bus {} in timestamp {}, increase is now {}",
        //     bus, timestamp, increase
        // );
    }
    timestamp
}

pub fn part2_with_chinese_rt(input: &(usize, Vec<&str>)) -> usize {
    let bus_with_index = input
        .1
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, bus)| *bus != "x")
        .map(|(index, bus)| (index, bus.parse::<usize>().expect("wrong value in input!")));
    let (residues, modulii): (Vec<i64>, Vec<i64>) = bus_with_index
        .map(|(index, bus)| ((bus as i64 - index as i64), bus as i64))
        .unzip();
    match chinese_remainder(&residues, &modulii) {
        Some(sol) => sol.try_into().expect("negative chinese result!"),
        None => panic!("modulii not pairwise coprime"),
    }
}

// from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = "939
7,13,x,x,59,x,31,19
";

    #[test]
    fn test_part1_sample_1() {
        assert_eq!(295, part1(&parse_input(SAMPLE_INPUT_1)));
    }

    #[test]
    fn test_part2_sample_1() {
        assert_eq!(1068781, part2(&parse_input(SAMPLE_INPUT_1)));
    }

    const SAMPLE_INPUT_2: &str = "939
17,x,13,19";
    #[test]
    fn test_part2_sample_2() {
        assert_eq!(3417, part2(&parse_input(SAMPLE_INPUT_2)));
    }
    #[test]
    fn test_part2_crt_sample_2() {
        assert_eq!(3417, part2_with_chinese_rt(&parse_input(SAMPLE_INPUT_2)));
    }
    const SAMPLE_INPUT_3: &str = "939
67,7,59,61";
    #[test]
    fn test_part2_sample_3() {
        assert_eq!(754018, part2(&parse_input(SAMPLE_INPUT_3)));
    }
    const SAMPLE_INPUT_4: &str = "939
67,x,7,59,61";
    #[test]
    fn test_part2_sample_4() {
        assert_eq!(779210, part2(&parse_input(SAMPLE_INPUT_4)));
    }
    const SAMPLE_INPUT_5: &str = "939
67,7,x,59,61";
    #[test]
    fn test_part2_sample_5() {
        assert_eq!(1261476, part2(&parse_input(SAMPLE_INPUT_5)));
    }
    const SAMPLE_INPUT_6: &str = "939
1789,37,47,1889";
    #[test]
    fn test_part2_sample_6() {
        assert_eq!(1202161486, part2(&parse_input(SAMPLE_INPUT_6)));
    }
}
