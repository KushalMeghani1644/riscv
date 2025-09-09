# Improve documentation and code quality for RISC-V assembly instructions

## Summary

- This PR enhances the documentation and code quality of RISC-V assembly instruction wrappers in asm.rs while maintaining full backward compatibility.

## Changes Made: 

### Documentation Improvements: 

- Enhanced safety documentation: Added comprehensive safety sections explaining when and why instructions like ebreak and ecall are unsafe, including specific warnings about exception handling and stack pointer management.

- Expanded behavioral descriptions: Provided detailed explanations of instruction behavior, including privilege mode effects for ecall and power management implications for wfi.

- Added practical use cases: Documented appropriate use cases for fence operations, including multiprocessor considerations and performance implications.

- Improved function parameter documentation: Enhanced documentation for sfence_vma() with clear parameter descriptions and usage examples.

## Code Quality Enhancements: 

- Removed redundant code: Eliminated unnecessary unimplemented!() calls on non-RISC-V targets since functions are already properly gated with cfg attributes.

- Added missing assembly options: Included preserves_flags option for instructions that do not modify processor flags (nop, wfi, ebreak, ecall).

- Fixed assembly template syntax: Updated sfence_vma() to use idiomatic {} placeholder syntax instead of explicit numbering.

- Improved code organization: Restructured the delay() function implementation for better readability.

### Enhanced Warning Documentation: 

- Strengthened timing accuracy warnings: Added comprehensive warnings about the delay() function's limitations, including interrupt interference and architecture dependencies, with strong recommendations to use proper timer peripherals for precise timing requirements.

- Added performance guidance: Documented the performance implications of fence operations and recommended using specific sfence_vma() over sfence_vma_all() when possible.

### Testing: 

- All existing functionality remains unchanged. The modifications only affect documentation and non-functional code improvements.

### Compatibility

- This PR maintains full backward compatibility. All existing APIs and behaviors are preserved.
