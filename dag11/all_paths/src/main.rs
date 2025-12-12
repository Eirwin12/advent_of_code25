fn main() {
    println!("Hello, world!");
}
#[derive(PartialEq)]
struct Devices {
    input: &str,
    output: Vec<&str>,
}

fn receive_input(input: &str) -> Some(Vec<Devices>) {
    let mut output: Vec<Devices> = Vec::new();
    for line in input.lines() {
        let Some(end) = line.find(':') else {return None;};
        let mut output_dev: Vec<&str> = Vec::new();
        for device in line[(end+2)..].split(" ") {
            output_dev.push(device);
        }
        output.push(Devices { input: line[0..end], output: output_dev });
    }
    output
}

//want default values, but isn't available
//https://stackoverflow.com/questions/24047686/default-function-arguments-in-rust
//enter None for default value
fn all_paths(devices: Vec<Devices>, begin: Option<&str>, end: Option<&str>) ->u64 {
    fn path(devices: Vec<Devices>, mut begin: &Devices, end: &str) -> Option<u64> {
        if begin.output.contains("out") {
            return Some(1);
        }
        let sum: u64 = 0;
        let mut i = 0;
        while i < devices.len(){
        // for device in devices {
            let device = devices[i];
            if !begin.output.contains(&device.input) {
                i += 1;
                continue;
            }
            //found a link
            if device.output.len() >=1 {
                sum += path(devices, &device, end)? + 1;
                i += 1;
            }
            else {
                begin = &device;
                i = 0;
            }
        }
        if sum == 0 {
            None
        }
        else {
            Some(sum)
        }
    }

    begin.unwrap_or_else("you");
    end.unwrap_or_else("out");
    let mut sum = 0;
    for device in devices {
        if device.input == begin {
            sum += path(devices, &device, end);
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_devices() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out";
        let all_devices: Vec<Devices> = vec![
            Devices{input: "aaa", output:vec!["you", "hhh"]},
            Devices{input: "you", output:vec!["bbb", "ccc"]},
            Devices{input: "bbb", output:vec!["ddd", "eee"]},
            Devices{input: "ccc", output:vec!["ddd", "eee", "fff"]},
            Devices{input: "ddd", output:vec!["ggg"]},
            Devices{input: "eee", output: vec!["out"]}];
        let devices = receive_input(input);
        for iter in 0..all_devices.len() {
            assert_eq!(devices, all_devices, "devices aren't placed correct")
        }
    }

    #[test]
    fn test_path() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        //path: you->bbb->ddd->ggg->out
        //              ->eee->out
        //      ccc->ddd->ggg->out
        //      ccc->eee->out
        //      ccc->fff->out
        let sum_path = all_paths(all_devices, None, None);
        
        assert_eq!(sum_path, 5);
    }
}