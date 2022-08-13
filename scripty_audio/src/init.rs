use crate::load_balancer::LoadBalancer;

pub async fn init_stt() {
    let balancer = LoadBalancer::new()
        .await
        .expect("failed to initialize a STT service");
    crate::load_balancer::LOAD_BALANCER
        .set(balancer)
        .unwrap_or_else(|_| panic!("don't try to set the load balancer twice"));
}
