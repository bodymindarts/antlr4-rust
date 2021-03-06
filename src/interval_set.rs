pub struct Interval {
    pub start: i32,
    pub stop: i32,
}

impl Interval {
    // stop is not included!
    pub fn new(start: i32, stop: i32) -> Interval {
        Interval {
            start: start,
            stop: stop,
        }
    }

    // pub fn contains(item: i32) -> bool {
    //  return item >= i.start && item < i.stop
    //
    //
    // pub fn String() -> &str {
    //  if i.start == i.stop-1 {
    //    return strconv.Itoa(i.start)
    //  }
    //
    //  return strconv.Itoa(i.start) + ".." + strconv.Itoa(i.stop-1)
    //
    //
    // pub fn length() -> i32 {
    //  return i.stop - i.start
    //
    //
}
// pub struct IntervalSet {
//  intervals []*Interval
//  readOnly:  bool
//
//
// impl IntervalSet {§//  pub fn new() -> *IntervalSet {
//
//  i := new(IntervalSet)
//
//  i.intervals = nil
//  i.readOnly = false
//
//  return i
//
//
// pub fn first() -> i32 {
//  if len(i.intervals) == 0 {
//    return TokenInvalidType
//  }
//
//  return i.intervals[0].start
//
//
// pub fn addOne(v: i32) {
//  i.addInterval(NewInterval(v, v+1))
//
//
// pub fn addRange(l, h: i32) {
//  i.addInterval(NewInterval(l, h+1))
//
//
// pub fn addInterval(v *Interval) {
//  if i.intervals == nil {
//    i.intervals = make([]*Interval, 0)
//    i.intervals = append(i.intervals, v)
//  } else {
//    // find insert pos
//    for k, i32erval := range i.intervals {
//      // distinct range -> insert
//      if v.stop < i32erval.start {
//        i.intervals = append(i.intervals[0:k], append([]*Interval{v}, i.intervals[k:]...)...)
//        return
//      } else if v.stop == i32erval.start {
//        i.intervals[k].start = v.start
//        return
//      } else if v.start <= i32erval.stop {
//        i.intervals[k] = NewInterval(intMin(interval.start, v.start), i32Max(interval.stop, v.stop))
//
//        // if not applying to end, merge potential overlaps
//        if k < len(i.intervals)-1 {
//          l := i.intervals[k]
//          r := i.intervals[k+1]
//          // if r contained in l
//          if l.stop >= r.stop {
//            i.intervals = append(i.intervals[0:k+1], i.intervals[k+2:]...)
//          } else if l.stop >= r.start { // partial overlap
//            i.intervals[k] = NewInterval(l.start, r.stop)
//            i.intervals = append(i.intervals[0:k+1], i.intervals[k+2:]...)
//          }
//        }
//        return
//      }
//    }
//    // greater than any exiting
//    i.intervals = append(i.intervals, v)
//  }
//
//
// pub fn addSet(other *IntervalSet) -> *IntervalSet {
//  if other.intervals != nil {
//    for k := 0; k < len(other.intervals); k++ {
//      i2 := other.intervals[k]
//      i.addInterval(NewInterval(i2.start, i2.stop))
//    }
//  }
//  return i
//
//
// pub fn complement(start: i32, stop: i32) -> *IntervalSet {
//  result := NewIntervalSet()
//  result.addInterval(NewInterval(start, stop+1))
//  for j := 0; j < len(i.intervals); j++ {
//    result.removeRange(i.intervals[j])
//  }
//  return result
//
//
// pub fn contains(item: i32) -> bool {
//  if i.intervals == nil {
//    return false
//  }
//  for k := 0; k < len(i.intervals); k++ {
//    if i.intervals[k].contains(item) {
//      return true
//    }
//  }
//  return false
//
//
// pub fn length() -> i32 {
//  len := 0
//
//  for _, v := range i.intervals {
//    len += v.length()
//  }
//
//  return len
//
//
// pub fn removeRange(v *Interval) {
//  if v.start == v.stop-1 {
//    i.removeOne(v.start)
//  } else if i.intervals != nil {
//    k := 0
//    for n := 0; n < len(i.intervals); n++ {
//      ni := i.intervals[k]
//      // i32ervals are ordered
//      if v.stop <= ni.start {
//        return
//      } else if v.start > ni.start && v.stop < ni.stop {
//        i.intervals[k] = NewInterval(ni.start, v.start)
//        x := NewInterval(v.stop, ni.stop)
//        // i.intervals.splice(k, 0, x)
//        i.intervals = append(i.intervals[0:k], append([]*Interval{x}, i.intervals[k:]...)...)
//        return
//      } else if v.start <= ni.start && v.stop >= ni.stop {
//        //                i.intervals.splice(k, 1)
//        i.intervals = append(i.intervals[0:k], i.intervals[k+1:]...)
//        k = k - 1 // need another pass
//      } else if v.start < ni.stop {
//        i.intervals[k] = NewInterval(ni.start, v.start)
//      } else if v.stop < ni.stop {
//        i.intervals[k] = NewInterval(v.stop, ni.stop)
//      }
//      k++
//    }
//  }
//
//
// pub fn removeOne(v: i32) {
//  if i.intervals != nil {
//    for k := 0; k < len(i.intervals); k++ {
//      ki := i.intervals[k]
//      // i32ervals i ordered
//      if v < ki.start {
//        return
//      } else if v == ki.start && v == ki.stop-1 {
//        //        i.intervals.splice(k, 1)
//        i.intervals = append(i.intervals[0:k], i.intervals[k+1:]...)
//        return
//      } else if v == ki.start {
//        i.intervals[k] = NewInterval(ki.start+1, ki.stop)
//        return
//      } else if v == ki.stop-1 {
//        i.intervals[k] = NewInterval(ki.start, ki.stop-1)
//        return
//      } else if v < ki.stop-1 {
//        x := NewInterval(ki.start, v)
//        ki.start = v + 1
//        //        i.intervals.splice(k, 0, x)
//        i.intervals = append(i.intervals[0:k], append([]*Interval{x}, i.intervals[k:]...)...)
//        return
//      }
//    }
//  }
//
//
// pub fn String() -> &str {
//  return i.StringVerbose(nil, nil, false)
//
//
// pub fn StringVerbose(literalNames []string, symbolicNames []string, elemsAreChar: bool) -> &str {
//
//  if i.intervals == nil {
//    return "{}"
//  } else if literalNames != nil || symbolicNames != nil {
//    return i.toTokenString(literalNames, symbolicNames)
//  } else if elemsAreChar {
//    return i.toCharString()
//  }
//
//  return i.toIndexString()
//
//
// pub fn toCharString() -> &str {
//  names := make([]string, len(i.intervals))
//
//  for j := 0; j < len(i.intervals); j++ {
//    v := i.intervals[j]
//    if v.stop == v.start+1 {
//      if v.start == TokenEOF {
//        names = append(names, "<EOF>")
//      } else {
//        names = append(names, ("'" + &str(v.start) + "'"))
//      }
//    } else {
//      names = append(names, "'"+string(v.start)+"'..'"+string(v.stop-1)+"'")
//    }
//  }
//  if len(names) > 1 {
//    return "{" + &strs.Join(names, ", ") + "}"
//  }
//
//  return names[0]
//
//
// pub fn toIndexString() -> &str {
//
//  names := make([]string, 0)
//  for j := 0; j < len(i.intervals); j++ {
//    v := i.intervals[j]
//    if v.stop == v.start+1 {
//      if v.start == TokenEOF {
//        names = append(names, "<EOF>")
//      } else {
//        names = append(names, strconv.Itoa(v.start))
//      }
//    } else {
//      names = append(names, strconv.Itoa(v.start)+".."+strconv.Itoa(v.stop-1))
//    }
//  }
//  if len(names) > 1 {
//    return "{" + &strs.Join(names, ", ") + "}"
//  }
//
//  return names[0]
//
//
// pub fn toTokenString(literalNames []string, symbolicNames []string) -> &str {
//  names := make([]string, 0)
//  for _, v := range i.intervals {
//    for j := v.start; j < v.stop; j++ {
//      names = append(names, i.elementName(literalNames, symbolicNames, j))
//    }
//  }
//  if len(names) > 1 {
//    return "{" + &strs.Join(names, ", ") + "}"
//  }
//
//  return names[0]
//
//
// pub fn elementName(literalNames []string, symbolicNames []string, a: i32) -> &str {
//  if a == TokenEOF {
//    return "<EOF>"
//  } else if a == TokenEpsilon {
//    return "<EPSILON>"
//  } else {
//    if a < len(literalNames) && literalNames[a] != "" {
//      return literalNames[a]
//    }
//
//    return symbolicNames[a]
//  }
//
