#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PacketData {
    Item(usize),
    List(Vec<PacketData>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Packet {
    data: PacketData,
}

pub fn parse_packet_data(input: &str) -> (Vec<PacketData>, &str) {
    let mut parsed: Vec<PacketData> = vec![];
    let mut chars = input.chars();

    let mut buffer = String::new();

    while let Some(char) = chars.next() {
        if char == ']' {
            if !buffer.is_empty() {
                parsed.push(PacketData::Item(buffer.parse().unwrap()));
            }
            return (parsed, chars.as_str());
        }

        if char == '[' {
            let (nested, rest) = parse_packet_data(chars.as_str());
            parsed.push(PacketData::List(nested));
            chars = rest.chars();
            continue;
        }

        if char == ',' {
            if !buffer.is_empty() {
                parsed.push(PacketData::Item(buffer.parse().unwrap()));
                buffer.clear();
            }

            continue;
        }

        buffer += char.to_string().as_str();
    }

    (parsed, chars.as_str())
}

/*
impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PacketData::Integer(a), PacketData::Integer(b)) => a.partial_cmp(b),
            (PacketData::Integer(a), b) => {
                PacketData::List(vec![PacketData::Integer(*a)]).partial_cmp(b)
            }
            (a, PacketData::Integer(b)) => {
                a.partial_cmp(&PacketData::List(vec![PacketData::Integer(*b)]))
            }
            (PacketData::List(a), PacketData::List(b)) => {
                let not_equal_pair = a.iter().zip(b.iter()).find(|(a, b)| !a.eq(b));

                if let Some((a, b)) = not_equal_pair {
                    a.partial_cmp(b)
                } else if a.len() < b.len() {
                    Some(std::cmp::Ordering::Less)
                } else if a.len() > b.len() {
                    Some(std::cmp::Ordering::Greater)
                } else {
                    None
                }
            }
        }
    }
}
*/

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketData::Item(a), PacketData::Item(b)) => {
                let ordering = a.cmp(b);
                /* println!("Compare {} vs {}, => {:?}", a, b, ordering); */
                a.cmp(b)
            }
            (PacketData::Item(_), PacketData::List(_)) => {
                PacketData::List(vec![self.clone()]).cmp(other)
            }
            (PacketData::List(a), PacketData::List(b)) => {
                let mut cursor = 0;

                while cursor < a.len().min(b.len()) {
                    let ordering = a[cursor].cmp(&b[cursor]);
                    cursor += 1;

                    if ordering.is_eq() {
                        continue;
                    }

                    return ordering;
                }

                let ordering = a.len().cmp(&b.len());

                // println!("a.len()={} < b.len({}), {:?}", a.len(), b.len(), ordering);

                ordering
            }
            (PacketData::List(_), PacketData::Item(_)) => {
                self.cmp(&PacketData::List(vec![other.clone()]))
            }
        }
    }
}

impl From<&str> for Packet {
    fn from(input: &str) -> Self {
        let (packets, _) = parse_packet_data(input);

        let top_packet = packets.first().unwrap().clone();

        Self { data: top_packet }
    }
}

pub fn solve_1(input: &str) -> String {
    let blocks = input.split("\n\n");

    let mut sum = 0;

    for (index, block) in blocks.enumerate() {
        let line_vector: Vec<&str> = block.lines().collect();
        assert!(line_vector.len() == 2);
        let left = Packet::from(line_vector[0]);
        let right = Packet::from(line_vector[1]);

        if left < right {
            sum += index + 1;
        }
    }

    sum.to_string()
}

pub fn solve_2(input: &str) -> String {
    "Ok".to_string()
}
