use vt_core::Program;

pub fn generate(_program: &Program) -> String {
    "// VisioTalk generated script — placeholder\n\nconst visioTalkRuntime = {\n  onObjectDetected: (obj) => {},\n  onDistanceThreshold: (obj) => {},\n  onAlert: (msg) => {},\n};\n\nexport default visioTalkRuntime;\n"
        .into()
}
