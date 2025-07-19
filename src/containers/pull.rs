// Required libraries: bollard, tokio
// Full tokio runtime should be enabled in Cargo.toml
use bollard::Docker;
use rhai::ImmutableString;

#[tokio::main]
pub async fn pull(s: ImmutableString) {
    let docker = Docker::connect_with_socket_defaults().unwrap();
    let container_info = docker
        .inspect_container(s.as_str(), None::<bollard::query_parameters::InspectContainerOptions>)
        .await.unwrap();
    let container_id = &container_info.id.unwrap();
    docker.start_container(container_id, None::<bollard::query_parameters::StartContainerOptions>).await.unwrap();

    println!("Container {:?} started", container_id);
}