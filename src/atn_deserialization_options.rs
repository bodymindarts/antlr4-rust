pub const ATN_DESERIALIZATION_OPTIONS_DEFAULT_OPTIONS: &'static ATNDeserializationOptions =
    &ATNDeserializationOptions {
        read_only: true,
        verify_atn: false,
        generate_rule_bypass_transitions: false,
    };

pub struct ATNDeserializationOptions {
    read_only: bool,
    verify_atn: bool,
    generate_rule_bypass_transitions: bool,
}

// impl ATNDeserializationOptions {
//     pub fn new(copyFrom: ATNDeserializationOptions) -> *ATNDeserializationOptions {
//  o := new(ATNDeserializationOptions)

//  if CopyFrom != nil {
//    o.readOnly = CopyFrom.readOnly
//    o.verifyATN = CopyFrom.verifyATN
//    o.generateRuleBypassTransitions = CopyFrom.generateRuleBypassTransitions
//  }

//  return o
//     }
