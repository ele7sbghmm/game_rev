big_endian 

set hkx 16
goto $hkx

section -collapsed "header" {
  hex 8 "magic"
  uint32 "tag"
  uint32 "version"
  uint8 "pointer size"
  uint8 "endian"
  uint8 "padding option"
  uint8 "bass class"
  uint32 "section count"
  uint32 "contents section index"
  uint32 "contents section offset"
  uint32 "contents classname section index"
  uint32 "contents classname section offset"
  ascii 16 "Havok version"
  int32 "flags"
  int16 "unknown"
  int16 "section offset"
}
section -collapsed "classnames" {
  sectionname [ascii 20]

  set classnamesOffset [expr $hkx + [uint32]]
  entry "absoulte offset" [format "0x%x" $classnamesOffset] 4 [expr [pos] - 4]
  uint32 "relative offset 1" -hex
  uint32 "relative offset 2" -hex
  uint32 "relative offset 3" -hex
  uint32 "relative offset 4" -hex
  uint32 "relative offset 5" -hex
  uint32 "relative offset 6" -hex
}
section "data" {
  sectionname [ascii 20]

  set dataSection [expr $hkx + [uint32]]
  entry "beginning of data section" [format "0x%x" $dataSection] 4 [expr [pos] - 4]

  set dataPointer [expr $dataSection + [uint32]]
  entry "data pointer" [format "0x%x" $dataPointer] 4 [expr [pos] - 4]
  set linkedEntries [expr $dataSection + [uint32]]
  entry "linked entries" [format "0x%x" $linkedEntries] 4 [expr [pos] - 4]
  set classMapping [expr $dataSection + [uint32]]
  entry "class mapping" [format "0x%x" $classMapping] 4 [expr [pos] - 4]

  set eof [expr $dataSection + [uint32]]
  uint32
  uint32
  entry "eof" [format "0x%x" $eof] 4 [expr [pos] - 4]
  entry "\"" [format "0x%x" $eof] 4 [expr [pos] - 4]
  entry "\"" [format "0x%x" $eof] 4 [expr [pos] - 4]

  set dataPointerCount [expr [expr $linkedEntries - $dataPointer] / 4]
  set linkedEntriesCount [expr [expr $classMapping - $linkedEntries] / 12]
  set classMappingCount [expr [expr $eof - $classMapping] / 12]
}
section -collapsed "types" {
  sectionname [ascii 20]

  uint32 "absolute offset" -hex
}
section -collapsed "class names" {
  goto $classnamesOffset
  set n 0
  while {[pos] < [len]} {
    set n [expr $n + 1]
    set id [uint32]
    set nine [uint8]
    if {$nine != 9} { break }
    # cstr "ascii" [format "%x %x" [expr [pos] - 16] $id]
    cstr "ascii" [format "%x" [expr [pos] - 16]]
  }
  sectionname [format "%d class names" $n]
}

goto $dataPointer
set n $dataPointerCount
section "$n data pointers" {
  for {set i 0} {$i < $n} {incr i} {
    set a [uint32]
    set aO [expr $a + $dataSection]
    entry [format "= %x" $a] [format "*%x" $aO] 4 $aO
  }
}

set n $linkedEntriesCount
section -collapsed "$n linked entries" {
  for {set i 0} {$i < $n} {incr i} {
    section [format "%x" [expr $i + 1]] {
      set a [int32]
      set aO [expr $a + $dataSection]
      set bP [pos]
      set b [int32]
      set c [int32]
      set cO [expr $c + $dataSection]
      if {$a != -1} {
        entry [format "%x" $a] [format "%x" $aO] 4 $aO
        entry [format "%x" $b] [format "@ %x" $bP] 4 $bP
        entry [format "%x" $c] [format "%x" $cO] 4 $cO
      }
    }
  }
}

goto $classMapping

set n $classMappingCount
section -collapsed "$n class mapping" {
  for {set i 0} {$i < $n} {incr i} {
    section [format "%x" [expr $i + 1]] {
      set a [int32]
      set aO [expr $a + $dataSection]
      entry [format "0x%x" $a] [format "0x%x" $aO] 4 $aO
      set b [int32]
      entry [format "0x%x" $b] [format "0x%x" [expr [pos] - 4]] 4 [expr [pos] - 4]
      set c [int32]
      set cO [expr $classnamesOffset + $c]
      set old [pos]
      goto $cO
      cstr "ascii" [format "0x%x" "$c"]
      goto $old
    }
  }
}
