// This file was automatically generated by build.rs
.set noreorder

.globl file_open
file_open:
    j 0xA0
    li $9, 0x00

.globl exit
exit:
    j 0xA0
    li $9, 0x06

.globl save_state
save_state:
    j 0xA0
    li $9, 0x13

.globl rand
rand:
    j 0xA0
    li $9, 0x2F

.globl srand
srand:
    j 0xA0
    li $9, 0x30

.globl malloc
malloc:
    j 0xA0
    li $9, 0x33

.globl free
free:
    j 0xA0
    li $9, 0x34

.globl calloc
calloc:
    j 0xA0
    li $9, 0x37

.globl realloc
realloc:
    j 0xA0
    li $9, 0x38

.globl init_heap
init_heap:
    j 0xA0
    li $9, 0x39

.globl system_error_exit
system_error_exit:
    j 0xA0
    li $9, 0x3A

.globl printf
printf:
    j 0xA0
    li $9, 0x3F

.globl load_exe_header
load_exe_header:
    j 0xA0
    li $9, 0x41

.globl load_exe_file
load_exe_file:
    j 0xA0
    li $9, 0x42

.globl do_execute
do_execute:
    j 0xA0
    li $9, 0x43

.globl flush_cache
flush_cache:
    j 0xA0
    li $9, 0x44

.globl gpu_send_dma
gpu_send_dma:
    j 0xA0
    li $9, 0x47

.globl gpu_gp1_command_word
gpu_gp1_command_word:
    j 0xA0
    li $9, 0x48

.globl gpu_command_word
gpu_command_word:
    j 0xA0
    li $9, 0x49

.globl gpu_command_word_params
gpu_command_word_params:
    j 0xA0
    li $9, 0x4A

.globl gpu_get_status
gpu_get_status:
    j 0xA0
    li $9, 0x4D

.globl load_and_execute
load_and_execute:
    j 0xA0
    li $9, 0x51

.globl cd_remove
cd_remove:
    j 0xA0
    li $9, 0x72

.globl warm_boot
warm_boot:
    j 0xA0
    li $9, 0xA0

.globl get_timer
get_timer:
    j 0xB0
    li $9, 0x03

.globl enable_timer_irq
enable_timer_irq:
    j 0xB0
    li $9, 0x04

.globl disable_timer_irq
disable_timer_irq:
    j 0xB0
    li $9, 0x05

.globl restart_timer
restart_timer:
    j 0xB0
    li $9, 0x06

.globl init_pad
init_pad:
    j 0xB0
    li $9, 0x12

.globl start_pad
start_pad:
    j 0xB0
    li $9, 0x13

.globl stop_pad
stop_pad:
    j 0xB0
    li $9, 0x14

.globl change_clear_pad
change_clear_pad:
    j 0xB0
    li $9, 0x5B

.globl change_clear_rcnt
change_clear_rcnt:
    j 0xC0
    li $9, 0x0A

.globl flush_std_in_out_put
flush_std_in_out_put:
    j 0xC0
    li $9, 0x13

.globl enter_critical_section
enter_critical_section:
    li $4, 0x01
    syscall 0x0

.globl exit_critical_section
exit_critical_section:
    li $4, 0x02
    syscall 0x0
