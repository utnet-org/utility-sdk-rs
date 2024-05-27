use unc_sdk::UncSchema;

// https://stackoverflow.com/a/71721454/9806233
// https://github.com/nvzqz/impls/blob/e616c2d65615aa04cd266dd9f7bcab14e2a10d50/src/lib.rs#L647-L661
macro_rules! impls {
    ($ty:ty: $trait:path) => {{
        trait DoesNotImpl {
            const IMPLS: bool = false;
        }
        impl<T: ?Sized> DoesNotImpl for T {}

        struct Wrapper<T: ?Sized>(std::marker::PhantomData<T>);

        #[allow(dead_code)]
        impl<T: ?Sized + $trait> Wrapper<T> {
            const IMPLS: bool = true;
        }

        <Wrapper<$ty>>::IMPLS
    }};
}

macro_rules! const_assert_impls {
    ($ty:ty: $trait:path) => {
        const _: () = {
            assert!(
        impls!($ty: $trait),
                concat!("`", stringify!($ty), "` does not implement `", stringify!($trait), "`")
            )
        };
    };
    ($ty:ty: !$trait:path) => {
        const _: () = {
            assert!(
        !impls!($ty: $trait),
                concat!(
                    "`",
                    stringify!($ty),
                    "` implements `",
                    stringify!($trait),
                    "` but shouldn't"
                )
            )
        };
    };
}

pub fn non_mod_scoped() {
    #[derive(UncSchema)]
    struct InnerValue;

    const_assert_impls!(InnerValue: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(InnerValue: !unc_sdk::borsh::BorshSchema);

    #[derive(UncSchema)]
    struct Value {
        field: InnerValue,
    }

    const_assert_impls!(Value: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(Value: !unc_sdk::borsh::BorshSchema);
}

pub fn no_schema_spec() {
    #[derive(UncSchema)]
    #[serde(rename = "UnitNoSchemaSpecSTRUCT")]
    struct UnitStructNoSchemaSpec;

    const_assert_impls!(UnitStructNoSchemaSpec: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(UnitStructNoSchemaSpec: !unc_sdk::borsh::BorshSchema);

    #[derive(UncSchema)]
    #[serde(rename = "UNITNoSchemaSpecENUM")]
    pub enum UnitEnumNoSchemaSpec {}

    const_assert_impls!(UnitEnumNoSchemaSpec: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(UnitEnumNoSchemaSpec: !unc_sdk::borsh::BorshSchema);

    #[derive(UncSchema)]
    #[serde(rename = "NoSchemaSpecENUM")]
    pub enum EnumNoSchemaSpec {
        NoAttrs,
        #[serde(rename = "serde_via_schemars")]
        Serde,
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        Nested {
            #[serde(alias = "inner_inner_hehe")]
            nested: UnitEnumNoSchemaSpec,
        },
    }

    const_assert_impls!(EnumNoSchemaSpec: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(EnumNoSchemaSpec: !unc_sdk::borsh::BorshSchema);

    #[derive(UncSchema)]
    #[serde(rename = "NoSchemaSpecSTRUCT")]
    struct StructNoSchemaSpec {
        var1: EnumNoSchemaSpec,
        var2: EnumNoSchemaSpec,
    }

    const_assert_impls!(StructNoSchemaSpec: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(StructNoSchemaSpec: !unc_sdk::borsh::BorshSchema);
}

pub fn json_schema_spec() {
    #[derive(UncSchema)]
    #[abi(json)]
    #[serde(rename = "UnitNoSchemaSpecSTRUCT")]
    pub struct UnitStructNoSchemaSpec;

    const_assert_impls!(UnitStructNoSchemaSpec: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(UnitStructNoSchemaSpec: !unc_sdk::borsh::BorshSchema);

    #[derive(UncSchema)]
    #[abi(json)]
    #[serde(rename = "UNITNoSchemaSpecENUM")]
    pub enum UnitEnumNoSchemaSpec {}

    const_assert_impls!(UnitEnumNoSchemaSpec: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(UnitEnumNoSchemaSpec: !unc_sdk::borsh::BorshSchema);

    #[derive(UncSchema)]
    #[abi(json)]
    #[serde(rename = "NoSchemaSpecENUM")]
    pub enum EnumNoSchemaSpec {
        NoAttrs,
        #[serde(rename = "serde_via_schemars")]
        Serde,
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        Nested {
            #[serde(alias = "inner_inner_hehe")]
            nested: UnitEnumNoSchemaSpec,
        },
    }

    const_assert_impls!(EnumNoSchemaSpec: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(EnumNoSchemaSpec: !unc_sdk::borsh::BorshSchema);

    #[derive(UncSchema)]
    #[abi(json)]
    #[serde(rename = "NoSchemaSpecSTRUCT")]
    struct StructNoSchemaSpec {
        var1: EnumNoSchemaSpec,
        var2: EnumNoSchemaSpec,
    }

    const_assert_impls!(StructNoSchemaSpec: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(StructNoSchemaSpec: !unc_sdk::borsh::BorshSchema);
}

pub fn borsh_schema_spec() {
    #[derive(UncSchema)]
    #[abi(borsh)]
    pub struct UnitStructNoSchemaSpec;

    const_assert_impls!(UnitStructNoSchemaSpec: unc_sdk::borsh::BorshSchema);
    const_assert_impls!(UnitStructNoSchemaSpec: !unc_sdk::schemars::JsonSchema);

    #[derive(UncSchema)]
    #[abi(borsh)]
    pub enum UnitEnumNoSchemaSpec {}

    const_assert_impls!(UnitEnumNoSchemaSpec: unc_sdk::borsh::BorshSchema);
    const_assert_impls!(UnitEnumNoSchemaSpec: !unc_sdk::schemars::JsonSchema);

    #[derive(UncSchema)]
    #[abi(borsh)]
    pub enum EnumNoSchemaSpec {
        NoAttrs,
        #[borsh(skip)]
        BorshSkip,
        Nested {
            #[borsh(skip)]
            nested: UnitEnumNoSchemaSpec,
        },
    }

    const_assert_impls!(EnumNoSchemaSpec: unc_sdk::borsh::BorshSchema);
    const_assert_impls!(EnumNoSchemaSpec: !unc_sdk::schemars::JsonSchema);

    #[derive(UncSchema)]
    #[abi(borsh)]
    struct StructNoSchemaSpec {
        var1: EnumNoSchemaSpec,
        #[borsh(skip)]
        var2: EnumNoSchemaSpec,
    }

    const_assert_impls!(StructNoSchemaSpec: unc_sdk::borsh::BorshSchema);
    const_assert_impls!(StructNoSchemaSpec: !unc_sdk::schemars::JsonSchema);
}

pub fn json_borsh_schema_spec() {
    #[derive(UncSchema)]
    #[abi(json, borsh)]
    #[serde(rename = "UnitNoSchemaSpecSTRUCT")]
    pub struct UnitStructNoSchemaSpec;

    const_assert_impls!(UnitStructNoSchemaSpec: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(UnitStructNoSchemaSpec: unc_sdk::borsh::BorshSchema);

    #[derive(UncSchema)]
    #[abi(json, borsh)]
    #[serde(rename = "UNITNoSchemaSpecENUM")]
    pub enum UnitEnumNoSchemaSpec {}

    const_assert_impls!(UnitEnumNoSchemaSpec: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(UnitEnumNoSchemaSpec: unc_sdk::borsh::BorshSchema);

    #[derive(UncSchema)]
    #[abi(json, borsh)]
    #[serde(rename = "NoSchemaSpecENUM")]
    pub enum EnumNoSchemaSpec {
        NoAttrs,
        #[borsh(skip)]
        BorshSkip,
        #[serde(rename = "serde_via_schemars")]
        Serde,
        #[borsh(skip)]
        #[serde(skip)]
        BorshSerde,
        #[serde(skip)]
        #[borsh(skip)]
        SerdeBorsh,
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        Nested {
            #[borsh(skip)]
            #[serde(alias = "inner_inner_hehe")]
            nested: UnitEnumNoSchemaSpec,
        },
    }

    const_assert_impls!(EnumNoSchemaSpec: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(EnumNoSchemaSpec: unc_sdk::borsh::BorshSchema);

    #[derive(UncSchema)]
    #[abi(json, borsh)]
    #[serde(rename = "NoSchemaSpecSTRUCT")]
    struct StructNoSchemaSpec {
        var1: EnumNoSchemaSpec,
        #[borsh(skip)]
        var2: EnumNoSchemaSpec,
    }

    const_assert_impls!(StructNoSchemaSpec: unc_sdk::schemars::JsonSchema);
    const_assert_impls!(StructNoSchemaSpec: unc_sdk::borsh::BorshSchema);
}

// fixme! this should fail, since A__UNC_SCHEMA_PROXY does not derive UncSchema
// fixme! hygeinic macro expansion is required to make this work
// fixme! or just explicit checks, making sure that no ident is suffixed with
// fixme! __UNC_SCHEMA_PROXY

#[allow(non_camel_case_types)]
struct A__UNC_SCHEMA_PROXY {}

#[derive(UncSchema)]
struct A(A__UNC_SCHEMA_PROXY);

fn main() {}
