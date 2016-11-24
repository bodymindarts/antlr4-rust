use std::char::decode_utf16;
use std::result::Result;
use atn::ATN;
use atn_deserialization_options::{ATNDeserializationOptions,
                                  ATN_DESERIALIZATION_OPTIONS_DEFAULT_OPTIONS};

type UUID = &'static str;
// This is the earliest supported serialized UUID.
// stick to serialized version for now, we don't need a UUID instance
pub const BASE_SERIALIZED_UUID: UUID = "AADB8D7E-AEEF-4415-AD2B-8204D6CF042E";

// This list contains all of the currently supported UUIDs, ordered by when
// the feature first appeared in this branch.
pub const SUPPORTED_UUIDS: &'static [UUID] = &[BASE_SERIALIZED_UUID];

pub const SERIALIZED_VERSION: i32 = 3;

// This is the current serialized UUID.
pub const SERIALIZED_UUID: UUID = BASE_SERIALIZED_UUID;

pub const DEFAULT_ATN_DESERIALIZER: &'static ATNDeserializer<'static> =
    &ATNDeserializer { deserialization_options: ATN_DESERIALIZATION_OPTIONS_DEFAULT_OPTIONS };

// pub struct LoopEndStateIntPair {
//  item0 *LoopEndState
//  item1 i32
//
//
// pub struct BlockStartStateIntPair {
//  item0 BlockStartState
//  item1 i32
//
//

fn decode_u16(data: &[u16]) -> Vec<i32> {
    let mut data_vec: Vec<i32> = Vec::with_capacity(data.len());
    for code_point in decode_utf16(data.iter().cloned()) {
        match code_point {
            Ok(c) => data_vec.push(c as i32),
            Err(_) => panic!("Could not decode atn!"),
        }
    }
    data_vec
}

pub struct ATNDeserializer<'a> {
    deserialization_options: &'a ATNDeserializationOptions,
}

impl<'a> ATNDeserializer<'a> {
    pub fn new(options: &'a ATNDeserializationOptions) -> Self {
        ATNDeserializer { deserialization_options: options }
    }
    pub fn deserialize(&self, data: &[u16]) -> ATN {
        let mut deserialization = Deserialization {
            data: decode_u16(data),
            pos: 0,
            uuid: "".to_string(),
        };
        deserialization.init();
        deserialization.check_version();
        deserialization.check_UUID();
        //  let atn = self.readATN();
        //
        //  self.readStates(atn)
        //  self.readRules(atn)
        //  self.readModes(atn)
        //
        //  let sets = self.readSets(atn);
        //
        //  self.readEdges(atn, sets)
        //  self.readDecisions(atn)
        //  self.readLexerActions(atn)
        //  self.markPrecedenceDecisions(atn)
        //  self.verifyATN(atn)
        //
        //  if self.deserializationOptions.generateRuleBypassTransitions && atn.grammarType == ATNTypeParser {
        //    self.generateRuleBypassTransitions(atn)
        //    // Re-verify after modification
        //    self.verifyATN(atn)
        //  }
        //
        //  return atn
        ATN {}
    }
}

struct Deserialization {
    data: Vec<i32>,
    pos: usize,
    uuid: String,
}

impl Deserialization {
    fn init(&mut self) {
        let version = self.data[0]; // first value is the version number
        self.data = self.data.iter().map(|n| n - 2).collect();
        self.data[0] = version;
    }

    fn check_version(&mut self) {
        let version = self.read_int();

        if version != SERIALIZED_VERSION {
            panic!("Could not deserialize ATN with version {}  (expected {})",
                   version,
                   SERIALIZED_VERSION)
        }
    }

    fn read_int(&mut self) -> i32 {
        let v = self.data[self.pos];

        self.pos += 1;

        v as i32
    }
    pub fn check_UUID(&mut self) {
        let uuid = self.read_UUID();

        let mut found = false;
        if SUPPORTED_UUIDS.iter().all(|id| id != &uuid) {
            panic!("Could not deserialize ATN with UUID: {} (expected {} or a legacy UUID).",
                   uuid,
                   SERIALIZED_UUID);
        }

        self.uuid = uuid
    }
    pub fn read_UUID(&mut self) -> String {
        let mut bb: [i32; 16] = [0; 16];

        for i in (0..8).rev() {
            let integer = self.read_int();

            bb[(2 * i) + 1] = integer & 0xFF;
            bb[2 * i] = (integer >> 8) & 0xFF;
        }

        format!("{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:\
                 02X}{:02X}{:02X}{:02X}",
                bb[0],
                bb[1],
                bb[2],
                bb[3],
                bb[4],
                bb[5],
                bb[6],
                bb[7],
                bb[8],
                bb[9],
                bb[10],
                bb[11],
                bb[12],
                bb[13],
                bb[14],
                bb[15])
    }

    fn readATN(&mut self) -> ATN {
        let grammarType = self.read_int();
        let maxTokenType = self.read_int();

        // NewATN(grammarType, maxTokenType)
        ATN {}
    }
}

// pub fn &strInSlice(a: &str, list []string) -> i32 { // non-member
//  for i, let b = range list {;
//    if b == a {
//      return i
//    }
//  }
//
//  return -1
//
//
// / isFeatureSupported determines if a particular serialized representation of an
// / ATN supports a particular feature, identified by the UUID used for
// / serializing the ATN at the time the feature was first i32roduced. Feature is
// / the UUID marking the first time the feature was supported in the serialized
// / ATN. ActualUuid is the UUID of the actual serialized ATN which is currently
// / being deserialized. It returns true if actualUuid represents a serialized ATN
// / at or after the feature identified by feature was i32roduced, and otherwise
// / false.
// pub fn isFeatureSupported(feature, actualUUID: &str) -> bool {
//  let idx1 = &strInSlice(feature, SupportedUUIDs);
//
//  if idx1 < 0 {
//    return false
//  }
//
//  let idx2 = &strInSlice(actualUUID, SupportedUUIDs);
//
//  return idx2 >= idx1
//
//
// pub fn readStates(atn *ATN) {
//  let loopBackStateNumbers = make([]LoopEndStateIntPair, 0);
//  let endStateNumbers = make([]BlockStartStateIntPair, 0);
//
//  let nstates = self.read_int();
//
//  for let i = 0; i < nstates; i++ {;
//    let stype = self.read_int();
//
//    // Ignore bad types of states
//    if stype == ATNStateInvalidType {
//      atn.addState(nil)
//
//      continue
//    }
//
//    let ruleIndex = self.read_int();
//
//    if ruleIndex == 0xFFFF {
//      ruleIndex = -1
//    }
//
//    let s = self.stateFactory(stype, ruleIndex);
//
//    if stype == ATNStateLoopEnd {
//      let loopBackStateNumber = self.read_int();
//
//      loopBackStateNumbers = append(loopBackStateNumbers, LoopEndStateIntPair{s.(*LoopEndState), loopBackStateNumber})
//    } else if s2, let ok = s.(BlockStartState); ok {;
//      let endStateNumber = self.read_int();
//
//      endStateNumbers = append(endStateNumbers, BlockStartStateIntPair{s2, endStateNumber})
//    }
//
//    atn.addState(s)
//  }
//
//  // Delay the assignment of loop back and end states until we know all the state
//  // instances have been initialized
//  for let j = 0; j < len(loopBackStateNumbers); j++ {;
//    let pair = loopBackStateNumbers[j];
//
//    pair.item0.loopBackState = atn.states[pair.item1]
//  }
//
//  for let j = 0; j < len(endStateNumbers); j++ {;
//    let pair = endStateNumbers[j];
//
//    pair.item0.setEndState(atn.states[pair.item1].(*BlockEndState))
//  }
//
//  let numNonGreedyStates = self.read_int();
//
//  for let j = 0; j < numNonGreedyStates; j++ {;
//    let stateNumber = self.read_int();
//
//    atn.states[stateNumber].(DecisionState).setNonGreedy(true)
//  }
//
//  let numPrecedenceStates = self.read_int();
//
//  for let j = 0; j < numPrecedenceStates; j++ {;
//    let stateNumber = self.read_int();
//
//    atn.states[stateNumber].(*RuleStartState).isPrecedenceRule = true
//  }
//
//
// pub fn readRules(atn *ATN) {
//  let nrules = self.read_int();
//
//  if atn.grammarType == ATNTypeLexer {
//    atn.ruleToTokenType = make([]int, nrules) // TODO: initIntArray(nrules, 0)
//  }
//
//  atn.ruleToStartState = make([]*RuleStartState, nrules) // TODO: initIntArray(nrules, 0)
//
//  for let i = 0; i < nrules; i++ {;
//    let s = self.read_int();
//    let startState = atn.states[s].(*RuleStartState);
//
//    atn.ruleToStartState[i] = startState
//
//    if atn.grammarType == ATNTypeLexer {
//      let tokenType = self.read_int();
//
//      if tokenType == 0xFFFF {
//        tokenType = TokenEOF
//      }
//
//      atn.ruleToTokenType[i] = tokenType
//    }
//  }
//
//  atn.ruleToStopState = make([]*RuleStopState, nrules) //initIntArray(nrules, 0)
//
//  for let i = 0; i < len(atn.states); i++ {;
//    let state = atn.states[i];
//
//    if s2, let ok = state.(*RuleStopState); ok {;
//      atn.ruleToStopState[s2.ruleIndex] = s2
//      atn.ruleToStartState[s2.ruleIndex].stopState = s2
//    }
//  }
//
//
// pub fn readModes(atn *ATN) {
//  let nmodes = self.read_int();
//
//  for let i = 0; i < nmodes; i++ {;
//    let s = self.read_int();
//
//    atn.modeToStartState = append(atn.modeToStartState, atn.states[s].(*TokensStartState))
//  }
//
//
// pub fn readSets(atn *ATN) -> []*IntervalSet {
//  let sets = make([]*IntervalSet, 0);
//  let m = self.read_int();
//
//  for let i = 0; i < m; i++ {;
//    let iset = NewIntervalSet();
//
//    sets = append(sets, iset)
//
//    let n = self.read_int();
//    let containsEOF = self.read_int();
//
//    if containsEOF != 0 {
//      iset.addOne(-1)
//    }
//
//    for let j = 0; j < n; j++ {;
//      let i1 = self.read_int();
//      let i2 = self.read_int();
//
//      iset.addRange(i1, i2)
//    }
//  }
//
//  return sets
//
//
// pub fn readEdges(atn *ATN, sets []*IntervalSet) {
//  let nedges = self.read_int();
//
//  for let i = 0; i < nedges; i++ {;
//    var (
//      src      = self.read_int()
//      trg      = self.read_int()
//      ttype    = self.read_int()
//      arg1     = self.read_int()
//      arg2     = self.read_int()
//      arg3     = self.read_int()
//      trans    = self.edgeFactory(atn, ttype, src, trg, arg1, arg2, arg3, sets)
//      srcState = atn.states[src]
//    )
//
//    srcState.AddTransition(trans, -1)
//  }
//
//  // Edges for rule stop states can be derived, so they are not serialized
//  for let i = 0; i < len(atn.states); i++ {;
//    let state = atn.states[i];
//
//    for let j = 0; j < len(state.GetTransitions()); j++ {;
//      var t, ok = state.GetTransitions()[j].(*RuleTransition)
//
//      if !ok {
//        continue
//      }
//
//      let outermostPrecedenceReturn = -1;
//
//      if atn.ruleToStartState[t.getTarget().GetRuleIndex()].isPrecedenceRule {
//        if t.precedence == 0 {
//          outermostPrecedenceReturn = t.getTarget().GetRuleIndex()
//        }
//      }
//
//      let trans = NewEpsilonTransition(t.followState, outermostPrecedenceReturn);
//
//      atn.ruleToStopState[t.getTarget().GetRuleIndex()].AddTransition(trans, -1)
//    }
//  }
//
//  for let i = 0; i < len(atn.states); i++ {;
//    let state = atn.states[i];
//
//    if s2, let ok = state.(*BaseBlockStartState); ok {;
//      // We need to know the end state to set its start state
//      if s2.endState == nil {
//        panic("IllegalState")
//      }
//
//      // Block end states can only be associated to a single block start state
//      if s2.endState.startState != nil {
//        panic("IllegalState")
//      }
//
//      s2.endState.startState = state
//    }
//
//    if s2, let ok = state.(*PlusLoopbackState); ok {;
//      for let j = 0; j < len(s2.GetTransitions()); j++ {;
//        let target = s2.GetTransitions()[j].getTarget();
//
//        if t2, let ok = target.(*PlusBlockStartState); ok {;
//          t2.loopBackState = state
//        }
//      }
//    } else if s2, let ok = state.(*StarLoopbackState); ok {;
//      for let j = 0; j < len(s2.GetTransitions()); j++ {;
//        let target = s2.GetTransitions()[j].getTarget();
//
//        if t2, let ok = target.(*StarLoopEntryState); ok {;
//          t2.loopBackState = state
//        }
//      }
//    }
//  }
//
//
// pub fn readDecisions(atn *ATN) {
//  let ndecisions = self.read_int();
//
//  for let i = 0; i < ndecisions; i++ {;
//    let s = self.read_int();
//    let decState = atn.states[s].(DecisionState);
//
//    atn.DecisionToState = append(atn.DecisionToState, decState)
//    decState.setDecision(i)
//  }
//
//
// pub fn readLexerActions(atn *ATN) {
//  if atn.grammarType == ATNTypeLexer {
//    let count = self.read_int();
//
//    atn.lexerActions = make([]LexerAction, count) // initIntArray(count, nil)
//
//    for let i = 0; i < count; i++ {;
//      let actionType = self.read_int();
//      let data1 = self.read_int();
//
//      if data1 == 0xFFFF {
//        data1 = -1
//      }
//
//      let data2 = self.read_int();
//
//      if data2 == 0xFFFF {
//        data2 = -1
//      }
//
//      let lexerAction = self.lexerActionFactory(actionType, data1, data2);
//
//      atn.lexerActions[i] = lexerAction
//    }
//  }
//
//
// pub fn generateRuleBypassTransitions(atn *ATN) {
//  let count = len(atn.ruleToStartState);
//
//  for let i = 0; i < count; i++ {;
//    atn.ruleToTokenType[i] = atn.maxTokenType + i + 1
//  }
//
//  for let i = 0; i < count; i++ {;
//    self.generateRuleBypassTransition(atn, i)
//  }
//
//
// pub fn generateRuleBypassTransition(atn *ATN, idx: i32) {
//  let bypassStart = NewBasicBlockStartState();
//
//  bypassStart.ruleIndex = idx
//  atn.addState(bypassStart)
//
//  let bypassStop = NewBlockEndState();
//
//  bypassStop.ruleIndex = idx
//  atn.addState(bypassStop)
//
//  bypassStart.endState = bypassStop
//
//  atn.defineDecisionState(bypassStart.BaseDecisionState)
//
//  bypassStop.startState = bypassStart
//
//  var excludeTransition Transition
//  var endState ATNState
//
//  if atn.ruleToStartState[idx].isPrecedenceRule {
//    // Wrap from the beginning of the rule to the StarLoopEntryState
//    endState = nil
//
//    for let i = 0; i < len(atn.states); i++ {;
//      let state = atn.states[i];
//
//      if self.stateIsEndStateFor(state, idx) != nil {
//        endState = state
//        excludeTransition = state.(*StarLoopEntryState).loopBackState.GetTransitions()[0]
//
//        break
//      }
//    }
//
//    if excludeTransition == nil {
//      panic("Couldn't identify final state of the precedence rule prefix section.")
//    }
//  } else {
//    endState = atn.ruleToStopState[idx]
//  }
//
//  // All non-excluded transitions that currently target end state need to target
//  // blockEnd instead
//  for let i = 0; i < len(atn.states); i++ {;
//    let state = atn.states[i];
//
//    for let j = 0; j < len(state.GetTransitions()); j++ {;
//      let transition = state.GetTransitions()[j];
//
//      if transition == excludeTransition {
//        continue
//      }
//
//      if transition.getTarget() == endState {
//        transition.setTarget(bypassStop)
//      }
//    }
//  }
//
//  // All transitions leaving the rule start state need to leave blockStart instead
//  let ruleToStartState = atn.ruleToStartState[idx];
//  let count = len(ruleToStartState.GetTransitions());
//
//  for count > 0 {
//    bypassStart.AddTransition(ruleToStartState.GetTransitions()[count-1], -1)
//    ruleToStartState.SetTransitions([]Transition{ruleToStartState.GetTransitions()[len(ruleToStartState.GetTransitions())-1]})
//  }
//
//  // Link the new states
//  atn.ruleToStartState[idx].AddTransition(NewEpsilonTransition(bypassStart, -1), -1)
//  bypassStop.AddTransition(NewEpsilonTransition(endState, -1), -1)
//
//  let MatchState = NewBasicState();
//
//  atn.addState(MatchState)
//  MatchState.AddTransition(NewAtomTransition(bypassStop, atn.ruleToTokenType[idx]), -1)
//  bypassStart.AddTransition(NewEpsilonTransition(MatchState, -1), -1)
//
//
// pub fn stateIsEndStateFor(state: ATNState, idx: i32) -> ATNState {
//  if state.GetRuleIndex() != idx {
//    return nil
//  }
//
//  if _, let ok = state.(*StarLoopEntryState); !ok {;
//    return nil
//  }
//
//  let maybeLoopEndState = state.GetTransitions()[len(state.GetTransitions())-1].getTarget();
//
//  if _, let ok = maybeLoopEndState.(*LoopEndState); !ok {;
//    return nil
//  }
//
//  var _, ok = maybeLoopEndState.GetTransitions()[0].getTarget().(*RuleStopState)
//
//  if maybeLoopEndState.(*LoopEndState).epsilonOnlyTransitions && ok {
//    return state
//  }
//
//  return nil
//
//
// / markPrecedenceDecisions analyzes the StarLoopEntryState states in the
// / specified ATN to set the StarLoopEntryState.precedenceRuleDecision field to
// / the correct value.
// pub fn markPrecedenceDecisions(atn *ATN) {
//  for _, let state = range atn.states {;
//    if _, let ok = state.(*StarLoopEntryState); !ok {;
//      continue
//    }
//
//    // We analyze the ATN to determine if a ATN decision state is the
//    // decision for the closure block that determines whether a
//    // precedence rule should continue or complete.
//    if atn.ruleToStartState[state.GetRuleIndex()].isPrecedenceRule {
//      let maybeLoopEndState = state.GetTransitions()[len(state.GetTransitions())-1].getTarget();
//
//      if s3, let ok = maybeLoopEndState.(*LoopEndState); ok {;
//        var _, ok2 = maybeLoopEndState.GetTransitions()[0].getTarget().(*RuleStopState)
//
//        if s3.epsilonOnlyTransitions && ok2 {
//          state.(*StarLoopEntryState).precedenceRuleDecision = true
//        }
//      }
//    }
//  }
//
//
// pub fn verifyATN(atn *ATN) {
//  if !a.deserializationOptions.verifyATN {
//    return
//  }
//
//  // Verify assumptions
//  for let i = 0; i < len(atn.states); i++ {;
//    let state = atn.states[i];
//
//    if state == nil {
//      continue
//    }
//
//    self.checkCondition(state.GetEpsilonOnlyTransitions() || len(state.GetTransitions()) <= 1, "")
//
//    switch let s2 = state.(type) {;
//    case *PlusBlockStartState:
//      self.checkCondition(s2.loopBackState != nil, "")
//
//    case *StarLoopEntryState:
//      self.checkCondition(s2.loopBackState != nil, "")
//      self.checkCondition(len(s2.GetTransitions()) == 2, "")
//
//      switch let s2 = state.(type) {;
//      case *StarBlockStartState:
//        var _, ok2 = s2.GetTransitions()[1].getTarget().(*LoopEndState)
//
//        self.checkCondition(ok2, "")
//        self.checkCondition(!s2.nonGreedy, "")
//
//      case *LoopEndState:
//        var s3, ok2 = s2.GetTransitions()[1].getTarget().(*StarBlockStartState)
//
//        self.checkCondition(ok2, "")
//        self.checkCondition(s3.nonGreedy, "")
//
//      default:
//        panic("IllegalState")
//      }
//
//    case *StarLoopbackState:
//      self.checkCondition(len(state.GetTransitions()) == 1, "")
//
//      var _, ok2 = state.GetTransitions()[0].getTarget().(*StarLoopEntryState)
//
//      self.checkCondition(ok2, "")
//
//    case *LoopEndState:
//      self.checkCondition(s2.loopBackState != nil, "")
//
//    case *RuleStartState:
//      self.checkCondition(s2.stopState != nil, "")
//
//    case *BaseBlockStartState:
//      self.checkCondition(s2.endState != nil, "")
//
//    case *BlockEndState:
//      self.checkCondition(s2.startState != nil, "")
//
//    case DecisionState:
//      self.checkCondition(len(s2.GetTransitions()) <= 1 || s2.getDecision() >= 0, "")
//
//    default:
//      var _, ok = s2.(*RuleStopState)
//
//      self.checkCondition(len(s2.GetTransitions()) <= 1 || ok, "")
//    }
//  }
//
//
// pub fn checkCondition(condition: bool, message: &str) {
//  if !condition {
//    if message == "" {
//      message = "IllegalState"
//    }
//
//    panic(message)
//  }
//
//
// /TODO
// /pub fn readLong() -> i3264 {
// /    panic("Not implemented")
// /    var low = self.read_int32()
// /    var high = self.read_int32()
// /    return (low & 0x00000000FFFFFFFF) | (high << i3232)
// /}
//
// pub fn createByteToHex() -> []string { // non-member
//  let bth = make([]string, 256);
//
//  for let i = 0; i < 256; i++ {;
//    bth[i] = &strs.ToUpper(hex.EncodeToString([]byte{byte(i)}))
//  }
//
//  return bth
//
//
// var byteToHex = createByteToHex()
//
//
// pub fn edgeFactory(atn *ATN, typeIndex, src, trg, arg1, arg2, arg3: i32, sets []*IntervalSet) -> Transition {
//  let target = atn.states[trg];
//
//  switch typeIndex {
//  case TransitionEPSILON:
//    return NewEpsilonTransition(target, -1)
//
//  case TransitionRANGE:
//    if arg3 != 0 {
//      return NewRangeTransition(target, TokenEOF, arg2)
//    }
//
//    return NewRangeTransition(target, arg1, arg2)
//
//  case TransitionRULE:
//    return NewRuleTransition(atn.states[arg1], arg2, arg3, target)
//
//  case TransitionPREDICATE:
//    return NewPredicateTransition(target, arg1, arg2, arg3 != 0)
//
//  case TransitionPRECEDENCE:
//    return NewPrecedencePredicateTransition(target, arg1)
//
//  case TransitionATOM:
//    if arg3 != 0 {
//      return NewAtomTransition(target, TokenEOF)
//    }
//
//    return NewAtomTransition(target, arg1)
//
//  case TransitionACTION:
//    return NewActionTransition(target, arg1, arg2, arg3 != 0)
//
//  case TransitionSET:
//    return NewSetTransition(target, sets[arg1])
//
//  case TransitionNOTSET:
//    return NewNotSetTransition(target, sets[arg1])
//
//  case TransitionWILDCARD:
//    return NewWildcardTransition(target)
//  }
//
//  panic("The specified transition type is not valid.")
//
//
// pub fn stateFactory(typeIndex, ruleIndex: i32) -> ATNState {
//  var s ATNState
//
//  switch typeIndex {
//  case ATNStateInvalidType:
//    return nil
//
//  case ATNStateBasic:
//    s = NewBasicState()
//
//  case ATNStateRuleStart:
//    s = NewRuleStartState()
//
//  case ATNStateBlockStart:
//    s = NewBasicBlockStartState()
//
//  case ATNStatePlusBlockStart:
//    s = NewPlusBlockStartState()
//
//  case ATNStateStarBlockStart:
//    s = NewStarBlockStartState()
//
//  case ATNStateTokenStart:
//    s = NewTokensStartState()
//
//  case ATNStateRuleStop:
//    s = NewRuleStopState()
//
//  case ATNStateBlockEnd:
//    s = NewBlockEndState()
//
//  case ATNStateStarLoopBack:
//    s = NewStarLoopbackState()
//
//  case ATNStateStarLoopEntry:
//    s = NewStarLoopEntryState()
//
//  case ATNStatePlusLoopBack:
//    s = NewPlusLoopbackState()
//
//  case ATNStateLoopEnd:
//    s = NewLoopEndState()
//
//  default:
//    panic(fmt.Sprintf("state type %d is invalid", typeIndex))
//  }
//
//  s.SetRuleIndex(ruleIndex)
//
//  return s
//
//
// pub fn lexerActionFactory(typeIndex, data1, data2: i32) -> LexerAction {
//  switch typeIndex {
//  case LexerActionTypeChannel:
//    return NewLexerChannelAction(data1)
//
//  case LexerActionTypeCustom:
//    return NewLexerCustomAction(data1, data2)
//
//  case LexerActionTypeMode:
//    return NewLexerModeAction(data1)
//
//  case LexerActionTypeMore:
//    return LexerMoreActionINSTANCE
//
//  case LexerActionTypePopMode:
//    return LexerPopModeActionINSTANCE
//
//  case LexerActionTypePushMode:
//    return NewLexerPushModeAction(data1)
//
//  case LexerActionTypeSkip:
//    return LexerSkipActionINSTANCE
//
//  case LexerActionTypeType:
//    return NewLexerTypeAction(data1)
//
//  default:
//    panic(fmt.Sprintf("lexer action %d is invalid", typeIndex))
//  }
//
