# Native calls to code in another PSX image (generated _pcjmp_<ADDR>
# trampolines load the target into eax and jump here).
#
# If a thunk to a native body is installed at the target, tail-jump to it
# with the caller's stack untouched. Otherwise the target is MIPS code (not
# decompiled, or not loaded as expected): run it in the interpreter with the
# caller's arguments and return its result.

    .text
    .globl _pc_psx_dispatch
_pc_psx_dispatch:
    pushl %eax
    pushl %eax
    calll _pc_thunk_at
    addl $4, %esp
    testl %eax, %eax
    popl %eax
    jz 1f
    jmpl *%eax
1:
    leal 4(%esp), %ecx      # the caller's arguments
    pushl %ecx
    pushl %eax
    calll _pc_mips_call
    addl $8, %esp
    retl

# Entry into a game function selected to run interpreted (PE2_INTERPRET):
# eax = its PSX address; run it with the caller's arguments.
    .globl _pc_interp_entry
_pc_interp_entry:
    leal 4(%esp), %ecx
    pushl %ecx
    pushl %eax
    calll _pc_mips_call
    addl $8, %esp
    retl

# Entry into a game function under PE2_SHADOW: eax = its shadow slot; run
# the original and the native body from the same state and compare.
    .globl _pc_shadow_entry
_pc_shadow_entry:
    leal 4(%esp), %ecx
    pushl %ecx
    pushl %eax
    calll _pc_shadow_call
    addl $8, %esp
    retl
