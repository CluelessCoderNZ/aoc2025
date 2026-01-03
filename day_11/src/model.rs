use std::{collections::HashMap, fmt::{Debug, Display}};


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(u32);

impl DeviceId {
    pub fn you() -> Self {
        DeviceId::try_from("you").unwrap()
    }

    pub fn server_rack() -> Self {
        DeviceId::try_from("svr").unwrap()
    }

    pub fn dac() -> Self {
        DeviceId::try_from("dac").unwrap()
    }

    pub fn fft() -> Self {
        DeviceId::try_from("fft").unwrap()
    }

    pub fn end() -> Self {
        DeviceId::try_from("out").unwrap()
    }
}

impl TryFrom<&str> for DeviceId {
    type Error = ();
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut output: u32 = 0;

        if value.len() > 4 {
            return Err(());
        }
        
        for (index, char) in value.chars().enumerate() {
            if char.is_ascii_alphabetic() {
                let char = u8::try_from(char).map_err(|_| ())?;
                output |= (char as u32) << (8*index); 
            } else {
                return Err(());
            }
        }

        Ok(Self(output))
    } 
}

impl Into<String> for DeviceId {
    fn into(self) -> String {
        (0..std::mem::size_of::<u32>())
        .filter_map(|i| {
            let mask = (u8::MAX as u32) << (8*i);
            let char = (self.0 & mask) >> (8*i);
            let char = char::try_from(char).unwrap();

            if char.is_ascii_alphabetic() {
                Some(char)
            } else {
                None
            }
        }).collect::<String>()
    }
}

impl Display for DeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<String>::into(*self))
    }
}

impl Debug for DeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(\"{}\": {})", self, self.0)
    }
}

pub struct Device {
    pub id: DeviceId,
    pub outputs: Vec<DeviceId>
}

impl Device {
    pub fn from_str(name: &str, outputs: Vec<&str>) -> Result<Self, ()> {
        let id = DeviceId::try_from(name)?;
        let outputs = outputs.into_iter()
            .map(|out| DeviceId::try_from(out))
            .collect::<Result<Vec<DeviceId>, ()>>()?;

        Ok(Self {
            id,
            outputs
        })
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output_str = self.outputs.iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{}: {}", self.id, output_str)
    }
}

pub fn count_all_paths(graph: &HashMap<DeviceId, Vec<DeviceId>>) -> u64 {
    explore(&DeviceId::you(), graph)
}

pub fn count_problematic_paths(graph: &HashMap<DeviceId, Vec<DeviceId>>) -> u64 {
    explore_problem(
        &DeviceId::server_rack(),
        &DeviceId::end(),
        graph,
        0,
       &mut Default::default()
    )
}

fn explore(
    node: &DeviceId,
    graph: &HashMap<DeviceId, Vec<DeviceId>>
) -> u64 {
    if *node == DeviceId::end() {
        1
    } else {
        graph[node].iter()
            .map(|neighbour| explore(neighbour, graph))
            .sum()
    }
}

fn explore_problem(
    node: &DeviceId,
    target: &DeviceId,
    graph: &HashMap<DeviceId, Vec<DeviceId>>,
    mut visited_flag: u8,
    cache: &mut HashMap<u32, u64>
) -> u64 {
    // Bit of a hack since I know the top 8 bit are not used,
    // but it is a *fun* & **fast** hack
    let cache_key = node.0 | ((visited_flag as u32) << 24);
    if node == target {
        if visited_flag == 0b11 {
            1
        } else {
            0
        }
    } else if let Some(result) = cache.get(&cache_key) {
        *result
    } else {
        if *node == DeviceId::dac() {
            visited_flag |= 0b01;
        } else if *node == DeviceId::fft() {
            visited_flag |= 0b10;
        } 

        let subtotal = graph[node].iter()
            .map(|neighbour| explore_problem(neighbour, target, graph, visited_flag, cache))
            .sum();

        cache.insert(cache_key, subtotal);
        subtotal
    }
}

#[cfg(test)]
mod test {
    use crate::model::DeviceId;

    #[test]
    fn test_device_id_ok() {
        let name = "wow";
        let id = DeviceId::try_from(name).expect("Valid name");
        let parsed: String = id.into();
        assert_eq!(name, parsed)
    }

    #[test]
    fn test_device_id_invalid_long() {
        let name = "abcde";
        DeviceId::try_from(name).expect_err("Not valid name");
    }

    #[test]
    fn test_device_id_invalid_chars() {
        let name = "999";
        DeviceId::try_from(name).expect_err("Not valid name");
    }
}