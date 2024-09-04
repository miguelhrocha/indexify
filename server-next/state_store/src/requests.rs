use data_model::{ComputeGraph, Task};

pub enum RequestType {
    CreateNameSpace(NamespaceRequest),
    CreateComputeGraph(CreateComputeGraphRequest),
    DeleteComputeGraph(DeleteComputeGraphRequest),
    CreateTasks(CreateTaskRequest),
}

pub struct NamespaceRequest {
    pub name: String,
}

pub struct CreateComputeGraphRequest {
    pub namespace: String,
    pub compute_graph: ComputeGraph,
}

pub struct DeleteComputeGraphRequest {
    pub namespace: String,
    pub name: String,
}

pub struct CreateTaskRequest {
    pub tasks: Vec<Task>,
}
