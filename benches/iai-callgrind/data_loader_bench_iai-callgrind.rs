  let loader = HttpDataLoader {
    client: client.clone(),
    batched: None,
    body: |_: HashMap<String, Vec<&Value>>, _: &str| async_graphql::Value::Null
   };

  let loader = loader.to_data_loader(Batch::default().delay(1));

  let request1 = reqwest::Request::new(reqwest::Method::GET, "http://example.com/1".parse().unwrap());
  let request2 = reqwest::Request::new(reqwest::Method::GET, "http://example.com/2".parse().unwrap());

  let headers_to_consider = BTreeSet::from(["Header1".to_string(), "Header2".to_string()]);
  let key1 = DataLoaderRequest::new(request1, headers_to_consider.clone());
  let key2 = DataLoaderRequest::new(request2, headers_to_consider);

  let futures1 = (0..100).map(|_| loader.load_one(key1.clone()));
  let futures2 = (0..100).map(|_| loader.load_one(key2.clone()));

  // Await the futures
  join_all(futures1.chain(futures2)).await;

  assert_eq!(
    client.request_count.load(Ordering::SeqCst),
    0,
    "Only one request should be made for the same key"
  );
  run_data_loader_benchmark(client).await;
}

library_benchmark_group!(name = data_loader; benchmarks = benchmark_data_loader);
main!(library_benchmark_groups = data_loader);
