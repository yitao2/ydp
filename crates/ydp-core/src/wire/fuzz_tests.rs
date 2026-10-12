//! wire visitor 的稳定版确定性随机测试。

use super::{Envelope, parse};

/// 固定算法与种子的伪随机源，使失败可以仅凭迭代编号重放。
struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn range(&mut self, upper: usize) -> usize {
        (self.next() as usize) % upper
    }
}

fn assert_envelope_invariants(envelope: &Envelope<'_>) {
    if envelope.id.is_some() {
        assert!(envelope.method.is_none());
        assert_ne!(envelope.result.is_some(), envelope.error.is_some());
    } else {
        assert!(envelope.method.is_some());
        assert!(envelope.result.is_none());
        assert!(envelope.error.is_none());
    }
}

#[test]
fn deterministic_random_bytes_do_not_escape_wire_contract() {
    let mut random = Random(0x8f3c_4d92_17a6_5be1);
    for iteration in 0..20_000 {
        let len = random.range(2049);
        let mut bytes = vec![0_u8; len];
        for byte in &mut bytes {
            *byte = random.next() as u8;
        }
        let max_message = random.range(2304);
        let max_depth = random.range(160);
        if let Ok(envelope) = parse(&bytes, max_message, max_depth) {
            assert_envelope_invariants(&envelope);
        }
        assert!(iteration < 20_000, "保留迭代编号以便重放");
    }
}

#[test]
fn deterministic_mutations_cover_known_envelope_shapes() {
    const CORPUS: &[&[u8]] = &[
        br#"{"id":1,"result":{}}"#,
        br#"{"id":1,"error":{"code":-1,"message":"x","data":null}}"#,
        br#"{"method":"Runtime.consoleAPICalled","params":{}}"#,
        br#"{"method":"Echo.\u0070ing","sessionId":"child","params":{}}"#,
        br#"{"id":1,"id":2,"result":{}}"#,
        br#"{"id":0,"result":{}}"#,
        br#"{"method":"x","result":{}}"#,
        br#"{"id":1,"result":null}"#,
    ];
    let mut random = Random(0x6c1e_09b4_a725_f83d);
    for iteration in 0..10_000 {
        let mut bytes = CORPUS[random.range(CORPUS.len())].to_vec();
        for _ in 0..=random.range(8) {
            match random.range(3) {
                0 if !bytes.is_empty() => {
                    let index = random.range(bytes.len());
                    bytes[index] = random.next() as u8;
                }
                1 if !bytes.is_empty() => {
                    let index = random.range(bytes.len());
                    bytes.remove(index);
                }
                _ if bytes.len() < 4096 => {
                    let index = random.range(bytes.len() + 1);
                    bytes.insert(index, random.next() as u8);
                }
                _ => {}
            }
        }
        if let Ok(envelope) = parse(&bytes, 4096, 128) {
            assert_envelope_invariants(&envelope);
        }
        assert!(iteration < 10_000, "保留迭代编号以便重放");
    }
}
