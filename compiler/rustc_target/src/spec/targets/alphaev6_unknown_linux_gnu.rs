use rustc_abi::Endian;

use crate::spec::{
    Arch, Cc, LinkSelfContainedDefault, LinkerFlavor, Lld, PanicStrategy, Target, TargetMetadata,
    TargetOptions, base,
};

pub(crate) fn target() -> Target {
    let mut base = base::linux_gnu::opts();
    base.cpu = "ev6".into();
    base.max_atomic_width = Some(64);
    base.position_independent_executables = false;
    base.static_position_independent_executables = false;

    Target {
        llvm_target: "alphaev6-unknown-linux-gnu".into(),
        metadata: TargetMetadata {
            description: Some("Experimental DEC Alpha Linux (EV6, GCC backend)".into()),
            tier: None,
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout: "e-m:e-i64:64-n32:64".into(),
        arch: Arch::Alpha,
        options: TargetOptions {
            endian: Endian::Little,
            mcount: "_mcount".into(),
            panic_strategy: PanicStrategy::Abort,
            disable_redzone: true,
            linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::No),
            link_self_contained: LinkSelfContainedDefault::False,
            ..base
        },
    }
}
