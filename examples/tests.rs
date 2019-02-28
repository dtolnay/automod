// This example demonstrates using the automod macro to collect together a
// directory of test cases. Each source file in the 'regression' directory can
// be dedicated to testing an individual numbered issue. As files are added in
// that directory, they automatically become part of the crate without needing
// to be added explicitly to some handwritten list.
//
// To see the tests running:
//
//    cargo test --example tests

mod regression {
    automod::dir!("examples/regression");
}

fn main() {}
