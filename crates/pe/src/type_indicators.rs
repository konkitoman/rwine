#[derive(Debug)]
pub enum TypeIndicators {
    AMD64(TypeIndicatorsAMD64),
    I386(TypeIndicatorsI386),
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct TypeIndicatorsAMD64: u16{
        ///The relocation is ignored.
        const IMAGE_REL_AMD64_ABSOLUTE = 0x0000;

        ///The 64-bit VA of the relocation target.
        const IMAGE_REL_AMD64_ADDR64 = 0x0001;

        ///The 32-bit VA of the relocation target.
        const IMAGE_REL_AMD64_ADDR32 = 0x0002;

        ///The 32-bit address without an image base (RVA).
        const IMAGE_REL_AMD64_ADDR32NB = 0x0003;

        ///The 32-bit relative address from the byte following the relocation.
        const IMAGE_REL_AMD64_REL32 = 0x0004;

        ///The 32-bit address relative to byte distance 1 from the relocation.
        const IMAGE_REL_AMD64_REL32_1 = 0x0005;

        ///The 32-bit address relative to byte distance 2 from the relocation.
        const IMAGE_REL_AMD64_REL32_2 = 0x0006;

        ///The 32-bit address relative to byte distance 3 from the relocation.
        const IMAGE_REL_AMD64_REL32_3 = 0x0007;

        ///The 32-bit address relative to byte distance 4 from the relocation.
        const IMAGE_REL_AMD64_REL32_4 = 0x0008;

        ///The 32-bit address relative to byte distance 5 from the relocation.
        const IMAGE_REL_AMD64_REL32_5 = 0x0009;

        ///The 16-bit section index of the section that contains the target. This is used to support debugging information.
        const IMAGE_REL_AMD64_SECTION = 0x000A;

        ///The 32-bit offset of the target from the beginning of its section. This is used to support debugging information and static thread local storage.
        const IMAGE_REL_AMD64_SECREL = 0x000B;

        ///A 7-bit unsigned offset from the base of the section that contains the target.
        const IMAGE_REL_AMD64_SECREL7 = 0x000C;

        ///CLR tokens.
        const IMAGE_REL_AMD64_TOKEN = 0x000D;

        ///A 32-bit signed span-dependent value emitted into the object.
        const IMAGE_REL_AMD64_SREL32 = 0x000E;

        ///A pair that must immediately follow every span-dependent value.
        const IMAGE_REL_AMD64_PAIR = 0x000F;

        ///A 32-bit signed span-dependent value that is applied at link time.
        const IMAGE_REL_AMD64_SSPAN32 = 0x0010;
    }
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct TypeIndicatorsI386: u16{
        ///The relocation is ignored.
        const IMAGE_REL_I386_ABSOLUTE = 0x0000;

        ///Not supported.
        const IMAGE_REL_I386_DIR16 = 0x0001;

        ///Not supported.
        const IMAGE_REL_I386_REL16 = 0x0002;

        ///The target's 32-bit VA.
        const IMAGE_REL_I386_DIR32 = 0x0006;

        ///The target's 32-bit RVA.
        const IMAGE_REL_I386_DIR32NB = 0x0007;

        ///Not supported.
        const IMAGE_REL_I386_SEG12 = 0x0009;

        ///The 16-bit section index of the section that contains the target. This is used to support debugging information.
        const IMAGE_REL_I386_SECTION = 0x000A;

        ///The 32-bit offset of the target from the beginning of its section. This is used to support debugging information and static thread local storage.
        const IMAGE_REL_I386_SECREL = 0x000B;

        ///The CLR token.
        const IMAGE_REL_I386_TOKEN = 0x000C;

        ///A 7-bit offset from the base of the section that contains the target.
        const IMAGE_REL_I386_SECREL7 = 0x000D;

        ///The 32-bit relative displacement to the target. This supports the x86 relative branch and call instructions.
        const IMAGE_REL_I386_REL32 = 0x0014;
    }
}
