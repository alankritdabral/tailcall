use iai_callgrind::{black_box, library_benchmark, library_benchmark_group, main};
use serde_json::json;
use tailcall::benchmark::gather_path_matches;

const NUM_ITERATIONS: usize = 10; 

#[library_benchmark]
fn benchmark_batched_body() {
    for _ in 0..NUM_ITERATIONS {
        let input = json!({
            "data": [
                {"user": {"id": "1"}},
                {"user": {"id": "2"}},
                {"user": {"id": "3"}},
                {"user": [
                    {"id": "4"},
                    {"id": "5"}
                ]}
            ]
        });

        black_box(gather_path_matches(&input, &["data", "user", "id"]));
    }
}

library_benchmark_group!(name = batched_body; benchmarks = benchmark_batched_body);
main!(library_benchmark_groups = batched_body);
