use crate::k8s;

pub fn get_pods() -> Vec<k8s::PodInfo> {
    vec![
        k8s::PodInfo {
            name: "nginx-7b8d6c5d9-x4k2m".into(),
            namespace: "default".into(),
            status: "Running".into(),
            restarts: 0,
            age: "2d".into(),
        },
        k8s::PodInfo {
            name: "redis-master-0".into(),
            namespace: "default".into(),
            status: "Running".into(),
            restarts: 1,
            age: "5d".into(),
        },
        k8s::PodInfo {
            name: "api-gateway-6f7d8c9-q8n3p".into(),
            namespace: "backend".into(),
            status: "Running".into(),
            restarts: 0,
            age: "12h".into(),
        },
        k8s::PodInfo {
            name: "worker-batch-j7k2x".into(),
            namespace: "jobs".into(),
            status: "Succeeded".into(),
            restarts: 0,
            age: "3h".into(),
        },
        k8s::PodInfo {
            name: "postgres-0".into(),
            namespace: "database".into(),
            status: "Running".into(),
            restarts: 0,
            age: "14d".into(),
        },
        k8s::PodInfo {
            name: "cronjob-cleanup-f9z1l".into(),
            namespace: "jobs".into(),
            status: "CrashLoopBackOff".into(),
            restarts: 12,
            age: "1h".into(),
        },
        k8s::PodInfo {
            name: "monitoring-agent-2v8x4".into(),
            namespace: "monitoring".into(),
            status: "Pending".into(),
            restarts: 0,
            age: "5m".into(),
        },
    ]
}

pub fn get_deployments() -> Vec<k8s::DeploymentInfo> {
    vec![
        k8s::DeploymentInfo {
            name: "nginx".into(),
            namespace: "default".into(),
            ready: "3/3".into(),
            up_to_date: 3,
            age: "2d".into(),
        },
        k8s::DeploymentInfo {
            name: "api-gateway".into(),
            namespace: "backend".into(),
            ready: "2/2".into(),
            up_to_date: 2,
            age: "12h".into(),
        },
        k8s::DeploymentInfo {
            name: "redis".into(),
            namespace: "default".into(),
            ready: "1/1".into(),
            up_to_date: 1,
            age: "5d".into(),
        },
        k8s::DeploymentInfo {
            name: "postgres".into(),
            namespace: "database".into(),
            ready: "1/1".into(),
            up_to_date: 1,
            age: "14d".into(),
        },
        k8s::DeploymentInfo {
            name: "monitoring-agent".into(),
            namespace: "monitoring".into(),
            ready: "0/1".into(),
            up_to_date: 0,
            age: "5m".into(),
        },
    ]
}
