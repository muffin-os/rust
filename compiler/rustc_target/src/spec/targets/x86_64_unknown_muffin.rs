use crate::spec::{
    Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, StackProbeType, Target, TargetMetadata,
    TargetOptions, cvs,
};

pub(crate) fn target() -> Target {
    let opts = TargetOptions {
        cpu: "x86-64".into(),
        os: "muffin".into(),

        families: cvs!["unix"],
        has_thread_local: true,
        plt_by_default: false,
        max_atomic_width: Some(64),
        stack_probes: StackProbeType::Inline,
        relocation_model: RelocModel::Static,
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        linker: Some("rust-lld".into()),
        panic_strategy: PanicStrategy::Abort,
        features: "-avx,-avx2".into(),
        disable_redzone: true,
        ..Default::default()
    };
    Target {
        llvm_target: "x86_64-unknown-muffin".into(),
        metadata: TargetMetadata {
            description: Some("MuffinOS x86_64".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout:
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: "x86_64".into(),
        options: opts,
    }
}
