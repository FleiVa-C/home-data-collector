fn main (){
    tonic_build::configure()
        .build_client(false)
        .compile(&["../../shared/src/proto/timeseries_buffer.proto"], &["../../shared/src/proto"])
        .unwrap()
}

