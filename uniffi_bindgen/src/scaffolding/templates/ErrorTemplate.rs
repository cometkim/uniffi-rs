{#
// For each error declared in the UDL, we assume the caller has provided a corresponding
// rust `enum`. We provide the traits for sending it across the FFI, which will fail to
// compile if the provided struct has a different shape to the one declared in the UDL.
//
// We define a unit-struct to implement the trait to sidestep Rust's orphan rule (ADR-0006). It's
// public so other crates can refer to it via an `[External='crate'] typedef`
#}

#[doc(hidden)]
unsafe impl ::uniffi::FfiConverter<crate::UniFfiTag> for r#{{ e.name() }} {
    uniffi::ffi_converter_rust_buffer_lift_and_lower!(crate::UniFfiTag);

    {% if e.is_flat() %}

    // For "flat" error enums, we stringify the error on the Rust side and surface that
    // as the error message in the foreign language.


    fn write(obj: r#{{ e.name() }}, buf: &mut std::vec::Vec<u8>) {
        use uniffi::deps::bytes::BufMut;
        let msg = obj.to_string();
        match obj {
            {%- for variant in e.variants() %}
            r#{{ e.name() }}::r#{{ variant.name() }}{..} => {
                buf.put_i32({{ loop.index }});
                {{ Type::String.borrow()|ffi_converter }}::write(msg, buf);
            },
            {%- endfor %}
        };
    }

    {%- if ci.should_generate_error_read(e) %}
    fn try_read(buf: &mut &[u8]) -> uniffi::deps::anyhow::Result<r#{{ e.name() }}> {
        use uniffi::deps::bytes::Buf;
        uniffi::check_remaining(buf, 4)?;
        Ok(match buf.get_i32() {
            {%- for variant in e.variants() %}
            {{ loop.index }} => r#{{ e.name() }}::r#{{ variant.name() }}{ },
            {%- endfor %}
            v => uniffi::deps::anyhow::bail!("Invalid {{ e.name() }} enum value: {}", v),
        })
    }
    {%- else %}
    fn try_read(_buf: &mut &[u8]) -> uniffi::deps::anyhow::Result<r#{{ e.name() }}> {
        panic!("try_read not supported for flat errors");
    }
    {%- endif %}

    {% else %}

    // For rich structured enums, we map individual fields on the Rust side over to
    // corresponding fields on the foreign-language side.
    //
    // If a variant doesn't have fields defined in the UDL, it's currently still possible that
    // the Rust enum has fields and they're just not listed. In that case we use the `Variant{..}`
    // syntax to match the variant while ignoring its fields.

    fn write(obj: r#{{ e.name() }}, buf: &mut std::vec::Vec<u8>) {
        use uniffi::deps::bytes::BufMut;
        match obj {
            {%- for variant in e.variants() %}
            r#{{ e.name() }}::r#{{ variant.name() }}{% if variant.has_fields() %} { {% for field in variant.fields() %}r#{{ field.name() }}, {%- endfor %} }{% else %}{..}{% endif %} => {
                buf.put_i32({{ loop.index }});
                {% for field in variant.fields() -%}
                {{ field.type_().borrow()|ffi_converter }}::write(r#{{ field.name() }}, buf);
                {%- endfor %}
            },
            {%- endfor %}
        };
    }

    fn try_read(buf: &mut &[u8]) -> uniffi::deps::anyhow::Result<r#{{ e.name() }}> {
        // Note: no need to call should_generate_error_read here, since it is always true for
        // non-flat errors
        use uniffi::deps::bytes::Buf;
        uniffi::check_remaining(buf, 4)?;
        Ok(match buf.get_i32() {
            {%- for variant in e.variants() %}
            {{ loop.index }} => r#{{ e.name() }}::r#{{ variant.name() }}{% if variant.has_fields() %} {
                {% for field in variant.fields() %}
                r#{{ field.name() }}: {{ field.type_().borrow()|ffi_converter }}::try_read(buf)?,
                {%- endfor %}
            }{% endif %},
            {%- endfor %}
            v => uniffi::deps::anyhow::bail!("Invalid {{ e.name() }} enum value: {}", v),
        })
    }
    {% endif %}
}
