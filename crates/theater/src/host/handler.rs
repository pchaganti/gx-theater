use crate::actor::handle::ActorHandle;
use crate::config::permissions::*;
use crate::host::environment::EnvironmentHost;
use crate::host::filesystem::FileSystemHost;
use crate::host::framework::HttpFramework;
use crate::host::http_client::HttpClientHost;
use crate::host::message_server::MessageServerHost;
use crate::host::process::ProcessHost;
use crate::host::random::RandomHost;
use crate::host::runtime::RuntimeHost;
use crate::host::store::StoreHost;
use crate::host::supervisor::SupervisorHost;
use crate::host::timing::TimingHost;
use crate::shutdown::ShutdownReceiver;
use crate::wasm::{ActorComponent, ActorInstance};
use anyhow::Result;

pub enum Handler {
    MessageServer(MessageServerHost, Option<MessageServerPermissions>),
    Environment(EnvironmentHost, Option<EnvironmentPermissions>),
    FileSystem(FileSystemHost, Option<FileSystemPermissions>),
    HttpClient(HttpClientHost, Option<HttpClientPermissions>),
    HttpFramework(HttpFramework, Option<HttpFrameworkPermissions>),
    Process(ProcessHost, Option<ProcessPermissions>),
    Runtime(RuntimeHost, Option<RuntimePermissions>),
    Supervisor(SupervisorHost, Option<SupervisorPermissions>),
    Store(StoreHost, Option<StorePermissions>),
    Timing(TimingHost, Option<TimingPermissions>),
    Random(RandomHost, Option<RandomPermissions>),
}

impl Handler {
    pub async fn start(
        &mut self,
        actor_handle: ActorHandle,
        shutdown_receiver: ShutdownReceiver,
    ) -> Result<()> {
        match self {
            Handler::MessageServer(h, _) => Ok(h
                .start(actor_handle, shutdown_receiver)
                .await
                .expect("Error starting message server")),
            Handler::Environment(h, _) => Ok(h
                .start(actor_handle, shutdown_receiver)
                .await
                .expect("Error starting environment handler")),
            Handler::FileSystem(h, _) => Ok(h
                .start(actor_handle, shutdown_receiver)
                .await
                .expect("Error starting filesystem")),
            Handler::HttpClient(h, _) => Ok(h
                .start(actor_handle, shutdown_receiver)
                .await
                .expect("Error starting http client")),
            Handler::HttpFramework(h, _) => Ok(h
                .start(actor_handle, shutdown_receiver)
                .await
                .expect("Error starting http framework")),
            Handler::Process(h, _) => Ok(h
                .start(actor_handle, shutdown_receiver)
                .await
                .expect("Error starting process handler")),
            Handler::Runtime(h, _) => Ok(h
                .start(actor_handle, shutdown_receiver)
                .await
                .expect("Error starting runtime")),
            Handler::Supervisor(h, _) => Ok(h
                .start(actor_handle, shutdown_receiver)
                .await
                .expect("Error starting supervisor")),
            Handler::Store(h, _) => Ok(h
                .start(actor_handle, shutdown_receiver)
                .await
                .expect("Error starting store")),
            Handler::Timing(h, _) => Ok(h
                .start(actor_handle, shutdown_receiver)
                .await
                .expect("Error starting timing")),
            Handler::Random(h, _) => Ok(h
                .start(actor_handle, shutdown_receiver)
                .await
                .expect("Error starting random")),
        }
    }

    pub async fn setup_host_functions(
        &mut self,
        actor_component: &mut ActorComponent,
    ) -> Result<()> {
        match self {
            Handler::MessageServer(h, _) => Ok(h
                .setup_host_functions(actor_component)
                .await
                .expect("Error setting up message server host functions")),
            Handler::Environment(h, _) => Ok(h
                .setup_host_functions(actor_component)
                .await
                .expect("Error setting up environment host functions")),
            Handler::FileSystem(h, _) => Ok(h
                .setup_host_functions(actor_component)
                .await
                .expect("Error setting up filesystem host functions")),
            Handler::HttpClient(h, _) => Ok(h
                .setup_host_functions(actor_component)
                .await
                .expect("Error setting up http client host functions")),
            Handler::HttpFramework(h, _) => Ok(h
                .setup_host_functions(actor_component)
                .await
                .expect("Error setting up http framework host functions")),
            Handler::Process(h, _) => Ok(h
                .setup_host_functions(actor_component)
                .await
                .expect("Error setting up process host functions")),
            Handler::Runtime(h, _) => Ok(h
                .setup_host_functions(actor_component)
                .await
                .expect("Error setting up runtime host functions")),
            Handler::Supervisor(h, _) => Ok(h
                .setup_host_functions(actor_component)
                .await
                .expect("Error setting up supervisor host functions")),
            Handler::Store(h, _) => Ok(h
                .setup_host_functions(actor_component)
                .await
                .expect("Error setting up store host functions")),
            Handler::Timing(h, _) => Ok(h
                .setup_host_functions(actor_component)
                .await
                .expect("Error setting up timing host functions")),
            Handler::Random(h, _) => Ok(h
                .setup_host_functions(actor_component)
                .await
                .expect("Error setting up random host functions")),
        }
    }

    pub async fn add_export_functions(&self, actor_instance: &mut ActorInstance) -> Result<()> {
        match self {
            Handler::MessageServer(handler, _) => Ok(handler
                .add_export_functions(actor_instance)
                .await
                .expect("Error adding functions to message server")),
            Handler::Environment(handler, _) => Ok(handler
                .add_export_functions(actor_instance)
                .await
                .expect("Error adding functions to environment handler")),
            Handler::FileSystem(handler, _) => Ok(handler
                .add_export_functions(actor_instance)
                .await
                .expect("Error adding functions to filesystem")),
            Handler::HttpClient(handler, _) => Ok(handler
                .add_export_functions(actor_instance)
                .await
                .expect("Error adding functions to http client")),
            Handler::HttpFramework(handler, _) => Ok(handler
                .add_export_functions(actor_instance)
                .await
                .expect("Error adding functions to http framework")),
            Handler::Process(handler, _) => Ok(handler
                .add_export_functions(actor_instance)
                .await
                .expect("Error adding functions to process handler")),
            Handler::Runtime(handler, _) => Ok(handler
                .add_export_functions(actor_instance)
                .await
                .expect("Error adding functions to runtime")),
            Handler::Supervisor(handler, _) => Ok(handler
                .add_export_functions(actor_instance)
                .await
                .expect("Error adding functions to supervisor")),
            Handler::Store(handler, _) => Ok(handler
                .add_export_functions(actor_instance)
                .await
                .expect("Error adding functions to store")),
            Handler::Timing(handler, _) => Ok(handler
                .add_export_functions(actor_instance)
                .await
                .expect("Error adding functions to timing")),
            Handler::Random(handler, _) => Ok(handler
                .add_export_functions(actor_instance)
                .await
                .expect("Error adding functions to random")),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Handler::MessageServer(_, _) => "message-server",
            Handler::Environment(_, _) => "environment",
            Handler::FileSystem(_, _) => "filesystem",
            Handler::HttpClient(_, _) => "http-client",
            Handler::HttpFramework(_, _) => "http-framework",
            Handler::Process(_, _) => "process",
            Handler::Runtime(_, _) => "runtime",
            Handler::Supervisor(_, _) => "supervisor",
            Handler::Store(_, _) => "store",
            Handler::Timing(_, _) => "timing",
            Handler::Random(_, _) => "random",
        }
    }
}
