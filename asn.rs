pub fn split(data: &[u8]) -> Option<Vec<&[u8]>> {
    let mut i = 1;
    if let Some(_) = prefixed(data, &mut i) {
        let mut res = vec![];
        while i < data.len() {
            if data[i] == 2 {
                i += 1;
                if let Some(n) = prefixed(data, &mut i) {
                    res.push(&data[i..i + n]);
                    i += n;
                    continue;
                }
            }
            return None;
        }
        return Some(res);
    }
    return None;
}

fn prefixed(data: &[u8], i: &mut usize) -> Option<usize> {
    let mut res = None;
    if *i < data.len() {
        if data[*i] <= 128 {
            let val = data[*i] as usize;
            *i += 1;
            res = Some(val);
        } else {
            let mut get = data[*i] as usize - 128;
            let end = *i + get + 1;
            if data.len() > *i + get {
                let mut sum = 0;
                let mut mul = 1;
                while get > 0 {
                    sum += data[*i + get] as usize * mul;
                    if sum > 16000 {
                        return None;
                    }
                    mul *= 256;
                    get -= 1;
                }
                *i = end;
                res = Some(sum);
            }
        }
    }
    if let Some(x) = res {
        if *i + x <= data.len() {
            return Some(x);
        }
    }
    return None;
}
