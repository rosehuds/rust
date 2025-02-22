use crate::spec::{LinkerFlavor, LldFlavor, PanicStrategy, TargetOptions, TlsModel};

pub fn opts() -> TargetOptions {
    let pre_link_args = TargetOptions::link_args(
        LinkerFlavor::Ld,
        &["--build-id", "--hash-style=gnu", "--Bstatic"],
    );

    TargetOptions {
        os: "hermit".into(),
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        linker: Some("rust-lld".into()),
        executables: true,
        has_thread_local: true,
        pre_link_args,
        panic_strategy: PanicStrategy::Abort,
        position_independent_executables: true,
        static_position_independent_executables: true,
        tls_model: TlsModel::InitialExec,
        ..Default::default()
    }
}
