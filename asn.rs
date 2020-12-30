
mod asn {
    #[derive(Debug)]
    pub struct AsnData<'a>(pub u8, pub &'a [u8]);

    pub fn data_split(mut data: &[u8]) -> Option<Vec<AsnData>> {
        let mut res = vec![];
        while let Some(id) = data.get(0) {
            let (packet, rest) = prefixed(&data[1..])?;
            res.push(AsnData(*id, packet));
            data = rest;
        }
        Some(res)
    }

    pub fn prefixed(data: &[u8]) -> Option<(&[u8], &[u8])> {
        //One byte 0..127, many bytes = N & 0x7F
        data.split_first().and_then(|(one, rest)| {
            let one = *one as usize;
            let (len, res) = if one < 128 {
                (0, one)
            } else {
                let len = one & 0x7f;
                if len > 4 || len > rest.len() {
                    return None;
                }
                let mut res = 0;
                for x in 0..len {
                    res = res << 8 | rest[x] as u32;
                }
                (len, res as usize)
            };
            let tot = len + res;
            let next = rest.get(tot..)?;
            let this = rest.get(len..tot)?;
            Some((this, next))
        })
    }
}
