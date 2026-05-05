# LAST_APPLIED_FIX

Version: v55

Goal:
- Move `write` closer to real fd semantics.

Changed:
- Added `RuntimeWriteTarget`
- Added fd-backed write target selection
- Updated central syscall dispatcher write action
- Updated external init runtime write to use fd target
- Preserved v53f trap entry alignment
